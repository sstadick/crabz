use flate2::Compression;
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder;
use std::io::{Read, Write};
use std::process::{Command, Output, Stdio};

fn formats() -> Vec<&'static str> {
    let formats = vec!["gzip", "bgzf", "mgzip", "deflate"];
    #[cfg(feature = "any_zlib")]
    let formats = {
        let mut formats = formats;
        formats.push("zlib");
        formats
    };
    #[cfg(feature = "snappy")]
    let formats = {
        let mut formats = formats;
        formats.push("snap");
        formats
    };
    formats
}

fn run_crabz(args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_crabz"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start crabz");

    let mut stdin = child.stdin.take().expect("crabz stdin was not piped");
    let input = input.to_vec();
    let input_writer = std::thread::spawn(move || stdin.write_all(&input));

    let output = child.wait_with_output().expect("failed to wait for crabz");
    input_writer
        .join()
        .expect("crabz input writer panicked")
        .expect("failed to write crabz input");
    output
}

fn multi_block_payload() -> Vec<u8> {
    let mut state = 0x4d59_5df4_d0f3_3173_u64;
    (0..(3 * 128 * 1024 + 257))
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state as u8
        })
        .collect()
}

fn payloads() -> Vec<(&'static str, Vec<u8>)> {
    vec![
        ("empty", Vec::new()),
        ("small", b"crabz round-trip smoke test\n".to_vec()),
        ("multi-block", multi_block_payload()),
    ]
}

fn assert_success(output: &Output, operation: &str, format: &str, threads: &str, payload: &str) {
    assert!(
        output.status.success(),
        "{operation} failed for {format}, {payload}, with {threads} threads: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn every_format_round_trips_through_the_cli() {
    for (payload, input) in payloads() {
        for format in formats() {
            for threads in ["1", "4"] {
                let compressed = run_crabz(
                    &[
                        "--format",
                        format,
                        "--compression-threads",
                        threads,
                        "--quiet",
                    ],
                    &input,
                );
                assert_success(&compressed, "compression", format, threads, payload);

                let decompressed = run_crabz(
                    &[
                        "--decompress",
                        "--format",
                        format,
                        "--compression-threads",
                        threads,
                        "--quiet",
                    ],
                    &compressed.stdout,
                );
                assert_success(&decompressed, "decompression", format, threads, payload);
                assert_eq!(
                    decompressed.stdout, input,
                    "round trip changed data for {format}, {payload}, with {threads} threads"
                );
            }
        }
    }
}

#[test]
fn gzip_is_compatible_with_flate2() {
    let input = multi_block_payload();

    let compressed = run_crabz(
        &["--format", "gzip", "--compression-threads", "4", "--quiet"],
        &input,
    );
    assert_success(&compressed, "compression", "gzip", "4", "multi-block");

    let mut decoded = Vec::new();
    MultiGzDecoder::new(&compressed.stdout[..])
        .read_to_end(&mut decoded)
        .expect("flate2 failed to decode crabz gzip output");
    assert_eq!(decoded, input, "flate2 changed crabz gzip output");

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(&input)
        .expect("failed to write flate2 gzip input");
    let compressed = encoder
        .finish()
        .expect("failed to finish flate2 gzip stream");
    let decompressed = run_crabz(
        &[
            "--decompress",
            "--format",
            "gzip",
            "--compression-threads",
            "4",
            "--quiet",
        ],
        &compressed,
    );
    assert_success(&decompressed, "decompression", "gzip", "4", "flate2 output");
    assert_eq!(
        decompressed.stdout, input,
        "crabz changed flate2 gzip output"
    );
}
