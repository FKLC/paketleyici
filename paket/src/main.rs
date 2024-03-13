use serde::Deserialize;
use std::{
    env, fs,
    io::{self, Read, Seek},
    path::PathBuf,
    process::{ExitCode, Stdio},
};
use tar::Archive;

#[derive(Deserialize)]
struct RunnerConfig {
    #[serde(rename(deserialize = "f"))]
    folder: String,
    #[serde(rename(deserialize = "c"))]
    command: Vec<String>,
    #[serde(rename(deserialize = "a"))]
    args_pos: u8,
    #[serde(rename(deserialize = "p"))]
    append_path: bool,
}

fn main() -> ExitCode {
    let mut executable = fs::File::open(&env::current_exe().unwrap()).unwrap();

    executable.seek(io::SeekFrom::End(-8)).unwrap();
    let paket_size = u32::from_le_bytes(read_buffer(&mut executable, 4).try_into().unwrap()) as u64;

    executable.seek(io::SeekFrom::Start(paket_size)).unwrap();
    let config_size =u16::from_le_bytes(read_buffer(&mut executable, 2).try_into().unwrap()) as u64;
    let config_buffer = read_buffer(&mut executable, config_size);
    let config: RunnerConfig = serde_json::from_slice(&config_buffer).unwrap();

    let output_path = env::temp_dir().join(&config.folder);
    if !output_path.is_dir() {
        fs::create_dir(&output_path).unwrap();

        let tarball_size = u64::from_le_bytes(read_buffer(&mut executable, 8).try_into().unwrap());
        let mut ar = Archive::new(executable.take(tarball_size));
        ar.unpack(&output_path).unwrap();
    }

    let mut args = config
        .command
        .iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    if config.args_pos > 0 {
        let runner_args = env::args().skip(1).collect::<Vec<_>>().join(" ");
        args[config.args_pos as usize - 1].push_str(&runner_args);
    } else {
        args.extend(env::args().skip(1));
    }

    let program = match config.append_path {
        true => output_path.join(&args[0]),
        false => PathBuf::from(args[0].clone()),
    };

    let mut child = std::process::Command::new(program)
        .args(&args[1..])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = child.wait().unwrap();
    match status.code() {
        Some(code) => {
            let code: u8 = code.try_into().unwrap();
            ExitCode::from(code)
        }
        None => ExitCode::FAILURE,
    }
}

fn read_buffer(file: &mut fs::File, size: u64) -> Vec<u8> {
    let mut buffer = vec![0; size as usize];
    file.read_exact(&mut buffer).unwrap();
    buffer
}
