use std::{path::PathBuf, u8};
use std::path::Path;
use std::io;
use std::process::Command;
use clap::{Parser, Subcommand};
use image::{ImageReader, DynamicImage, ImageEncoder};
use std::fs::File;
use std::io::BufWriter;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};

pub fn compress_file(
    input: PathBuf,
    output_name: Option<String>,
    quality: Option<u8>,
) -> std::io::Result<()> {

    eprintln!("inside compress file");

    // Formulate the output string/path
    let output_file_name;
    let output_dir;
    
    let extension = input
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    let input_file_name = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // pattern matching, destructures output_name only if it is Some(), args.output is partially moved and can't be reused
    // v takes ownership of the inner value
    if let Some(v) = output_name {
        output_file_name = format!("{}.{}", v, &extension);
    } else {
        output_file_name = format!("{}_compressed.{}", &input_file_name, &extension);
    }
    output_dir = input.parent().unwrap().join(output_file_name);

    println!("output dir is {:?}", &output_dir);

    // Actually navigate compression
    match extension.as_str() {
        "pdf" => compress_pdf(&input, &output_dir, &quality),
        "jpg" | "jpeg" | "img" => compress_jpg(&input, &output_dir, &quality),
        "png" => compress_png(&input, &output_dir),
        _ => {
            eprintln!("File type not currently supported");
            Ok(())
        }
    }
    .expect("File type provided not supported by cmd_compressor");
    Ok(())
}

/// Compresses a PDF file using Ghostscript.
fn compress_pdf(input: &Path, output: &Path, quality: &Option<u8>) -> io::Result<()> {

    let compression_setting = match quality {
        Some(value) if *value > 0 && *value <= 25 => "/prepress",
        Some(value) if *value > 25 && *value <= 50 => "/printer",
        Some(value) if *value > 50 && *value <= 75 => "/ebook",
        Some(_) => "/screen",
        None => "/default"
    };

    eprintln!("compression setting is {}", compression_setting);

    let status = Command::new("gswin64c")
        .args([
            "-sDEVICE=pdfwrite",
            "-dCompatibilityLevel=1.4",
            &format!("-dPDFSETTINGS={}", compression_setting),
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
fn compress_jpg(input: &Path, output: &Path, quality: &Option<u8>) -> io::Result<()> {
    let img = ImageReader::open(input)?.decode().expect("Could not decode image");
    let out_buf = File::create(output)?;
    let writer = BufWriter::new(out_buf);
    let quality_value = 100 - quality.unwrap_or(70);
    let mut encoder = JpegEncoder::new_with_quality(writer, quality_value);

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
