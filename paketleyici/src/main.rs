use serde::{Deserialize, Serialize};
use std::env::args;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

#[derive(Deserialize)]
struct Config {
    paket: String,
    tarball: String,
    folder: String,
    command: Vec<String>,
    args_pos: u8,
    append_path: bool,
}

#[derive(Serialize)]
struct RunnerConfig {
    #[serde(rename(serialize = "f"))]
    folder: String,
    #[serde(rename(serialize = "c"))]
    command: Vec<String>,
    #[serde(rename(serialize = "a"))]
    args_pos: u8,
    #[serde(rename(serialize = "p"))]
    append_path: bool,
}

fn main() {
    if args().len() != 2 {
        println!(
            "Usage: {} <config_file>",
            args().next().unwrap()
        );
        return;
    }

    let config = read_config();
    let output_name = create_output_name(&config.paket);

    let mut paket_file = File::open(&config.paket).unwrap();
    let mut tarball_file = File::open(&config.tarball).unwrap();
    let mut output_file = File::create(output_name).unwrap();

    let paket_size: u64 = paket_file.metadata().unwrap().len().try_into().unwrap();
    let tarball_size = tarball_file.metadata().unwrap().len();

    // Write the paket file to the output file
    io::copy(&mut paket_file, &mut output_file).unwrap();

    // Write the runner config to the output file
    let runner_config = RunnerConfig {
        folder: config.folder,
        command: config.command,
        args_pos: config.args_pos,
        append_path: config.append_path,
    };
    let runner_config = serde_json::to_string(&runner_config).unwrap();
    output_file.write(&(runner_config.len() as u16).to_le_bytes()).unwrap();
    output_file.write(runner_config.as_bytes()).unwrap();

    // Write the tarball file to the output file
    output_file.write(&tarball_size.to_le_bytes()).unwrap();
    io::copy(&mut tarball_file, &mut output_file).unwrap();

    // Write the paket size to the output file
    output_file.write(&paket_size.to_le_bytes()).unwrap();
}

fn read_config() -> Config {
    let config_path = args().nth(1).unwrap();
    let config_file = File::open(config_path).unwrap();
    serde_json::from_reader(config_file).unwrap()
}

fn create_output_name(paket_path: &str) -> String {
    let paket_path = Path::new(&paket_path);
    let paket_name = paket_path.file_stem().unwrap().to_str().unwrap();
    let paket_extension = paket_path.extension().unwrap_or_default().to_str().unwrap();

    if paket_extension == "" {
        format!("{}_out", paket_name)
    } else {
        format!("{}_out.{}", paket_name, paket_extension)
    }
}
