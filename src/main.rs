use anyhow::{Error, Result};
use env_logger::Env;
#[cfg(feature = "any_zlib")]
use flate2::write::ZlibDecoder;
use flate2::write::{DeflateDecoder, GzDecoder};
#[cfg(feature = "any_zlib")]
use gzp::deflate::Zlib;
use gzp::deflate::{Gzip, RawDeflate};
use gzp::parz::Compression;
#[cfg(feature = "snappy")]
use gzp::snap::Snap;
use gzp::{ZBuilder, ZWriter};
use lazy_static::lazy_static;
use log::info;
#[cfg(feature = "snappy")]
use snap::read::FrameDecoder;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::process::exit;
use structopt::{clap::AppSettings::ColoredHelp, StructOpt};
use strum::{EnumString, EnumVariantNames, VariantNames};

// Make the feature flags work for `any_zlib` and `snappy`

lazy_static! {
    /// Return the number of cpus as an &str
    pub static ref NUM_CPU: String = num_cpus::get().to_string();
}

pub mod built_info {
    use structopt::lazy_static::lazy_static;

    include!(concat!(env!("OUT_DIR"), "/built.rs"));

    /// Get a software version string including
    ///   - Git commit hash
    ///   - Git dirty info (whether the repo had uncommitted changes)
    ///   - Cargo package version if no git info found
    fn get_software_version() -> String {
        let prefix = if let Some(s) = GIT_COMMIT_HASH {
            format!("{}-{}", PKG_VERSION, s[0..8].to_owned())
        } else {
            // This shouldn't happen
            PKG_VERSION.to_string()
        };
        let suffix = match GIT_DIRTY {
            Some(true) => "-dirty",
            _ => "",
        };
        format!("{}{}", prefix, suffix)
    }

    lazy_static! {
        /// Version of the software with git hash
        pub static ref VERSION: String = get_software_version();
    }
}

/// Get a bufferd input reader from stdin or a file
fn get_input(path: Option<PathBuf>) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = match path {
        Some(path) => {
            if path.as_os_str() == "-" {
                Box::new(BufReader::new(io::stdin()))
            } else {
                Box::new(BufReader::new(File::open(path)?))
            }
        }
        None => Box::new(BufReader::new(io::stdin())),
    };
    Ok(reader)
}

/// Get a buffered output writer from stdout or a file
fn get_output(path: Option<PathBuf>) -> Result<Box<dyn Write + Send + 'static>> {
    let writer: Box<dyn Write + Send + 'static> = match path {
        Some(path) => {
            if path.as_os_str() == "-" {
                Box::new(BufWriter::new(io::stdout()))
            } else {
                Box::new(BufWriter::new(File::create(path)?))
            }
        }
        None => Box::new(BufWriter::new(io::stdout())),
    };
    Ok(writer)
}

/// Check if err is a broken pipe.
#[inline]
fn is_broken_pipe(err: &Error) -> bool {
    if let Some(io_err) = err.root_cause().downcast_ref::<io::Error>() {
        if io_err.kind() == io::ErrorKind::BrokenPipe {
            return true;
        }
    }
    false
}

#[derive(EnumString, EnumVariantNames, Debug, Copy, Clone)]
#[strum(serialize_all = "kebab_case")]
enum Format {
    #[strum(serialize = "gzip", serialize = "gz")]
    Gzip,
    #[cfg(feature = "any_zlib")]
    #[strum(serialize = "zlib", serialize = "zz")]
    Zlib,
    #[strum(serialize = "deflate")]
    RawDeflate,
    #[cfg(feature = "snappy")]
    #[strum(serialize = "snap", serialize = "sz")]
    Snap,
}

