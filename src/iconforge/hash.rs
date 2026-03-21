use crate::error::Error;
use std::{cell::RefCell, fs::File, hash::Hasher, io::Read};
use twox_hash::XxHash64;

const BUFFER_SIZE: usize = 65536;
// don't allocate another buffer every time we hash a file, just reuse the same
// buffer.
thread_local!( static FILE_HASH_BUFFER: RefCell<[u8; BUFFER_SIZE]> = const { RefCell::new([0; BUFFER_SIZE]) } );

/// This seed is just a random number that should stay the same between builds
/// and runs
const CONSISTENT_XXHASH_SEED: u64 = 17479268743136991876;

pub fn fixed_twox_file(path: &str) -> Result<String, Error> {
	let mut hasher = XxHash64::with_seed(CONSISTENT_XXHASH_SEED);
	let mut file = File::open(path)?;
	FILE_HASH_BUFFER.with_borrow_mut(|buffer| {
		loop {
			let bytes_read = file.read(buffer)?;
			if bytes_read == 0 {
				break;
			}
			hasher.write(&buffer[..bytes_read]);
		}
		Ok(format!("{:x}", hasher.finish()))
	})
}

pub fn fixed_twox_string<B: AsRef<[u8]>>(bytes: B) -> String {
	let mut hasher = XxHash64::with_seed(CONSISTENT_XXHASH_SEED);
	hasher.write(bytes.as_ref());
	format!("{:x}", hasher.finish())
}
