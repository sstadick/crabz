#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use anyhow::{bail, Error, Result};
use env_logger::Env;
use flate2::read::MultiGzDecoder;
use flate2::write::DeflateDecoder;
use git_version::git_version;
use gzp::deflate::{Bgzf, Gzip, Mgzip, RawDeflate};
use gzp::par::compress::Compression;
use gzp::par::decompress::ParDecompressBuilder;
use gzp::{BgzfSyncReader, MgzipSyncReader};
use gzp::{ZBuilder, ZWriter};
use lazy_static::lazy_static;
use log::info;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::process::exit;
use structopt::{clap::AppSettings::ColoredHelp, StructOpt};
use strum::{EnumString, EnumVariantNames, VariantNames};

#[cfg(feature = "any_zlib")]
use flate2::write::ZlibDecoder;
#[cfg(feature = "any_zlib")]
use gzp::deflate::Zlib;

#[cfg(feature = "snappy")]
use gzp::snap::Snap;
#[cfg(feature = "snappy")]
use snap::read::FrameDecoder;

const BUFFERSIZE: usize = 64 * 1024;

macro_rules! string_set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert(String::from($x)); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

lazy_static! {
    /// Return the number of cpus as an &str
    pub static ref NUM_CPU: String = num_cpus::get().to_string();
}

pub const VERSION: &str = git_version!(
    cargo_prefix = "cargo:",
    prefix = "git:",
    // Note that on the CLI, the v* needs to be in single quotes
    // When passed here though there seems to be some magic quoting that happens.
    args = ["--always", "--dirty=-modified", "--match=v*"]
);

/// Get a bufferd input reader from stdin or a file
fn get_input(path: Option<PathBuf>) -> Result<Box<dyn Read + Send + 'static>> {
    let reader: Box<dyn Read + Send + 'static> = match path {
        Some(path) => {
            if path.as_os_str() == "-" {
                Box::new(BufReader::with_capacity(BUFFERSIZE, io::stdin()))
            } else {
                Box::new(BufReader::with_capacity(BUFFERSIZE, File::open(path)?))
            }
        }
        None => Box::new(BufReader::with_capacity(BUFFERSIZE, io::stdin())),
    };
    Ok(reader)
}

/// Get a buffered output writer from stdout or a file.
///
/// If input is_some and in_place is true and output is None, figure out the inplace name
#[allow(clippy::unnecessary_unwrap)]
fn get_output(
    path: Option<PathBuf>,
    input_file: Option<PathBuf>,
    in_place: bool,
    is_decompress: bool,
    format: Format,
) -> Result<Box<dyn Write + Send + 'static>> {
    let writer: Box<dyn Write + Send + 'static> = match path {
        Some(path) => {
            if path.as_os_str() == "-" {
                Box::new(BufWriter::with_capacity(BUFFERSIZE, io::stdout()))
            } else {
                Box::new(BufWriter::with_capacity(BUFFERSIZE, File::create(path)?))
            }
        }
        None => {
            // Create a file
            if in_place && input_file.is_some() {
                let input_file = input_file.unwrap();
                let (ext, allowed) = format.get_extension();
                if is_decompress {
                    if let Some(found_ext) = input_file.extension().map(|x| x.to_string_lossy()) {
                        if allowed.contains(&found_ext.to_string()) {
                            let input_file_str = input_file.to_string_lossy();
                            let stripped = input_file_str
                                .strip_suffix(&format!(".{}", found_ext))
                                .unwrap();
                            Box::new(BufWriter::with_capacity(
                                BUFFERSIZE,
                                File::create(stripped)?,
                            ))
                        } else {
                            bail!(
                                "Extension on {:?} does not match expected of {:?}",
                                input_file,
                                ext
                            )
                        }
                    } else {
                        bail!(
                            "No extension on {:?}, does not match expected of {:?}",
                            input_file,
                            ext
                        )
                    }
                } else {
                    let out = format!("{}.{}", input_file.to_string_lossy(), ext);
                    Box::new(BufWriter::with_capacity(BUFFERSIZE, File::create(out)?))
                }
            } else {
                Box::new(BufWriter::with_capacity(BUFFERSIZE, io::stdout()))
            }
        }
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

