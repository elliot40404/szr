use std::env;
use std::ffi::OsStr;
use std::fs::metadata;
use std::process::ExitCode;

const HELP: &'static str = r#"szr v0.1.0 by Elliot40404
Prints file sizes in bytes, kilobytes, megabytes, and gigabytes
Usage: sz <file> <file> <file> ..."#;

fn main() -> ExitCode {
    match env::args_os().skip(1).count() {
        0 => {
            println!("{}", HELP);
            ExitCode::FAILURE
        }
        _ => {
            env::args_os()
                .skip(1)
                .for_each(|file| print_file_size(&file));
            ExitCode::SUCCESS
        }
    }
}

fn print_file_size(file: &OsStr) {
    if let Ok(metadata) = metadata(file) {
        let size = metadata.len();
        let size_kb = size as f64 / 1024.0;
        let size_mb = size_kb / 1024.0;
        let size_gb = size_mb / 1024.0;
        println!(
            "{} B  {:.2} KB  {:.2} MB  {:.2} GB  {:?}",
            size, size_kb, size_mb, size_gb, file
        );
    } else {
        eprintln!("Error: File not found or could not get metadata");
    }
}
