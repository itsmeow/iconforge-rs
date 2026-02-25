// #![forbid(unsafe_op_in_unsafe_fn)] - see
// github.com/rust-lang/rust/issues/121483
#![cfg_attr(
	all(windows, not(all(target_vendor = "pc", target_env = "msvc"))),
	allow(clippy::missing_const_for_thread_local)
)] // see https://github.com/rust-lang/rust-clippy/issues/13422

#[macro_use]
mod byond;
mod error;
mod jobs;

pub mod iconforge;
