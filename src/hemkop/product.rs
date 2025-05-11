use super::{ProductCode, HOST};
use anyhow::Result;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OtherVariantsResponse {
	pub page: usize,
	#[serde(rename = "pageSize")]
	pub page_size: usize,
	#[serde(rename = "totalPages")]
	pub total_pages: usize,
	pub links: serde_json::Value,
	pub items: Vec<OtherVariantProduct>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OtherVariantProduct {
	#[serde(rename = "potentialPromotions")]
	pub potential_promotions: Vec<serde_json::Value>,
	#[serde(rename = "inactivePotentialPromotions")]
	pub inactive_potential_promotions: Vec<serde_json::Value>,
	pub breadcrumbs: Vec<Breadcrumb>,
	pub labels: Vec<serde_json::Value>,
	pub ean: String,
	#[serde(rename = "variantType")]
	pub variant_type: String,
	#[serde(rename = "fruitAndVegetableVariety")]
	pub fruit_and_vegetable_variety: String,
	#[serde(rename = "otherCountries")]
	pub other_countries: Vec<serde_json::Value>,
	pub image: Option<ProductImage>,
	#[serde(rename = "originCountry")]
	pub origin_country: String,
	#[serde(rename = "extraInfoCode")]
	pub extra_info_code: String,
	#[serde(rename = "categoryGroupText")]
	pub category_group_text: Option<String>,
	#[serde(rename = "activeSubstance")]
	pub active_substance: String,
	#[serde(rename = "medicineType")]
	pub medicine_type: String,
	#[serde(rename = "medicineLink")]
	pub medicine_link: String,
	#[serde(rename = "isDrugProduct")]
	pub is_drug_product: bool,
	#[serde(rename = "saleStop")]
	pub sale_stop: Option<String>,
	pub ingredients: String,
	#[serde(rename = "tradeItemCountryOfOrigin")]
	pub trade_item_country_of_origin: String,
	#[serde(rename = "nutritionFacts")]
	pub nutrition_facts: String,
	#[serde(rename = "salableOnline")]
	pub salable_online: bool,
	#[serde(rename = "consumerStorageInstructions")]
	pub consumer_storage_instructions: String,
	pub instructions: String,
	#[serde(rename = "countryOfOriginStatement")]
	pub country_of_origin_statement: String,
	#[serde(rename = "preparationType")]
	pub preparation_type: String,
	#[serde(rename = "preparationDescription")]
	pub preparation_description: String,
	#[serde(rename = "regulatoryAct")]
	pub regulatory_act: String,
	#[serde(rename = "compulsoryAdditiveLabelInformation")]
	pub compulsory_additive_label_information: String,
	#[serde(rename = "servingSize")]
	pub serving_size: Option<f64>,
	#[serde(rename = "nutritionalClaim")]
	pub nutritional_claim: String,
	#[serde(rename = "healthClaimDescription")]
	pub health_claim_description: String,
	#[serde(rename = "animalData")]
	pub animal_data: Option<serde_json::Value>,
	#[serde(rename = "nutrientHeaders")]
	pub nutrient_headers: Vec<serde_json::Value>,
	#[serde(rename = "dietTypeInformation")]
	pub diet_type_information: String,
	#[serde(rename = "batteryInformation")]
	pub battery_information: Vec<serde_json::Value>,
	#[serde(rename = "nutritionDescription")]
	pub nutrition_description: String,
	#[serde(rename = "consumerUsageInstructions")]
	pub consumer_usage_instructions: String,
	#[serde(rename = "areBatteriesIncluded")]
	pub are_batteries_included: bool,
	#[serde(rename = "areBatteriesRequired")]
	pub are_batteries_required: bool,
	#[serde(rename = "provinceStatement")]
	pub province_statement: String,
	#[serde(rename = "googleAnalyticsCategory")]
	pub google_analytics_category: String,
	#[serde(rename = "nutrientComponents")]
	pub nutrient_components: Vec<serde_json::Value>,
	#[serde(rename = "foodContactName")]
	pub food_contact_name: String,
	#[serde(rename = "foodContactAddress")]
	pub food_contact_address: String,
	#[serde(rename = "nutritionsFactList")]
	pub nutritions_fact_list: Vec<serde_json::Value>,
	#[serde(rename = "extendedLabels")]
	pub extended_labels: Vec<serde_json::Value>,
	#[serde(rename = "minStorageTemperature")]
	pub min_storage_temperature: String,
	#[serde(rename = "maxStorageTemperature")]
	pub max_storage_temperature: String,
	#[serde(rename = "breadCrumbs")]
	pub bread_crumbs: Vec<Breadcrumb>,
	pub description: String,
	#[serde(rename = "priceValue")]
	pub price_value: Option<f64>,
	pub price: Option<String>,
	pub ranking: Option<f64>,
	#[serde(rename = "depositPrice")]
	pub deposit_price: Option<String>,
	#[serde(rename = "averageWeight")]
	pub average_weight: Option<f64>,
	#[serde(rename = "comparePrice")]
	pub compare_price: Option<String>,
	#[serde(rename = "comparePriceUnit")]
	pub compare_price_unit: Option<String>,
	#[serde(rename = "solrSearchScore")]
	pub solr_search_score: Option<f64>,
	#[serde(rename = "energyDeclaration")]
	pub energy_declaration: Option<serde_json::Value>,
	#[serde(rename = "newsSplashProduct")]
	pub news_splash_product: bool,
	#[serde(rename = "notAllowedB2b")]
	pub not_allowed_b2b: bool,
	#[serde(rename = "notAllowedAnonymous")]
	pub not_allowed_anonymous: bool,
	#[serde(rename = "productLine2")]
	pub product_line2: Option<String>,
	#[serde(rename = "pickupProductLine2")]
	pub pickup_product_line2: Option<String>,
	#[serde(rename = "priceUnit")]
	pub price_unit: Option<String>,
	#[serde(rename = "priceNoUnit")]
	pub price_no_unit: Option<String>,
	#[serde(rename = "savingsAmount")]
	pub savings_amount: Option<f64>,
	#[serde(rename = "gdprTrackingIncompliant")]
	pub gdpr_tracking_incompliant: bool,
	#[serde(rename = "showGoodPriceIcon")]
	pub show_good_price_icon: bool,
	#[serde(rename = "nicotineMedicalProduct")]
	pub nicotine_medical_product: bool,
	#[serde(rename = "tobaccoFreeNicotineProduct")]
	pub tobacco_free_nicotine_product: bool,
	#[serde(rename = "nonEkoProduct")]
	pub non_eko_product: bool,
	#[serde(rename = "precautionaryStatements")]
	pub precautionary_statements: Vec<serde_json::Value>,
	pub hazards: Vec<serde_json::Value>,
	#[serde(rename = "tobaccoProduct")]
	pub tobacco_product: bool,
	#[serde(rename = "seoDescription")]
	pub seo_description: String,
	#[serde(rename = "seoTitle")]
	pub seo_title: String,
	pub manufacturer: Option<String>,
	#[serde(rename = "incrementValue")]
	pub increment_value: Option<f64>,
	#[serde(rename = "productBasketType")]
	pub product_basket_type: Option<ProductBasketType>,
	#[serde(rename = "bargainProduct")]
	pub bargain_product: bool,
	#[serde(rename = "addToCartMessage")]
	pub add_to_cart_message: String,
	pub thumbnail: Option<ProductImage>,
	#[serde(rename = "outOfStock")]
	pub out_of_stock: bool,
	pub online: bool,
	#[serde(rename = "displayVolume")]
	pub display_volume: Option<String>,
	#[serde(rename = "addToCartDisabled")]
	pub add_to_cart_disabled: bool,
	pub name: Option<String>,
	pub code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Breadcrumb {
	pub url: Option<String>,
	pub name: Option<String>,
	#[serde(rename = "linkClass")]
	pub link_class: Option<String>,
	#[serde(rename = "categoryCode")]
	pub category_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductImage {
	#[serde(rename = "imageType")]
	pub image_type: Option<String>,
	pub format: Option<String>,
	pub url: Option<String>,
	#[serde(rename = "altText")]
	pub alt_text: Option<String>,
	#[serde(rename = "galleryIndex")]
	pub gallery_index: Option<usize>,
	pub width: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductBasketType {
	pub code: Option<String>,
	pub r#type: Option<String>,
}

pub async fn other_variants(
	client: &Client,
	product_code: &ProductCode,
) -> Result<OtherVariantsResponse> {
	let url = Url::parse(&format!(
		"https://{}/axfood/rest/products/{}/other-variants",
		HOST, product_code
	))
	.unwrap();

	let req = client.get(url).header("accept", "application/json");

	let res = req.send().await?.error_for_status()?.text().await?;

	Ok(serde_json::from_str(&res)?)
}

pub type ProductResponse = OtherVariantProduct;

pub async fn product(client: &Client, product_code: &ProductCode) -> Result<ProductResponse> {
	let url = Url::parse(&format!("https://{}/axfood/rest/p/{}", HOST, product_code)).unwrap();

	let req = client.get(url).header("accept", "application/json");

	let res = req.send().await?.error_for_status()?.text().await?;

	Ok(serde_json::from_str(&res)?)
}
