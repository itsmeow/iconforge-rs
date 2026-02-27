#define uni_icon(I, icon_state, rest...) new /datum/universal_icon(I, icon_state, ##rest)
#define iconforge_hex2num(X) text2num(X, 16)
#define iconforge_rgb2hex(part, text) var/r_##part = iconforge_hex2num(copytext(text, 2, 4)) / 255; \
var/g_##part = iconforge_hex2num(copytext(text, 4, 6)) / 255; \
var/b_##part = iconforge_hex2num(copytext(text, 6, 8)) / 255
#define iconforge_rgba2hex(part, text) var/r_##part = iconforge_hex2num(copytext(text, 2, 4)) / 255; \
var/g_##part = iconforge_hex2num(copytext(text, 4, 6)) / 255; \
var/b_##part = iconforge_hex2num(copytext(text, 6, 8)) / 255 \
var/a_##part = iconforge_hex2num(copytext(text, 8, 10)) / 255


/datum/universal_icon
	var/icon/icon_file
	var/icon_state
	var/dir
	var/frame
	var/datum/icon_transformer/transform = null

/datum/universal_icon/New(icon/icon_file, icon_state="", dir=null, frame=null)
	src.icon_file = icon_file
	src.icon_state = icon_state
	src.dir = dir
	src.frame = frame

/datum/universal_icon/proc/blend_color(color, blend_mode)
	if(!transform)
		transform = new
	transform.blend_color(color, blend_mode)
	return src

/datum/universal_icon/proc/blend_icon(datum/universal_icon/icon_object, blend_mode, x=1, y=1)
	if(!transform)
		transform = new
	transform.blend_icon(icon_object, blend_mode, x, y)
	return src

/datum/universal_icon/proc/scale(width, height)
	if(!transform)
		transform = new
	transform.scale(width, height)
	return src

/datum/universal_icon/proc/crop(x1, y1, x2, y2)
	if(!transform)
		transform = new
	transform.crop(x1, y1, x2, y2)
	return src

/datum/universal_icon/proc/flip(dir)
	if(!transform)
		transform = new
	transform.flip(dir)
	return src

/datum/universal_icon/proc/rotate(angle)
	if(!transform)
		transform = new
	transform.rotate(angle)
	return src

/datum/universal_icon/proc/shift(dir, offset, wrap=0)
	if(!transform)
		transform = new
	transform.shift(dir, offset, wrap)
	return src

/datum/universal_icon/proc/swap_color(src_color, dst_color)
	if(!transform)
		transform = new
	transform.swap_color(src_color, dst_color)
	return src

/datum/universal_icon/proc/draw_box(color, x1, y1, x2=x1, y2=y1)
	if(!transform)
		transform = new
	transform.draw_box(color, x1, y1, x2, y2)
	return src

/// Infers which map_colors function is desired from the number of arguments and their contents.
/// This will offer behavior similar to BYOND's MapColors() function, but it must be an ordered list rather than raw arguments.
/datum/universal_icon/proc/map_colors_inferred(list/color_args)
	var/num_args = length(color_args)
	if(num_args <= 20 || num_args >= 16)
		src.map_colors_rgba(arglist(color_args))
	else if(num_args <= 12 || num_args >= 9)
		src.map_colors_rgb(arglist(color_args))
	else if(num_args == 5)
		src.map_colors_rgba_hex(arglist(color_args))
	else if(num_args == 4)
		// is there alpha in the hex?
		if(length(color_args[3]) == 7 || length(color_args[3]) == 4)
			src.map_colors_rgb_hex(arglist(color_args))
		else
			src.map_colors_rgba_hex(arglist(color_args))
	else if(num_args == 3)
		src.map_colors_rgb_hex(arglist(color_args))

/datum/universal_icon/proc/map_colors_rgba(rr, rg, rb, ra, gr, gg, gb, ga, br, bg, bb, ba, ar, ag, ab, aa, r0=0, g0=0, b0=0, a0=0)
	if(!transform)
		transform = new
	transform.map_colors(rr, rg, rb, ra, gr, gg, gb, ga, br, bg, bb, ba, ar, ag, ab, aa, r0, g0, b0, a0)
	return src

