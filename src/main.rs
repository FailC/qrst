use clap::Parser;
use fast_qr::convert::{image::ImageBuilder, svg::SvgBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;
use std::error::Error;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Create QR-Code from String
    input: String,

    /// Save as PNG
    #[arg(short, long, value_name = "NAME")]
    png: Option<Option<String>>,

    /// Save as SVG
    #[arg(short, long, value_name = "NAME")]
    svg: Option<Option<String>>,

    /// provide custom resolution for PNG
    #[arg(short, long, default_value_t = 600)]
    resolution: u32,
}

fn save_to_svg(qrcode: &fast_qr::QRCode, file_name: String) {
    let _svg = SvgBuilder::default()
        .shape(Shape::Square)
        .to_file(qrcode, &file_name);
}

fn save_to_png(qrcode: &fast_qr::QRCode, file_name: String, size: u32) {
    let _img = ImageBuilder::default()
        .shape(Shape::Square)
        .background_color([255, 255, 255, 255])
        .fit_width(size)
        .to_file(qrcode, &file_name);
}

fn build_qr_code(args: Args) -> Result<(), Box<dyn Error>> {
    let input = args.input;
    let qrcode = match QRBuilder::new(input).build() {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    if args.png.is_some() || args.svg.is_some() {
        if let Some(file_name) = args.png {
            let file_name = file_name.unwrap_or(String::from("out.png"));
            let size = args.resolution;
            if size > 10000 {
                // limiting resolution to prevent high ram usage while building image
                return Err("resolution too high: max_value=10000".into());
            }
            save_to_png(&qrcode, file_name, size);
        }

        if let Some(file_name) = args.svg {
            let file_name = file_name.unwrap_or(String::from("out.svg"));
            save_to_svg(&qrcode, file_name);
        }
        return Ok(());
    }
    let string = qrcode.to_str();
    println!("{string}");
    Ok(())
}

fn main() {
    let args = Args::parse();

    match build_qr_code(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("ERROR: {err}");
            ExitCode::FAILURE
        }
    };
}
