use clap::Parser;
use fast_qr::convert::{image::ImageBuilder, svg::SvgBuilder, Builder, Shape};
use fast_qr::qr::{QRBuilder, QRCodeError};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Create QR-Code
    #[arg(short, long)]
    input: String,

    /// Save as PNG
    #[arg(short, long)]
    file: Option<Option<String>>,

    /// Save as SVG
    #[arg(short, long)]
    svg: Option<Option<String>>,
}

fn save_to_svg(qrcode: &fast_qr::QRCode, file_name: String) {
    let _svg = SvgBuilder::default()
        .shape(Shape::RoundedSquare)
        .to_file(&qrcode, &file_name);
}

fn save_to_png(qrcode: &fast_qr::QRCode, file_name: String) {
    let _img = ImageBuilder::default()
        .shape(Shape::Square)
        .background_color([255, 255, 255, 255])
        .fit_width(600)
        .to_file(&qrcode, &file_name);
}

fn build_qr_code(args: Args) -> Result<(), QRCodeError> {
    let input = args.input;
    let qrcode = match QRBuilder::new(input).build() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    if args.file.is_some() || args.svg.is_some() {
        if let Some(file_name) = args.file {
            let file_name = file_name.unwrap_or(String::from("out.png"));
            save_to_png(&qrcode, file_name);
        }

        if let Some(file_name) = args.svg {
            let file_name = file_name.unwrap_or(String::from("out.svg"));
            save_to_svg(&qrcode, file_name);
        }
        return Ok(());
    }
    let string = qrcode.to_str();
    println!("{}", string);
    Ok(())
}

fn main() {
    let args = Args::parse();

    match build_qr_code(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            ExitCode::FAILURE
        }
    };
}
