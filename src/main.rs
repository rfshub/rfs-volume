use std::env;
use std::path::PathBuf;

mod config;
mod volumes;

fn main() {
    let args: Vec<String> = env::args().collect();

    let toml_path = if args.len() > 1 {
        let input = PathBuf::from(&args[1]);
        if input.is_file() {
            Some(input)
        } else if input.is_dir() {
            config::find_rfs_toml(&input)
        } else {
            None
        }
    } else {
        config::find_rfs_toml(&env::current_dir().unwrap())
    };

    match toml_path {
        Some(path) => {
            if config::set_rfs_toml_path(&path) {
                println!("> Loaded rfs.toml from {}", path.display());
            } else {
                eprintln!("! Invalid rfs.toml file at {}", path.display());
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("! Could not find rfs.toml");
            std::process::exit(1);
        }
    }

    let json = volumes::get_volumes_json();
    println!("+ Volumes JSON: {}", json);
}
