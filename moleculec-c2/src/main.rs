#![allow(clippy::useless_format)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::too_many_arguments)]

mod generator;
mod utils;

use clap::{App, Arg};
use molecule_codegen::IntermediateFormat;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() {
    let matches = App::new("moleculec-c2")
        .version("0.1.1")
        .author("Xu Jiandong<xjd@cryptape>")
        .about("Improved C plugin for the molecule serialization system")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Set an input file (JSON format)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Set an output file (C/Rust source code)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("rust")
                .short("r")
                .long("rust")
                .help("Output Rust code"),
        )
        .get_matches();
    let input = matches.value_of("input").unwrap_or("-");
    // eprintln!("Using input file: {}", input);
    let output = matches.value_of("output").unwrap_or("-");
    // eprintln!("Using output file: {}", output);
    let gen_rust = matches.is_present("rust");

    let mut reader: Box<dyn Read>;
    if input != "-" {
        reader = Box::new(File::open(input).expect("Failed to open input file"));
    } else {
        reader = Box::new(io::stdin());
    };

    let stdout = io::stdout();
    let mut writer: Box<dyn Write>;
    if output != "-" {
        writer = Box::new(File::open(output).expect("Failed to open output file"));
    } else {
        let stdout_handle = stdout.lock();
        writer = Box::new(stdout_handle);
    };

    let mut content = Vec::<u8>::new();
    reader
        .read_to_end(&mut content)
        .expect("Failed to read input file");

    let format = IntermediateFormat::JSON;
    let ast = format.recover(&content).unwrap();

    let mut output = utils::Output::new();
    if gen_rust {
        utils::gen_rust(&mut output, &ast).unwrap();
    } else {
        utils::gen_c(&mut output, &ast).unwrap();
    }

    let mut file_name = OsString::from("STDIN");
    if input != "-" {
        let path = Path::new(input);
        file_name = path.file_stem().unwrap().into();
    }

    let all_data = if gen_rust {
        output.combine_rust(file_name.to_str().unwrap())
    } else {
        output.combine_c(file_name.to_str().unwrap())
    };
    writer.write_all(all_data.as_bytes()).unwrap();
    writer.flush().unwrap();
}
