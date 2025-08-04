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

pub mod gui;
#[derive(Parser)] // automatically implements the argument parser for a struct
#[command(author, version, about)]
/* #[command(author = "Adewale Adenuga")]
#[command(version = "1.0")]
#[command(about = "Use to compress images and PDFs")] */
struct Args {
    #[command(subcommand)]
    command: CommandLineArgs,
}

#[derive(Subcommand)]
enum CommandLineArgs {
    Compress {
        /// The file type to compress (e.g., pdf, jpg)
        #[arg(short, long)] // tells clap how to map command line flags i.e --file_type or -f
        file_type: String,

        /// The input file path
        #[arg(short, long)]
        input: PathBuf,

        /// The output file name (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
    Gui,
}


fn main() {
    let val = (is_ghostscript_installed(), is_cargo_installed());
    match val {
        (_, true) => { // cargo installed, install dependencies
            install_dependency(String::from("clap").as_ref()).expect("Error installing clap");
            install_dependency(String::from("image").as_ref()).expect("Error installing image");
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
            //print_usage_and_exit().expect("Not enough arguments provided");
            return;
        }
    };

    let extension;
    let input_file_name;

    // Formulate the output string/path
    let output_file_name;
    let output_dir;

    match args.command {
        CommandLineArgs::Compress { file_type, input, output } => {
            extension = input.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
            input_file_name = input.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();

            //pattern matching, destructures output only if it is Some(), args.output is partially moved and can't be reused
            // v takes ownership of the inner value
            if let Some(v) = output { 
                output_file_name = format!("{}.{}", v, &extension);
            } else {
                output_file_name = format!("{}_compressed.{}", &input_file_name, &extension);
            }
            output_dir = input.parent().unwrap().join(output_file_name);

            println!("output dir is {:?}", &output_dir);

            // Actually navigate compression
            match file_type.as_str() {
                "pdf" => compress_pdf(&input, &output_dir),
                "jpg" | "jpeg" | "img" => compress_jpg(&input, &output_dir),
                _ => {
                    eprintln!("File type not currently supported");
                    Ok(())
                }
            }.expect("File type provided not supported by cmd_compressor");
        },
        CommandLineArgs::Gui => {

        },
    }
}

/// Compresses a PDF file using Ghostscript.
fn compress_pdf(input: &Path, output: &Path) -> io::Result<()> {
    let status = Command::new("gswin64c")
        .args([
            "-sDEVICE=pdfwrite",
            "-dCompatibilityLevel=1.4",
            "-dPDFSETTINGS=/ebook", // or /screen
            "-dNOPAUSE", "-dQUIET", "-dBATCH",
            &format!("-sOutputFile={}", output.display()),
            &input.display().to_string(),
        ]).status().expect("Compress pdf ghostscript command run");
    if status.success() {
        println!("PDF compressed to {}", output.display())
    }
    Ok(())
}

/// Compresses a JPG using Ghostscript.
fn compress_jpg(input: &Path, output: &Path) -> io::Result<()> {
    let img = ImageReader::open(input)?.decode().expect("Could not decode image");
    let out_buf = File::create(output)?;
    let writer = BufWriter::new(out_buf);
    let mut encoder = JpegEncoder::new_with_quality(writer, 70);

    encoder.encode_image(&img).expect("Image encoding failed.");

    println!("✅ JPEG compressed to {}", output.display());
    Ok(())
}

/// Compress png file format
fn compress_png(input: &Path, output: &Path) -> io::Result<()> {

    let img = ImageReader::open(input)?.decode().expect("Could not decode message");
    let file = File::create(output)?;
    let writer = BufWriter::new(file);

    let encoder = PngEncoder::new(writer);
    encoder.write_image(img.as_bytes(), img.width(), img.height(), img.color().into()).expect("Png encoding failed");
    Ok(())
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