#[derive(EnumString, EnumVariantNames, strum::Display, Debug, Copy, Clone)]
#[strum(serialize_all = "kebab_case")]
enum Format {
    #[strum(serialize = "gzip", serialize = "gz")]
    Gzip,
    // TODO: is bgz valid?
    #[strum(serialize = "bgzf", serialize = "bgz")]
    Bgzf,
    #[strum(serialize = "mgzip", serialize = "mgz")]
    Mgzip,
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
        pin_at: Option<usize>,
    ) -> Box<dyn ZWriter>
    where
        W: Write + Send + 'static,
    {
        match self {
            Format::Gzip => ZBuilder::<Gzip, _>::new()
                .num_threads(num_threads)
                .compression_level(Compression::new(compression_level))
                .pin_threads(pin_at)
                .from_writer(writer),
            Format::Bgzf => ZBuilder::<Bgzf, _>::new()
                .num_threads(num_threads)
                .compression_level(Compression::new(compression_level))
                .pin_threads(pin_at)
                .from_writer(writer),
            Format::Mgzip => ZBuilder::<Mgzip, _>::new()
                .num_threads(num_threads)
                .compression_level(Compression::new(compression_level))
                .pin_threads(pin_at)
                .from_writer(writer),
            #[cfg(feature = "any_zlib")]
            Format::Zlib => ZBuilder::<Zlib, _>::new()
                .num_threads(num_threads)
                .compression_level(Compression::new(compression_level))
                .pin_threads(pin_at)
                .from_writer(writer),
            Format::RawDeflate => ZBuilder::<RawDeflate, _>::new()
                .num_threads(num_threads)
                .compression_level(Compression::new(compression_level))
                .pin_threads(pin_at)
                .from_writer(writer),
            #[cfg(feature = "snappy")]
            Format::Snap => ZBuilder::<Snap, _>::new()
                .num_threads(num_threads)
                .compression_level(Compression::new(compression_level))
                .pin_threads(pin_at)
                .from_writer(writer),
        }
    }

    fn get_highest_allowed_compression_level(&self) -> u32 {
        match self {
            Format::Gzip => 9,
            #[cfg(feature = "libdeflate")]
            Format::Bgzf => 12,
            #[cfg(not(feature = "libdeflate"))]
            Format::Bgzf => 9,
            #[cfg(feature = "libdeflate")]
            Format::Mgzip => 12,
            #[cfg(not(feature = "libdeflate"))]
            Format::Mgzip => 9,
            #[cfg(feature = "any_zlib")]
            Format::Zlib => 9,
            Format::RawDeflate => 9,
            // compression level is ignored
            #[cfg(feature = "snappy")]
            Format::Snap => u32::MAX,
        }
    }

    fn get_lowest_allowed_compression_level(&self) -> u32 {
        match self {
            Format::Gzip => 0,
            #[cfg(feature = "libdeflate")]
            Format::Bgzf => 1,
            #[cfg(not(feature = "libdeflate"))]
            Format::Bgzf => 0,
            #[cfg(feature = "libdeflate")]
            Format::Mgzip => 1,
            #[cfg(not(feature = "libdeflate"))]
            Format::Mgzip => 0,
            #[cfg(feature = "any_zlib")]
            Format::Zlib => 0,
            Format::RawDeflate => 0,
            // compression level is ignored
            #[cfg(feature = "snappy")]
            Format::Snap => 0,
        }
    }

    fn get_extension(&self) -> (&'static str, HashSet<String>) {
        match self {
            Format::Gzip => ("gz", string_set!["gz"]),
            Format::Bgzf => ("gz", string_set!["gz", "bgz"]),
            Format::Mgzip => ("gz", string_set!["gz", "mgz"]),
            #[cfg(feature = "any_zlib")]
            Format::Zlib => ("zz", string_set!["zz", "z", "gz"]),
            Format::RawDeflate => ("gz", string_set!["gz"]),
            Format::Snap => ("sz", string_set!["sz", "snappy"]),
        }
    }
}

/// Compress and decompress files.
#[derive(StructOpt, Debug)]
#[structopt(name = "crabz", author, global_setting(ColoredHelp), version = VERSION)]
struct Opts {
    /// Output path to write to, empty or "-" to write to stdout
    #[structopt(short, long)]
    output: Option<PathBuf>,

    /// Perform the compression / decompression in place.
    ///
    /// **NOTE** this will remove the input file at completion.
    #[structopt(short = "I", long)]
    in_place: bool,

    /// Input file to read from, empty or "-" to read from stdin
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,

    /// The format to use.
    #[structopt(short, long, default_value = "gzip", possible_values = Format::VARIANTS)]
    format: Format,

