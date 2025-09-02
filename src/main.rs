#![allow(unused_imports)]
#![allow(dead_code)]

use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ptr::null;
use clap::builder::Str;
use clap::{Parser, Subcommand};
use image::{ImageReader, DynamicImage, ImageEncoder};
use std::fs::File;
use std::io::BufWriter;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};

// Module definitions are almost always in file that "owns" the module: here main.rs, in android lib.rs
pub mod gui;
pub mod compress;
#[derive(Parser)] // automatically implements the argument parser for a struct
struct Args {
    #[command(subcommand)]
    mode: Option<Mode>,
}

#[derive(Subcommand)]
enum Mode {
    Cli {
        /// The input file path // tells clap how to map command line flags i.e --file_type or -f
        #[arg(short, long)]
        input: PathBuf,

        /// The output file name (optional)
        #[arg(short, long)]
        output_name: Option<String>,

        /// Compression quality (optional)
        #[arg(short, long)]
        quality: Option<u8>,
    },
    Gui,
}


fn main() {
    let val = (is_ghostscript_installed(), is_cargo_installed());
    match val {
        (_, true) => { // cargo installed, install dependencies
            install_dependency(String::from("clap").as_ref()).expect("Error installing clap");
            install_dependency(String::from("image").as_ref()).expect("Error installing image");
            install_dependency(String::from("rfd").as_ref()).expect("Error installing rfd");
            install_dependency(String::from("eframe").as_ref()).expect("Error installing eframe");
        },
        (_ , _) => { // installer script perhaps not run, or something went wrong with the installation
            eprintln!("Did you run the installer script before starting this program? You need Ghostscript and Cargo for this program to run");
            return;
        }
    };

    let args: Args = match Args::try_parse() {
        Ok(args) => args,
        Err(_) => {
            eprintln!("Not enough arguments!");
            return;
        }
    };

    match args.mode {
        Some(Mode::Cli { input, output_name, quality }) => {
            compress::compress_file(input, output_name, quality).expect("Compression module failed to compress");  
        },
        _ => {
            // Launch gui as default mode
            gui::run_gui().expect("Failed running gui");
        },
    }
}



/// Checks if Ghostscript is installed by attempting to run it with the `-v` flag.
fn is_ghostscript_installed() -> bool {
    // Constucts a command for launching the program at path program
    Command::new("gswin64c")
        .arg("-v")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Checks if Ghostscript is installed by attempting to run it with the `-v` flag.
fn is_cargo_installed() -> bool {
    // Constucts a command for launching the program at path program
    Command::new("cargo")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Install all dependencies (clap and image) if not found, returns a true
/// Would not work if run from command shell
fn install_dependency(dep: &str) -> io::Result<bool> {
    let status;
    if let Ok(file) = std::fs::read_to_string("Cargo.lock") {
        match file.contains(dep) {
            false => { //only download dependencies if they're not already installed
                status = Command::new("cargo")
                            .args(["add", dep])
                            .status()
                            .expect("Failed to run cargo add");
                if status.success() {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            },
            _ => {
                Ok(false)
            }
        }
    } else {
        return Ok(false)
    }
}
// TODO: Try to add this program programmatically to PATH

fn print_usage_and_exit() -> io::Result<()> {
    eprintln!("==================================================================");
    eprintln!("Read the README.md to begin");
    eprintln!("Use this program to compress the size of PDF files and JPGs using Ghostscript");
    eprintln!();
    eprintln!("<> is a required argument, [] is an optional argument, Current supported file types are PDF and JPG (or JPEG)");
    eprintln!();
    eprintln!("If an output file name is not provided, the program will use the input file name with a suffix '_compressed' added before the extension.");
    eprintln!();
    eprintln!("Usage: cmd_compressor -f <file_type> -i <input_file_path> -o [output_file_name]");
    eprintln!("You can use short or long flags for the arguments, e.g., -f or --file_type");
    eprintln!();
    eprintln!();
    eprintln!("Example: cmd_compressor -f --pdf -i .\\Downloads\\input.pdf -o output");
    eprintln!("             will return a compressed file named output.pdf in the source directory of the input file i.e .\\Downloads\\output.pdf");
    eprintln!();
    eprintln!();
    eprintln!("Example: cmd_compressor -f --jpg -i .\\Downloads\\input.jpg");
    eprintln!("             will return a compressed file named input_compressed.jpg in the source directory of the input file i.e .\\Downloads\\input_compressed.jpg");
    eprintln!();
    eprintln!("==================================================================");
    Ok(())
}