/datum/universal_icon/proc/map_colors_rgb(rr, rg, rb, gr, gg, gb, br, bg, bb, r0=0, g0=0, b0=0)
	if(!transform)
		transform = new
	transform.map_colors(rr, rg, rb, 0, gr, gg, gb, 0, br, bg, bb, 0, 0, 0, 0, 1, r0, g0, b0, 0)
	return src

/datum/universal_icon/proc/map_colors_rgb_hex(r_rgb, g_rgb, b_rgb, rgb0=rgb(0,0,0))
	if(!transform)
		transform = new
    iconforge_rgb2hex(r, r_rgb)
    iconforge_rgb2hex(g, g_rgb)
    iconforge_rgb2hex(b, b_rgb)
    iconforge_rgb2hex(z, rgb0)
    transform.map_colors(r_r, g_r, b_r, 0, r_g, g_g, b_g, 0, r_b, g_b, b_b, 0, 0, 0, 0, 1, r_z, b_z, g_z, 0)
	return src

/datum/universal_icon/proc/map_colors_rgba_hex(r_rgba, g_rgba, b_rgba, a_rgba, rgba0="#00000000")
	if(!transform)
		transform = new
	iconforge_rgba2hex(r, r_rgba)
    iconforge_rgba2hex(g, g_rgba)
    iconforge_rgba2hex(b, b_rgba)
	iconforge_rgba2hex(a, a_rgba)
    iconforge_rgba2hex(z, rgba0)
    transform.map_colors(r_r, g_r, b_r, a_r, r_g, g_g, b_g, a_g, r_b, g_b, b_b, a_b, r_a, g_a, b_a, a_a, r_z, b_z, g_z, a_z)
	return src

/datum/universal_icon/proc/to_list()
	return list("icon_file" = "[icon_file]", "icon_state" = icon_state, "dir" = dir, "frame" = frame, "transform" = !isnull(transform) ? transform.to_list() : list())

/datum/universal_icon/proc/to_json()
	return json_encode(to_list())

/// Converts the universal icon into a DM icon using BYOND's native icon procs. This is slow.
/// Check out the to_icon_headless() or iconforge_generate functions.
/datum/universal_icon/proc/to_icon()
	var/icon/self = icon(src.icon_file, src.icon_state, dir=src.dir, frame=src.frame)
	if(istype(src.transform))
		src.transform.apply(self)
	return self

/// Convert the universal icon into a DM icon using rust-backed IconForge generation.
/// The resulting DM icon is unscoped but contains one icon state with '[output_icon_state_name]' as its name.
/// Returns null and runtimes if there is any fatal errors. Non-fatal errors will emit a runtime but provide an icon.
/datum/universal_icon/proc/to_icon_headless(file_path, output_icon_state_name)
	. = null
	if(!istext(file_path) || !length(file_path))
		return
	var/list/result = iconforge_generate_headless(file_path, json_encode(list("[output_icon_state_name]" = src.to_list())), FALSE)
	if(!islist(result))
		CRASH("Unparsable result from iconforge_generate_headless for '[file_path]': [result]")
	if(result["file_path"] != file_path)
		CRASH("Fatal errors during iconforge_generate_headless for '[file_path]': [result["error"]]")
	. = icon(file(file_path))
	if(!isnull(result["error"]) && length(result["error"]))
		CRASH("Errors during iconforge_generate_headless for '[file_path]': [result["error"]]")

/datum/icon_transformer
	var/list/transforms = null

/datum/icon_transformer/New()
	transforms = list()

