use crate::{byond::catch_panic, core::gags, jobs};
use tracy_full::frame;

byond_fn!(fn iconforge_load_gags_config(config_path, config_json, config_icon_path) {
	let config_path = config_path.to_owned();
	let config_json = config_json.to_owned();
	let config_icon_path = config_icon_path.to_owned();
	let result = Some(match catch_panic(|| gags::load_gags_config(&config_path, &config_json, &config_icon_path)) {
		Ok(o) => match o {
			Ok(o) => o,
			Err(e) => e.to_string()
		},
		Err(e) => e.to_string()
	});
	frame!();
	result
});

byond_fn!(fn iconforge_load_gags_config_async(config_path, config_json, config_icon_path) {
	let config_path = config_path.to_owned();
	let config_json = config_json.to_owned();
	let config_icon_path = config_icon_path.to_owned();
	Some(jobs::start(move || {
		let result = match catch_panic(|| gags::load_gags_config(&config_path, &config_json, &config_icon_path)) {
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

byond_fn!(fn iconforge_gags(config_path, colors, output_dmi_path) {
	let config_path = config_path.to_owned();
	let colors = colors.to_owned();
	let output_dmi_path = output_dmi_path.to_owned();
	let result = Some(match catch_panic(|| gags::gags(&config_path, &colors, &output_dmi_path)) {
		Ok(o) => match o {
			Ok(o) => o,
			Err(e) => e.to_string()
		},
		Err(e) => e.to_string()
	});
	frame!();
	result
});

byond_fn!(fn iconforge_gags_async(config_path, colors, output_dmi_path) {
	let config_path = config_path.to_owned();
	let colors = colors.to_owned();
	let output_dmi_path = output_dmi_path.to_owned();
	Some(jobs::start(move || {
		let result = match catch_panic(|| gags::gags(&config_path, &colors, &output_dmi_path)) {
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
