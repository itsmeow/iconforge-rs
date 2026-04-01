use std::{fs::File, io::Read, path::Path};

use clap::{Parser, Subcommand};
#[cfg(feature = "spritesheet")]
use iconforge_rs::iconforge::spritesheet::{
	spritesheet_from_universal_icons_str, spritesheet_multisize_from_universal_icons_str,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	#[cfg(feature = "spritesheet")]
	Generate {
		#[arg(long)]
		file_path: String,
		#[arg(long)]
		spritesheet_name: String,
		#[arg(long)]
		sprites_json_path: String,
		#[arg(long)]
		hash_icons: bool,
		#[arg(long)]
		generate_dmi: bool,
		#[arg(long)]
		flatten: bool,
	},
	#[cfg(feature = "spritesheet")]
	GenerateHeadless {
		#[arg(long)]
		file_path: String,
		#[arg(long)]
		sprites_json_path: String,
		#[arg(long)]
		flatten: bool,
	},
}

fn main() {
	let cli = Cli::parse();

	match &cli.command {
		#[cfg(feature = "spritesheet")]
		Some(Commands::Generate {
			file_path,
			spritesheet_name,
			sprites_json_path,
			hash_icons,
			generate_dmi,
			flatten,
		}) => {
			let sprites_json_path = Path::new(sprites_json_path);
			let mut sprites_json = match File::open(sprites_json_path) {
				Ok(o) => o,
				Err(e) => {
					eprintln!("Failed to read sprites_json_path: {e:#?}");
					return;
				}
			};
			let mut sprites_json_txt = String::new();
			if let Err(e) = sprites_json.read_to_string(&mut sprites_json_txt) {
				eprintln!("Failed to read sprites_json_path: {e:#?}");
				return;
			}
			let result_json = match spritesheet_multisize_from_universal_icons_str(
				file_path,
				spritesheet_name,
				&sprites_json_txt,
				if *hash_icons { "1" } else { "0" },
				if *generate_dmi { "1" } else { "0" },
				if *flatten { "1" } else { "0" },
			) {
				Ok(o) => o,
				Err(e) => {
					eprintln!("Errors during generate_spritesheet: {e:#?}");
					return;
				}
			};
			println!(
				"Generate result: {}",
				serde_json::to_string_pretty(&result_json).unwrap()
			);
		}
		#[cfg(feature = "spritesheet")]
		Some(Commands::GenerateHeadless {
			file_path,
			sprites_json_path,
			flatten,
		}) => {
			let sprites_json_path = Path::new(sprites_json_path);
			let mut sprites_json = match File::open(sprites_json_path) {
				Ok(o) => o,
				Err(e) => {
					eprintln!("Failed to read sprites_json_path: {e:#?}");
					return;
				}
			};
			let mut sprites_json_txt = String::new();
			if let Err(e) = sprites_json.read_to_string(&mut sprites_json_txt) {
				eprintln!("Failed to read sprites_json_path: {e:#?}");
				return;
			}
			let result = spritesheet_from_universal_icons_str(
				file_path,
				&sprites_json_txt,
				if *flatten { "1" } else { "0" },
			);
			if let Some(file_path) = result.file_path {
				println!(
					"Generated headless file successfully (size {}x{}): {file_path}",
					result.width.unwrap(),
					result.height.unwrap()
				);
			}
			if let Some(e) = result.error {
				eprintln!("Errors during generate_headless: {e}");
			}
		}
		#[cfg(not(feature = "spritesheet"))]
		&Some(_) => todo!(),
		None => {}
	}
}
