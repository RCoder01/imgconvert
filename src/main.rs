use clap::Parser;
use image::ImageReader;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    ImageReader::open(args.input)?.decode()?.save(args.output)?;
    Ok(())
}