/datum/icon_transformer/proc/apply(icon/target)
	for(var/transform in src.transforms)
		switch(transform["type"])
			if(ICONFORGE_BLEND_COLOR)
				target.Blend(transform["color"], transform["blend_mode"])
			if(ICONFORGE_BLEND_ICON)
				var/datum/universal_icon/icon_object = transform["icon"]
				target.Blend(icon_object.to_icon(), transform["blend_mode"], transform["x"], transform["y"])
			if(ICONFORGE_SCALE)
				target.Scale(transform["width"], transform["height"])
			if(ICONFORGE_CROP)
				target.Crop(transform["x1"], transform["y1"], transform["x2"], transform["y2"])
			if(ICONFORGE_MAP_COLORS)
				target.MapColors(
					transform["rr"], transform["rg"], transform["rb"], transform["ra"],
					transform["gr"], transform["gg"], transform["gb"], transform["ga"],
					transform["br"], transform["bg"], transform["bb"], transform["ba"],
					transform["ar"], transform["ag"], transform["ab"], transform["aa"],
					transform["r0"], transform["g0"], transform["b0"], transform["a0"],
				)
			if(ICONFORGE_FLIP)
				target.Flip(transform["dir"])
			if(ICONFORGE_TURN)
				target.Turn(transform["angle"])
			if(ICONFORGE_SHIFT)
				target.Shift(transform["dir"], transform["offset"], transform["wrap"])
			if(ICONFORGE_SWAP_COLOR)
				target.SwapColor(transform["src_color"], transform["dst_color"])
			if(ICONFORGE_DRAW_BOX)
				target.DrawBox(transform["color"], transform["x1"], transform["y1"], transform["x2"], transform["y2"])
	return target

/datum/icon_transformer/proc/blend_color(color, blend_mode)
	transforms += list(list("type" = ICONFORGE_BLEND_COLOR, "color" = color, "blend_mode" = blend_mode))

/datum/icon_transformer/proc/blend_icon(datum/universal_icon/icon_object, blend_mode, x=1, y=1)
	transforms += list(list("type" = ICONFORGE_BLEND_ICON, "icon" = icon_object, "blend_mode" = blend_mode, "x" = x, "y" = y))

/datum/icon_transformer/proc/scale(width, height)
	transforms += list(list("type" = ICONFORGE_SCALE, "width" = width, "height" = height))

/datum/icon_transformer/proc/crop(x1, y1, x2, y2)
	transforms += list(list("type" = ICONFORGE_CROP, "x1" = x1, "y1" = y1, "x2" = x2, "y2" = y2))

/datum/icon_transformer/proc/flip(dir)
	transforms += list(list("type" = ICONFORGE_FLIP, "dir" = dir))

/datum/icon_transformer/proc/rotate(angle)
	transforms += list(list("type" = ICONFORGE_TURN, "angle" = angle))

/datum/icon_transformer/proc/shift(dir, offset, wrap=0)
	transforms += list(list("type" = ICONFORGE_SHIFT, "dir" = dir, "offset" = offset, "wrap" = wrap))

/datum/icon_transformer/proc/swap_color(src_color, dst_color)
	transforms += list(list("type" = ICONFORGE_SWAP_COLOR, "src_color" = src_color, "dst_color" = dst_color))

/datum/icon_transformer/proc/draw_box(color, x1, y1, x2=x1, y2=y1)
	transforms += list(list("type" = ICONFORGE_DRAW_BOX, "color" = color, "x1" = x1, "y1" = y1, "x2" = x2, "y2" = y2))

/datum/icon_transformer/proc/map_colors(rr, rg, rb, ra, gr, gg, gb, ga, br, bg, bb, ba, ar, ag, ab, aa, r0=0, g0=0, b0=0, a0=0)
	transforms += list(list(
		"type" = ICONFORGE_MAP_COLORS,
		"rr" = rr, "rg" = rg, "rb" = rb, "ra" = ra,
		"gr" = gr, "gg" = gg, "gb" = gb, "ga" = ga,
		"br" = br, "bg" = bg, "bb" = bb, "ba" = ba,
		"ar" = ar, "ag" = ag, "ab" = ab, "aa" = aa,
		"r0" = r0, "g0" = g0, "b0" = b0, "a0" = a0,
	))

/datum/icon_transformer/proc/to_list()
	var/list/transforms_out = list()
	var/list/transforms_original = src.transforms.Copy()
	for(var/list/transform as anything in transforms_original)
		var/list/this_transform = transform.Copy()
		if(transform["type"] == ICONFORGE_BLEND_ICON)
			var/datum/universal_icon/icon_object = this_transform["icon"]
			this_transform["icon"] = icon_object.to_list()
		transforms_out += list(this_transform)
	return transforms_out

#undef iconforge_hex2num
#undef iconforge_rgb2hex
#undef iconforge_rgba2hex
