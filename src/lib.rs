pub mod client;
pub mod config;
pub mod duplicates;
pub mod instance_info;
pub mod oauth;
pub mod oauth_resources;
pub mod post;
pub mod redgifs;
pub mod search;
pub mod server;
pub mod settings;
pub mod subreddit;
pub mod user;
pub mod utils;

#[cfg(test)]
#[doc(hidden)]
pub fn live_tests_enabled() -> bool {
	std::env::var("REDLIB_LIVE_TESTS").is_ok()
}
