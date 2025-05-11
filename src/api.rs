#[cfg(feature = "server")]
use crate::hemkop;
use dioxus::prelude::*;

#[cfg(feature = "server")]
mod utils {
	pub fn client() -> Result<reqwest::Client, reqwest::Error> {
		let mut headers = reqwest::header::HeaderMap::new();
		headers.insert(
			"accept-encoding",
			reqwest::header::HeaderValue::from_static("gzip, deflate, br, zstd"),
		);
		reqwest::ClientBuilder::new()
			.https_only(true)
			.user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.3")
			.default_headers(headers)
			.brotli(true)
			.zstd(true)
			.gzip(true)
			.deflate(true)
			.build()
	}
}

#[server]
pub async fn search(query: String) -> Result<Option<f64>, ServerFnError> {
	let client = utils::client()?;
	let result = match hemkop::search::search(&client, &query, 0, 1).await {
		Ok(r) => r,
		Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
	};

	Ok(result
		.product_search_page_data
		.results
		.get(0)
		.and_then(|p| p.price_value))
}
