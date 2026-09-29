use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Cli {
    // print system information
    #[arg(short, long, default_value_t = true)]
    info: bool,

    #[arg(long)]
    ascii_path: Option<String>,
}

struct Info {
    ascii_art: String,
    time: String,
    os: String,
    cpu: String,
    gpu: String,
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    #[serde(alias = "ascii-path")]
    ascii_path: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let cfg: Config = {
        let config_path = dirs::home_dir()
            .expect("could not determine home directory")
            .join(".config/rust_fetch/config.toml");

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path).expect("failed to read config file");
            toml::from_str(&contents).expect("failed to parse config file")
        } else {
            Config::default()
        }
    };
    let ascii_path = cli.ascii_path.or(cfg.ascii_path);

    if let Some(path) = ascii_path {
        let art = fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read ascii file '{}': {}", path, e));
        println!("{}", art);
    }

    if cli.info {
        println!("info: {}", cli.info);
    }
}