    /// Compression level
    #[structopt(short = "l", long, default_value = "6")]
    compression_level: u32,

    /// Number of compression threads to use, or if decompressing a format that allow for multi-threaded
    /// decompression, the number to use. Note that > 4 threads for decompression doesn't seem to help.
    #[structopt(short = "p", long, default_value = NUM_CPU.as_str())]
    compression_threads: usize,

    /// Flag to switch to decompressing inputs. Note: this flag may change in future releases
    #[structopt(short, long)]
    decompress: bool,

    /// Specify the physical core to pin threads at.
    ///
    /// This can provide a significant performance improvement, but has the downside of possibly conflicting
    /// with other pinned cores. If you are running multiple instances of `crabz` at once you can manually
    /// space out the pinned cores.
    ///
    /// # Example
    /// - Instance 1 has `-p 4 -P 0` set indicating that it will use 4 cores pinned at 0, 1, 2, 3
    /// - Instance 2 has `-p 4 -P 4` set indicating that it will use 4 cores pinned at 4, 5, 6, 7
    #[structopt(short = "P", long)]
    pin_at: Option<usize>,
}

fn main() -> Result<()> {
    let opts = setup();
    if opts.compression_level > opts.format.get_highest_allowed_compression_level()
        || opts.compression_level < opts.format.get_lowest_allowed_compression_level()
    {
        return Err(Error::msg("Invalid compression level"));
    }

    if opts.decompress {
        if let Err(err) = run_decompress(
            get_input(opts.file.clone())?,
            get_output(
                opts.output,
                opts.file.clone(),
                opts.in_place,
                opts.decompress,
                opts.format,
            )?,
            opts.format,
            opts.compression_threads,
            opts.pin_at,
        ) {
            if is_broken_pipe(&err) {
                exit(0)
            }
            return Err(err);
        }
    } else if let Err(err) = run_compress(
        get_input(opts.file.clone())?,
        get_output(
            opts.output,
            opts.file.clone(),
            opts.in_place,
            opts.decompress,
            opts.format,
        )?,
        opts.format,
        opts.compression_level,
        opts.compression_threads,
        opts.pin_at,
    ) {
        if is_broken_pipe(&err) {
            exit(0)
        }
        return Err(err);
    }

    // Remove input file
    if opts.in_place {
        if let Some(file) = opts.file {
            std::fs::remove_file(file)?;
        }
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
    pin_at: Option<usize>,
) -> Result<()>
where
    R: Read,
    W: Write + Send + 'static,
{
    info!(
        "Compressing ({}) with {} threads at compression level {}.",
        format.to_string(),
        num_threads,
        compression_level
    );
    let mut writer = format.create_compressor(output, num_threads, compression_level, pin_at);
    io::copy(&mut input, &mut writer)?;
    writer.finish()?;
    Ok(())
}

/// Run the compression program, returning any found errors
fn run_decompress<R, W>(
    mut input: R,
    mut output: W,
    format: Format,
    num_threads: usize,
    pin_at: Option<usize>,
) -> Result<()>
where
    R: Read + Send + 'static,
    W: Write + Send + 'static,
{
    info!(
        "Decompressing ({}) with {} threads available.",
        format.to_string(),
        num_threads
    );

    match format {
        Format::Gzip => {
            let mut reader = MultiGzDecoder::new(input);
            io::copy(&mut reader, &mut output)?;
            output.flush()?;
        }
        Format::Bgzf => {
            if num_threads == 0 {
                let mut reader = BgzfSyncReader::new(input);
                io::copy(&mut reader, &mut output)?;
                output.flush()?;
            } else {
                let mut reader = ParDecompressBuilder::<Bgzf>::new()
                    .num_threads(num_threads)
                    .unwrap()
                    .pin_threads(pin_at)
                    .from_reader(input);
                io::copy(&mut reader, &mut output)?;
                output.flush()?;
                reader.finish()?;
            };
        }
        Format::Mgzip => {
            if num_threads == 0 {
                let mut reader = MgzipSyncReader::new(input);
                io::copy(&mut reader, &mut output)?;
                output.flush()?;
            } else {
                let mut reader = ParDecompressBuilder::<Mgzip>::new()
                    .num_threads(num_threads)
                    .unwrap()
                    .pin_threads(pin_at)
                    .from_reader(input);
                io::copy(&mut reader, &mut output)?;
                output.flush()?;
                reader.finish()?;
            };
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
            output.flush()?;
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
