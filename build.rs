//! Buildscript which will save a `iconforge.dm` with the DLL's public API.

use std::{env, fs::File, io::Write, path::PathBuf};

macro_rules! feature_dm_file {
	($name:expr) => {
		&"dmsrc/{}.dm".replace("{}", $name)
	};
}

fn main() {
	if env::var("CARGO_PUBLISH").is_ok() {
		return;
	}
	let dm_path = if let Ok(custom_dir) = env::var("DM_OUT_DIR") {
		if custom_dir.trim().is_empty() {
			return;
		}
		PathBuf::from(custom_dir).join("iconforge.dm")
	} else {
		let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

		PathBuf::from(manifest_dir)
			.join("target")
			.join("iconforge.dm")
	};
	let mut f = File::create(&dm_path).unwrap_or_else(|_| {
		panic!(
			"Couldn't open `{}` for writing iconforge-rs DM headers. Set DM_OUT_DIR to an empty \
			 string to disable writing headers or to an absolute path you want to write headers \
			 to.",
			dm_path.display()
		)
	});

	// header
	writeln!(
		f,
		"{}",
		std::fs::read_to_string(feature_dm_file!("main")).unwrap()
	)
	.unwrap();

	writeln!(
		f,
		"{}",
		std::fs::read_to_string(feature_dm_file!("iconforge")).unwrap()
	)
	.unwrap();

	writeln!(
		f,
		"{}",
		std::fs::read_to_string(feature_dm_file!("universal_icon")).unwrap()
	)
	.unwrap();
}
