use std::{
    fs::File,
    io::{stdin, BufReader, BufWriter},
    path::Path,
};

use clap::Parser;
use image::{
    codecs::{
        png::{PngDecoder, PngEncoder},
        qoi::{QoiDecoder, QoiEncoder},
    },
    ImageDecoder, ImageEncoder, ImageFormat, ImageReader,
};
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
    output: String,
    #[arg(short, long)]
    input_type: Option<String>,
    #[arg(short, long)]
    output_type: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    convert(args)?;
    // let input = Path::new(&args.input);
    // let output = Path::new(&args.output);
    // if input.ends_with(".png") && output.ends_with(".qoi") {
    //     png2qoi(input, output)?;
    // } else if input.ends_with(".qoi") && output.ends_with(".png") {
    //     qoi2png(input, output)?;
    // }
    Ok(())
}

fn convert(args: Args) -> anyhow::Result<()> {
    ImageReader::open(args.input)?.decode()?.save(args.output)?;
    Ok(())
}

fn png2qoi(input: &Path, output: &Path) -> anyhow::Result<()> {
    let input = BufReader::new(File::open(input)?);
    let reader = PngDecoder::new(input)?;
    let mut bytes = vec![0; reader.total_bytes() as usize];
    let (width, height) = reader.dimensions();
    let color_type = reader.original_color_type();
    reader.read_image(&mut bytes)?;
    let writer = QoiEncoder::new(BufWriter::new(File::open(output)?));
    writer.write_image(&bytes, width, height, color_type)?;
    Ok(())
}

fn qoi2png(input: &Path, output: &Path) -> anyhow::Result<()> {
    let input = BufReader::new(File::open(input)?);
    let reader = QoiDecoder::new(input)?;
    let mut bytes = vec![0; reader.total_bytes() as usize];
    let (width, height) = reader.dimensions();
    let color_type = reader.original_color_type();
    reader.read_image(&mut bytes)?;
    let writer = PngEncoder::new(BufWriter::new(File::open(output)?));
    writer.write_image(&bytes, width, height, color_type)?;
    Ok(())
}

#[derive(Debug, Error)]
#[error("Input path has no valid extension and input_type does not specify a valid image type")]
pub struct MissingImageType;

// fn thing(args: Args) -> anyhow::Result<()> {
//     let reader = BufReader::new(File::open(args.input.clone())?);
//     let mut reader = ImageReader::new(reader);
//     let input_type_given = args
//         .input_type
//         .map(|input_type| {
//             ImageFormat::from_extension(input_type.clone())
//                 .or(ImageFormat::from_mime_type(input_type))
//         })
//         .flatten()
//         .or(ImageFormat::from_path(args.input).ok());
//     if let Some(input_type) = input_type_given {
//         reader.set_format(input_type);
//     } else {
//         reader = reader.with_guessed_format()?;
//         let Some(format) = reader.format() else {
//             return Err(MissingImageType.into());
//         };
//         let mut input = String::new();
//         loop {
//             println!(
//                 "No explicit input image type given, continue with deduced image type {}? (y/N): ",
//                 format.to_mime_type()
//             );
//             stdin().read_line(&mut input)?;
//             let input_lower = input.to_lowercase();
//             if input_lower == "y" || input_lower == "yes" {
//                 break;
//             } else if input_lower == "n" || input_lower == "no" {
//                 return Err(MissingImageType.into());
//             } else {
//                 println!("Invalid input, recieved \"{input}\"")
//             }
//         }
//     };
//     let output_type_given = args
//         .output_type
//         .map(|output_type| {
//             ImageFormat::from_extension(output_type.clone())
//                 .or(ImageFormat::from_mime_type(output_type))
//         })
//         .flatten()
//         .or(ImageFormat::from_path(args.output).ok());
//     ImageEncoder
//     Ok(())
// }
