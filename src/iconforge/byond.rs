use super::image_cache;
use crate::jobs;

byond_fn!(fn iconforge_check(id) {
	Some(jobs::check(id))
});

byond_fn!(
	fn iconforge_cleanup() {
		// Only perform cleanup if no jobs are currently using the icon cache
		if image_cache::CACHE_ACTIVE.load(std::sync::atomic::Ordering::SeqCst) > 0 {
			return Some("Skipped, cache in use");
		}

		image_cache::icon_cache_clear();
		image_cache::image_cache_clear();
		Some("Ok")
	}
);
