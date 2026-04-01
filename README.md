# iconforge

A library for "forging" spritesheets using [BYOND](https://www.byond.com/)'s [DMI](https://github.com/spacestation13/dmi-rust/) format at the speed of light.

This project was originally part of [rust-g](https://github.com/tgstation/rust-g), and uses some code from contributors of that project, however, IconForge is my original work. See [LICENSE_RUSTG](LICENSE_RUSTG).

## Building

Install [Rust](https://rustup.rs/) via rustup.

Create a release build:

```sh
cargo build --features spritesheet,gags,ffi --release --target i686-pc-windows-msvc
```

```sh
cargo build --features spritesheet,gags,ffi --release --target i686-unknown-linux-gnu
```

### Build Optimization

IconForge can run up to 10-20% faster when compiled with optimizations for your system, at the cost of compatibility with other/older systems:

```batch
set RUSTFLAGS="-C target-cpu=native"
cargo build --release --target i686-pc-windows-msvc
```

```sh
RUSTFLAGS="-C target-cpu=native"
cargo build --release --target i686-unknown-linux-gnu
```

Public release binaries of IconForge do *not* use native CPU optimizations, so building it yourself (on your server) is a great way to bump server performance.

## Installation

To install IconForge to a BYOND repository, copy `target/iconforge_rs.dm` into an early stage of your .dme, so that it can create `#define`s for later files. Place the remaining files wherever makes sense for your repository.

`iconforge_rs.dm` function calls will search for a corresponding native library (`.dll` or `.so`) in the following locations:

- In the repository / game root of your BYOND application.
- In `$HOME/.byond/bin/` (only on Linux)

It expects the filename of the native library to be any of the following:

- Windows: `iconforge_rs.dll` (or `iconforge_rs64.dll`, for 64-bit BYOND implementations such as [OpenDream](https://github.com/OpenDreamProject/OpenDream))
- Linux: `libiconforge_rs.so` (or `libiconforge_rs64.so`)
- Linux (Legacy, do not use): `iconforge_rs` (or `iconforge_rs64`)

## Features

### DM API

IconForge includes a DM API, which offers easy functions for direct interaction with the DLL. This includes the Universal Icon system, which is a managed DM library for imitating native icon procs. Simply replace `/icon` with `/datum/universal_icon` and change operations to match the alternate syntax (e.g. `MapColors` to `map_colors`), then you can use it with any IconForge generator, or even use the native generation via`to_icon()`

#### Inline Example

```c
// Define an icon and its transformations without generating it
var/datum/universal_icon/my_universal_icon = uni_icon('icons/my_icon.dmi', "my_icon_state")
my_universal_icon.scale(64, 64)

// Generate & assign icon to an atom
my_atom.icon = my_universal_icon.to_icon_headless("tmp/my_generated_icon.dmi", "my_spritesheet_icon_state")
my_atom.icon_state = "my_spritesheet_icon_state"
```

#### Headless Example

The inline function `to_icon_headless` performs this procedure, with additional error handling:

```c
// Create a mapping of icon_state names -> list-encoded universal icons
var/list/my_spritesheet_icons = list()

// Define an icon without generating it
var/datum/universal_icon/my_universal_icon = uni_icon('icons/my_icon.dmi', "my_icon_state")
my_universal_icon.scale(64, 64)

// Convert it to a list and map it to an output icon state name
my_spritesheet_icons["my_spritesheet_icon_state"] = my_universal_icon.to_list()

// Generate the mapped icons, encoding the mapping as JSON
iconforge_generate_headless("tmp/my_generated_icon.dmi", json_encode(my_spritesheet_icons), FALSE)

// Given that the previous generation was a success (some error checking would be a good idea, see to_icon_headless),
// have BYOND load it as an icon and assign it to the atom
my_atom.icon = icon(file("tmp/my_generated_icon.dmi"))
my_atom.icon_state = "my_spritesheet_icon_state"
```

#### Batched Multi-size or Async Generation

If you have a large sheet with multiple sizes of icons and want to efficiently handle them, `iconforge_generate` and `iconforge_generate_async` provide the tools to generate them with efficient caching.

The output from `hash_icons` can be used for `iconforge_cache_valid` to validate if a given `sprites` list will create the same result as a previous one by using the hash of all of the input files and the hash of the `sprites` transformations input.

Take a look at the DM functions for documentation on how to use them properly.


## Comparison to native operations

When compared to native BYOND icon manipulation procs, IconForge offers better speed and error handling. BYOND has a habit of silently eating weird operations, and is exceedingly slow - single icon operations can take up to 50ms each. This is unacceptable for any server-generated icons, where maintaining a playable tick rate is paramount.

IconForge speeds these operations up by a factor of 1000, some icon operations are in the microsecond range, with I/O being a primary limiter on speed. This is why IconForge takes a massively parallel, batched approach. Instead of generating one-off icons, several hundred are batched into one operation which can then be executed on multiple threads, collected, and *cached efficiently*, using IconForge's built-in cache validator.

IconForge also offers near-full parity on most native BYOND operations, ensuring that icons look the same or similar to those generated by the engine.

### Operation Parity Table

Parity is covered by tests and checked against BYOND's output as part of CI. This is a table of the standard to which they are tested against.

| Feature                    | Transform Define        | Parity Status      | Notes                                                            |
|----------------------------|-------------------------|--------------------|------------------------------------------------------------------|
| **BYOND procs**            |                         |                    |                                                                  |
| `Scale()`                  | `ICONFORGE_SCALE`       | ⚠️ Partial Parity  | Full parity for some types; excluded for odd sizes (e.g., 8x19). |
| `Crop()`                   | `ICONFORGE_CROP`        | ✅ Full Parity     | Extensively tested for expansion and shrinking.                  |
| `Blend()` (Icon)           | `ICONFORGE_BLEND_ICON`  | ✅ Full Parity     | Supports x/y offsets. See Blend Modes below.                     |
| `Blend()` (Color)          | `ICONFORGE_BLEND_COLOR` | ✅ Full Parity     | See Blend Modes below.                                           |
| `MapColors()`              | `ICONFORGE_MAP_COLORS`  | ✅ Full Parity     | Supports RGB, RGBA (Hex/Num), and Inversion.                     |
| `Flip()`                   | `ICONFORGE_FLIP`        | ✅ Full Parity     | Tested for all 8 cardinal/ordinal directions.                    |
| `Turn()`                   | `ICONFORGE_TURN`        | ⚠️ Partial Parity  | Parity for 90° increments only.                                  |
| `Shift()`                  | `ICONFORGE_SHIFT`       | ✅ Full Parity     | Supports wrapping, no-wrap, and overflows.                       |
| `SwapColor()`              | `ICONFORGE_SWAP_COLOR`  | ✅ Full Parity     | Handles various alpha/hex combinations.                          |
| `DrawBox()`                | `ICONFORGE_DRAW_BOX`    | ✅ Full Parity     | Supports coordinate ranges and alpha.                            |
| **BYOND Icon Blend Modes** |                         |                    |                                                                  |
| `ICON_ADD`                 | —                       | ✅ Full Parity     | Within +/- 1 bit per pixel.                                      |
| `ICON_SUBTRACT`            | —                       | ✅ Full Parity     | Within +/- 1 bit per pixel.                                      |
| `ICON_MULTIPLY`            | —                       | ✅ Full Parity     | Within +/- 1 bit per pixel.                                      |
| `ICON_OVERLAY`             | —                       | ✅ Full Parity     | Within +/- 1 bit per pixel.                                      |
| `ICON_AND`                 | —                       | ✅ Full Parity     | Within +/- 1 bit per pixel.                                      |
| `ICON_OR`                  | —                       | ✅ Full Parity     | Within +/- 1 bit per pixel.                                      |
| `ICON_UNDERLAY`            | —                       | ✅ Full Parity     | Within +/- 1 bit per pixel.                                      |
| **Other Features**         |                         |                    |                                                                  |
| GAGS API                   | —                       | ✅ Full Parity     | Supported via GAGS ColorMatrix layer type.                       |
| Mixed-Dir Blending         | —                       | ⚠️ Partial Parity  | 4/8 onto single dir or single dir onto 4/8 only.                 |
| Mixed-Frame Blending       | —                       | ⚠️ Partial Parity  | Multi-frame icons onto single frame only.                        |
| Movement States            | —                       | ❌ Not Implemented |                                                                  |
| Greyscale/Non-RGBA         | —                       | ❌ Not Implemented | IconForge will only output RGBA icons.                           |

### Notable Differences

#### Missing Icon States

BYOND will generally silently eat nonexistent icon states when blending icons, using the `""` (empty string) icon state as a default if it exists. IconForge will instead return an error, providing generally improved developer UX, as this is not typically wanted behavior and there is very little visibility into it.

#### Dir Mixing

IconForge does not allow specific blend operations between icon states that have a different number of directions. There is special handling for when an icon has only *one* direction and is blended onto one with multiple - this behavior will match BYOND as it is used occasionally and makes sense to the user.

There is a similar handling for blending a multiple-dir icon onto a single-dir icon, it will always use `SOUTH` from the upper icon, as this is also generally expected.

However, blending an icon with `NORTH`/`EAST`/`SOUTH`/`WEST` onto an icon `NORTH`/`EAST`/`NORTHEAST`/etc. is typically unwanted behavior, and IconForge will error in this case. The `universal_icon` API accepts `dir` restrictions, which can define a single dir to be blended instead.

#### Frame Mixing

The same difference exists for icon states with different numbers of frames - there is special handling for blending an icon with *multiple* frames onto one with only *one* frame (by adding copies of the first frame), because this is expected (and used) behavior to animate an icon with an overlay. IconForge will add new `delays` entries with a value of 1, the same as BYOND.

#### Crop & DrawBox Coordinate Ordering

If a crop or DrawBox operation places a larger bound before a smaller bound, IconForge will return an error. This is just a matter of ordering the coordinates by size (e.g. `(x1=64, y1=32, x2=32, y2=64)` should be `(x1=32, y1=32, x2=64, y2=64)`) as it is clearer to the developer and the result is identical.

#### Diagonal (non-cardinal) Flip() on a non-square icon

Did you know you can `Flip(NORTHEAST)`? Now you do! In BYOND, performing a flip on an icon that is not square provides unpredictable behavior. IconForge will error. This is because `Flip(NORTHEAST)` both flips AND rotates the image, and rotating an image of a non-square size will change its dimensions. This will cause the output state to have a different dimension than the rest of the images, which is generally a problem outside of special spritesheet handling.

#### Invalid colors

Providing a non-hex color to BYOND generally causes internal runtimes or the resulting icon to use white instead. IconForge will return decode errors for hex strings that are invalid.