impl Format {
    /// Create a compressor writer matching the selected format
    fn create_compressor<W>(
        &self,
        writer: W,
        num_threads: usize,
        compression_level: u32,
    ) -> Box<dyn ZWriter>
    where
        W: Write + Send + 'static,
    {
        match self {
            Format::Gzip => ZBuilder::<Gzip, _>::new()
                .num_threads(num_threads)
                .compression_level(Compression::new(compression_level))
                .from_writer(writer),
            #[cfg(feature = "any_zlib")]
            Format::Zlib => ZBuilder::<Zlib, _>::new()
                .num_threads(num_threads)
                .compression_level(Compression::new(compression_level))
                .from_writer(writer),
            Format::RawDeflate => ZBuilder::<RawDeflate, _>::new()
                .num_threads(num_threads)
                .compression_level(Compression::new(compression_level))
                .from_writer(writer),
            #[cfg(feature = "snappy")]
            Format::Snap => ZBuilder::<Snap, _>::new()
                .num_threads(num_threads)
                .compression_level(Compression::new(compression_level))
                .from_writer(writer),
        }
    }
}

/// A small POC program to compress files like pigz.
///
/// This will use all threads possible on your system.
#[derive(StructOpt, Debug)]
#[structopt(name = "crabz", author, global_setting(ColoredHelp), version = built_info::VERSION.as_str())]
struct Opts {
    /// Output path to write to, empty or "-" to write to stdout
    #[structopt(short, long)]
    output: Option<PathBuf>,

    /// Input file to read from, empty or "-" to read from stdin
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,

    /// The format to use.
    #[structopt(short, long, default_value = "gzip", possible_values = Format::VARIANTS)]
    format: Format,

    /// Compression level
    #[structopt(short, long, default_value = "3")]
    compression_level: u32,

    /// Number of compression threads to use.
    #[structopt(short = "p", long, default_value = NUM_CPU.as_str())]
    compression_threads: usize,

    /// Flag to switch to decompressing inputs. Note: this flag may change in future releases
    #[structopt(short, long)]
    decompress: bool,
}

fn main() -> Result<()> {
    let opts = setup();
    if opts.compression_level > 9 {
        return Err(Error::msg("Invalid compression level"));
    }

    if opts.decompress {
        if let Err(err) =
            run_decompress(get_input(opts.file)?, get_output(opts.output)?, opts.format)
        {
            if is_broken_pipe(&err) {
                exit(0)
            }
            return Err(err);
        }
    } else if let Err(err) = run_compress(
        get_input(opts.file)?,
        get_output(opts.output)?,
        opts.format,
        opts.compression_level,
        opts.compression_threads,
    ) {
        if is_broken_pipe(&err) {
            exit(0)
        }
        return Err(err);
    }
    Ok(())
}

/// Run the compression program, returning any found errors
fn run_compress<R, W>(
    mut input: R,
    output: W,
    format: Format,
    compression_level: u32,
    num_threads: usize,
) -> Result<()>
where
    R: Read,
    W: Write + Send + 'static,
{
    info!(
        "Compressing with {} threads at compression level {}.",
        num_threads, compression_level
    );
    let mut writer = format.create_compressor(output, num_threads, compression_level);
    io::copy(&mut input, &mut writer)?;
    writer.finish()?;
    Ok(())
}

/// Run the compression program, returning any found errors
fn run_decompress<R, W>(mut input: R, mut output: W, format: Format) -> Result<()>
where
    R: Read,
    W: Write + Send + 'static,
{
    info!("Decompressing.");

    match format {
        Format::Gzip => {
            let mut writer = GzDecoder::new(output);
            io::copy(&mut input, &mut writer)?;
            writer.finish()?;
        }
        #[cfg(feature = "any_zlib")]
        Format::Zlib => {
            let mut writer = ZlibDecoder::new(output);
            io::copy(&mut input, &mut writer)?;
            writer.finish()?;
        }
        Format::RawDeflate => {
            let mut writer = DeflateDecoder::new(output);
            io::copy(&mut input, &mut writer)?;
            writer.finish()?;
        }
        #[cfg(feature = "snappy")]
        Format::Snap => {
            let mut reader = FrameDecoder::new(input);
            io::copy(&mut reader, &mut output)?;
        }
    }

    Ok(())
}
/// Parse args and set up logging / tracing
fn setup() -> Opts {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    Opts::from_args()
}
