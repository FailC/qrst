use clap::Parser;
use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value_t = false)]
    file: bool,
}

fn save_to_file(qrcode: &fast_qr::QRCode) {
    let _img = ImageBuilder::default()
        .shape(Shape::RoundedSquare)
        .background_color([255, 255, 255, 255])
        .fit_width(600)
        .to_file(&qrcode, "out.png");
}

fn build_qr_code(args: Args) -> Result<(), String> {
    let input = args.input;
    let Ok(qrcode) = QRBuilder::new(input).build() else {
        return Err(String::from("ERROR: couldn't build QR-code"));
    };
    if args.file == true {
        save_to_file(&qrcode);
        return Ok(());
    }
    let string = qrcode.to_str(); // .print() exists
    println!("{}", string);
    Ok(())
}

fn main() {
    let args = Args::parse();
    // let input = args.input.clone();

    match build_qr_code(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::FAILURE
        }
    };
}
