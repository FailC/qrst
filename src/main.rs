use clap::Parser;
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

fn build_qr_code(args: Args) -> Result<(), String> {
    let input = args.input;
    let Ok(qrcode) = QRBuilder::new(input).build() else {
        return Err(String::from("ERROR: couldn't build qr code"));
    };
    let str = qrcode.to_str(); // .print() exists
    if args.file == false {
        println!("{}", str);
    }
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
