use color_eyre::Report;
use gzp::deflate::Gzip;
use gzp::parz::{Compression, ParZ};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::process::exit;
use structopt::{clap::AppSettings::ColoredHelp, StructOpt};
use tracing::info;
use tracing_subscriber::EnvFilter;

/// Get a bufferd input reader from stdin or a file
fn get_input(path: Option<PathBuf>) -> Result<Box<dyn Read>, Report> {
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
fn get_output(path: Option<PathBuf>) -> Result<Box<dyn Write + Send + 'static>, Report> {
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
fn is_broken_pipe(err: &Report) -> bool {
    if let Some(io_err) = err.root_cause().downcast_ref::<io::Error>() {
        if io_err.kind() == io::ErrorKind::BrokenPipe {
            return true;
        }
    }
    false
}

/// A small POC program to compress files like pigz.
///
/// This will use all threads possible on your system.
#[derive(StructOpt, Debug)]
#[structopt(name = "crabz", author, global_setting(ColoredHelp))]
struct Opts {
    /// Output path to write to, "-" to write to stdout
    #[structopt(short, long)]
    output: Option<PathBuf>,

    /// Input file to read from, "-" to read from stdin
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,

    /// Compression level
    #[structopt(short, long, default_value = "3")]
    compression_level: u32,

    // Number of compression threads to use, uses all available if not set
    #[structopt(short = "p", long)]
    compression_threads: Option<usize>,
}

fn main() -> Result<(), Report> {
    let opts = setup()?;
    if opts.compression_level > 9 {
        return Err(Report::msg("Invalid compression level"));
    }

    if let Err(err) = run(
        get_input(opts.file)?,
        get_output(opts.output)?,
        opts.compression_level,
        opts.compression_threads.unwrap_or_else(num_cpus::get),
    ) {
        if is_broken_pipe(&err) {
            exit(0)
        }
        return Err(err);
    }
    Ok(())
}

/// Run the program, returning any found errors
fn run<R, W>(
    mut input: R,
    output: W,
    compression_level: u32,
    num_threads: usize,
) -> Result<(), Report>
where
    R: Read,
    W: Write + Send + 'static,
{
    info!(
        "Compressing with {} threads at compression level {}.",
        num_threads, compression_level
    );
    let mut parz: ParZ<Gzip> = ParZ::builder(output)
        .compression_level(Compression::new(compression_level))
        .num_threads(num_threads)
        .build();
    io::copy(&mut input, &mut parz)?;
    parz.finish()?;
    Ok(())
}
/// Parse args and set up logging / tracing
fn setup() -> Result<Opts, Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    Ok(Opts::from_args())
}
