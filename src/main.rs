use chrono::{self, Local};
use clap::{Arg, ArgMatches, Command};
use datamatrix::{DataMatrix, SymbolList};
use env_logger::Builder;
use image::{GrayImage, ImageBuffer, Luma};
use log::LevelFilter;
use qrcode_generator::{self, QrCodeEcc};
use std::io::Write;

fn generate_qr(
    input: &str,
    error_correction: QrCodeEcc,
    size: u32,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    qrcode_generator::to_image_buffer(input, error_correction, size as usize)
        .expect("Failed to generate QR code")
}

fn generate_data_matrix(input: &str, blocksize: u32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let size = blocksize as usize;
    let bitmap = DataMatrix::encode(input.as_bytes(), SymbolList::default())
        .expect("Failed to generate data matrix")
        .bitmap();

    let width = ((bitmap.width() + 2) * size) as u32;
    let height = ((bitmap.height() + 2) * size) as u32;
    let mut image = GrayImage::from_pixel(width, height, Luma([255]));

    for (x, y) in bitmap.pixels() {
        for i in 0..blocksize {
            for j in 0..blocksize {
                let x_i = (x + 1) * size + j as usize;
                let y_j = (y + 1) * size + i as usize;
                image.put_pixel(x_i as u32, y_j as u32, Luma([0]))
            }
        }
    }

    return image;
}

fn save(filename: &str, image: ImageBuffer<Luma<u8>, Vec<u8>>) {
    image.save(filename).expect("Failed to save to image");
}

fn init() -> ArgMatches {
    // Accept command line arguments
    let args = Command::new("Matrix Code Generator")
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .about(clap::crate_description!())
        .arg(
            Arg::new("data")
                .help("The data to encode in the matrix code")
                .takes_value(true)
                .value_name("DATA"),
        )
        .arg(
            Arg::new("filename")
                .help("The filename of the output file")
                .takes_value(true)
                .value_name("FILENAME"),
        )
        .arg(
            Arg::new("qr")
                .help("Generate a QR-Code")
                .short('q')
                .long("qr")
                .alias("quickresponse")
                .required_unless_present_any(&["dm"])
                .conflicts_with_all(&["dm"]),
        )
        .arg(
            Arg::new("dm")
                .help("Generate a DataMatrix-Code")
                .short('d')
                .long("dm")
                .alias("datamatrix"),
        )
        .arg(
            Arg::new("size")
                .help("The Size of the generated image, this is the size of pixels for qr codes and the size of pixels per block for datamatrix codes")
                .short('s')
                .long("size")
                .takes_value(true)
                .value_name("SIZE")
                .value_parser(clap::value_parser!(u64).range(0..4294967296))
        )
        .arg(
            Arg::new("ecc")
                .help("The error correction scheme to use when generating qr codes")
                .short('e')
                .long("ecc")
                .alias("error-correction")
                .takes_value(true)
                .value_name("ECC")
                .requires("qr")
                .value_parser(clap::builder::PossibleValuesParser::new(
                    ["l", "L", "low", "Low", "LOW", "m", "M", "medium", "Medium", "MEDIUM", "q", "Q", "quartile", "Quartile", "QUARTILE", "h", "H", "high", "High", "HIGH"]
                ))
                .default_value("m")

        )
        .get_matches();

    // Build the logger
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {} - {}: {}",
                record.level(),
                Local::now().format("%d/%m/%y %H:%M:%S"),
                record.target(),
                record.args(),
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    return args;
}

fn main() {
    let args = init();

    let image = if args.is_present("qr") {
        let ecc = match args
            .value_of("ecc")
            .expect("No ecc choice was made")
            .to_lowercase()
            .chars()
            .next()
            .unwrap_or('m')
        {
            'l' => QrCodeEcc::Low,
            'm' => QrCodeEcc::Medium,
            'q' => QrCodeEcc::Quartile,
            'h' => QrCodeEcc::High,
            _ => QrCodeEcc::Medium,
        };
        generate_qr(
            args.value_of("data").expect("No data provided"),
            ecc,
            args.value_of("size")
                .unwrap_or("1024")
                .parse::<u32>()
                .unwrap_or(1024),
        )
    } else if args.is_present("dm") {
        generate_data_matrix(
            args.value_of("data").expect("No data provided"),
            args.value_of("size")
                .unwrap_or("5")
                .parse::<u32>()
                .unwrap_or(5),
        )
    } else {
        panic!("No valid matrix code choice was made!");
    };

    save(
        args.value_of("filename").expect("No filename provided"),
        image,
    );
}
