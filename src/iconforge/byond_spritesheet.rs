use super::spritesheet;
use crate::{byond::catch_panic, iconforge::spritesheet::HeadlessResult, jobs};
use tracy_full::frame;

byond_fn!(fn iconforge_generate(file_path, spritesheet_name, sprites, hash_icons, generate_dmi, flatten) {
	let file_path = file_path.to_owned();
	let spritesheet_name = spritesheet_name.to_owned();
	let sprites = sprites.to_owned();
	let hash_icons = hash_icons.to_owned();
	let generate_dmi = generate_dmi.to_owned();
	let flatten = flatten.to_owned();
	let result = Some(match catch_panic(|| spritesheet::generate_spritesheet(&file_path, &spritesheet_name, &sprites, &hash_icons, &generate_dmi, &flatten)) {
		Ok(o) => match o {
			Ok(o) => o,
			Err(e) => e.to_string()
		},
		Err(e) => e.to_string()
	});
	frame!();
	result
});

byond_fn!(fn iconforge_generate_async(file_path, spritesheet_name, sprites, hash_icons, generate_dmi, flatten) {
	let file_path = file_path.to_owned();
	let spritesheet_name = spritesheet_name.to_owned();
	let sprites = sprites.to_owned();
	let hash_icons = hash_icons.to_owned();
	let generate_dmi = generate_dmi.to_owned();
	let flatten = flatten.to_owned();
	Some(jobs::start(move || {
		let result = match catch_panic(|| spritesheet::generate_spritesheet(&file_path, &spritesheet_name, &sprites, &hash_icons, &generate_dmi, &flatten)) {
			Ok(o) => match o {
				Ok(o) => o,
				Err(e) => e.to_string()
			},
			Err(e) => e.to_string()
		};
		frame!();
		result
	}))
});

byond_fn!(fn iconforge_generate_headless(file_path, sprites, flatten) {
	let file_path = file_path.to_owned();
	let sprites = sprites.to_owned();
	let flatten = flatten.to_owned();
	let result = Some(match catch_panic::<_, HeadlessResult>(|| spritesheet::generate_headless_str(&file_path, &sprites, &flatten)) {
		Ok(o) => match serde_json::to_string::<HeadlessResult>(&o) {
			Ok(o) => o,
			Err(_) => String::from("{\"error\":\"Serde serialization error\"}") // nigh impossible but whatever
		},
		Err(e) => match serde_json::to_string::<HeadlessResult>(&HeadlessResult {
			file_path: None,
			width: None,
			height: None,
			error: Some(e.to_string()),
		}) {
			Ok(o) => o,
			Err(_) => String::from("{\"error\":\"Serde serialization error\"}")
		}
	});
	frame!();
	result
});

byond_fn!(fn iconforge_cache_valid(input_hash, dmi_hashes, sprites) {
	let input_hash = input_hash.to_owned();
	let dmi_hashes = dmi_hashes.to_owned();
	let sprites = sprites.to_owned();
	let result = Some(match catch_panic(|| spritesheet::cache_valid(&input_hash, &dmi_hashes, &sprites)) {
		Ok(o) => match o {
			Ok(o) => o,
			Err(e) => e.to_string()
		},
		Err(e) => e.to_string()
	});
	frame!();
	result
});

byond_fn!(fn iconforge_cache_valid_async(input_hash, dmi_hashes, sprites) {
	let input_hash = input_hash.to_owned();
	let dmi_hashes = dmi_hashes.to_owned();
	let sprites = sprites.to_owned();
	let result = Some(jobs::start(move || {
		match catch_panic(|| spritesheet::cache_valid(&input_hash, &dmi_hashes, &sprites)) {
			Ok(o) => match o {
				Ok(o) => o,
				Err(e) => e.to_string()
			},
			Err(e) => e.to_string()
		}
	}));
	frame!();
	result
});
