#![allow(unused_imports)]
#![allow(dead_code)]

use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use clap::Parser;
use image::{ImageReader, DynamicImage, ImageEncoder};
use std::fs::File;
use std::io::BufWriter;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};

#[derive(Parser)] // automatically implements the argument parser for a struct
struct Args {
    /// The file type to compress (e.g., pdf, jpg)
    #[arg(short, long)] // tells clap how to map command line flags i.e --file_type or -f
    file_type: String,

    /// The input file path
    #[arg(short, long)]
    input: PathBuf,

    /// The output file name (optional)
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args: Args = match Args::try_parse() {
        Ok(args) => args,
        Err(_) => {
            print_usage_and_exit().expect("Not enough arguments provided");
            return;
        }
    };
    
    let extension = args.input.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
    let input_file_name = args.input.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();

    // Formulate the output string/path
    let output_file_name;
    //pattern matching, destructures args.output only if it is Some(), args.output is partially moved and can't be reused
    // v takes ownership of the inner value
    if let Some(v) = args.output { 
        output_file_name = format!("{}.{}", v, &extension);
    } else {
        output_file_name = format!("{}_compressed.{}", &input_file_name, &extension);
    }
    
    let output_dir = args.input.parent().unwrap().join(output_file_name);

    println!("output dir is {:?}", &output_dir);

    match extension.as_str() {
        "pdf" => compress_pdf(&args.input, &output_dir),
        "jpg" | "jpeg" => compress_jpg(&args.input, &output_dir),
        _ => print_usage_and_exit()
    }.expect("File type provided not supported by cmd_compressor");

    let val = is_ghostscript_installed();
    match val {
        true => println!("Ghostscript is installed."),
        _ => {
            run_installer_script().expect("Ghostscript installer failed");
            return;
        }
    };

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

// TODO: Try to add this program programmatically to PATH

/// Run an installer script for Ghostscript.
fn run_installer_script() -> io::Result<()> {
    // Placeholder for the installer script logic
    // This function would typically execute a script or command to install Ghostscript
    // For example, it could use Command::new("installer_script.bat").status() to run a shell script
    Ok(())
}

fn print_usage_and_exit() -> io::Result<()> {
    eprintln!("==================================================================");
    eprintln!("Use this program to compress the size of PDF files and JPGs using Ghostscript");
    eprintln!();
    eprintln!("<> is a required argument, [] is an optional argument, Current supported file types are PDF and JPG");
    eprintln!();
    eprintln!("If an output file name is not provided, the program will use the input file name with a suffix '_compressed' added before the extension.");
    eprintln!();
    eprintln!("Usage: cmd_compressor <file_type> <input_file_path> [output_file_name]");
    eprintln!("You can use short or long flags for the arguments, e.g., -f or --file_type");
    eprintln!();
    eprintln!();
    eprintln!("Example: cmd_compressor --pdf .\\Downloads\\input.pdf output");
    eprintln!("             will return a compressed file named output.pdf in the source directory of the input file i.e .\\Downloads\\output.pdf");
    eprintln!();
    eprintln!();
    eprintln!("Example: cmd_compressor --jpg .\\Downloads\\input.jpg");
    eprintln!("             will return a compressed file named input_compressed.jpg in the source directory of the input file i.e .\\Downloads\\input_compressed.jpg");
    eprintln!();
    eprintln!("==================================================================");
    Ok(())
}
