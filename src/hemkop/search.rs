use super::HOST;
use dioxus::logger::tracing;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HemkopSearchResponse {
	#[serde(rename = "autocompleteResultData")]
	pub autocomplete_result_data: AutocompleteResultData,
	#[serde(rename = "productSearchPageData")]
	pub product_search_page_data: ProductSearchPageData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AutocompleteResultData {
	pub suggestions: Vec<Suggestion>,
	pub products: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Suggestion {
	pub term: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductSearchPageData {
	pub results: Vec<Product>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
	#[serde(rename = "priceValue")]
	pub price_value: Option<f64>,
	pub price: Option<String>,
	pub image: Option<ProductImage>,
	#[serde(rename = "depositPrice")]
	pub deposit_price: Option<String>,
	#[serde(rename = "comparePrice")]
	pub compare_price: Option<String>,
	#[serde(rename = "comparePriceUnit")]
	pub compare_price_unit: Option<String>,
	pub manufacturer: Option<String>,
	pub name: Option<String>,
	pub code: Option<String>,
	#[serde(rename = "displayVolume")]
	pub display_volume: Option<String>,
	#[serde(rename = "outOfStock")]
	pub out_of_stock: Option<bool>,
	pub online: Option<bool>,
	pub thumbnail: Option<ProductImage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductImage {
	pub url: String,
}

pub async fn search(
	client: &Client,
	query: &str,
	page: usize,
	size: usize,
) -> anyhow::Result<HemkopSearchResponse> {
	let mut url = Url::parse(&format!("https://{}/search/multisearchAsutype", HOST)).unwrap();
	url.query_pairs_mut()
		.append_pair("q", query)
		.append_pair("page", &page.to_string())
		.append_pair("size", &size.to_string());

	let req = client.get(url).header("accept", "application/json");

	let res = req.send().await?.error_for_status()?.text().await?;

	tracing::info!("{res:?}");

	Ok(serde_json::from_str(&res)?)
}
