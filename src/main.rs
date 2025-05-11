#![allow(non_camel_case_types)]

#[allow(unused)]
use dioxus::logger::tracing;
use dioxus::prelude::*;

mod api;
pub mod components;
// #[cfg(feature = "server")]
pub mod hemkop;
use crate::components::background_dots::BackgroundDots;
use crate::components::category::{CategoryContainer, ItemCategory};
use crate::components::kiosk_item::{KioskItem, PriceTrend};
use crate::components::navbar::Navbar;

#[cfg(feature = "web")]
mod window_events;

static CSS: Asset = asset!("resources/styles.css");
static LQIP_CSS: Asset = asset!("resources/lqip.css");
static NORMALIZE: Asset = asset!("resources/normalize.css");
static GFONTS: &str = "https://fonts.googleapis.com/css2?family=Dela+Gothic+One&family=Montserrat:ital,wght@0,100..900;1,100..900&family=Spline+Sans:wght@300..700&display=swap";

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
	#[route("/")]
	HomePage {},
	#[route("/buy")]
	BuyPage {},
	#[route("/sell")]
	SellPage {},
	#[route("/login")]
	LoginPage {},
}

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	use dioxus::prelude::*;

	rsx! {
		document::Stylesheet { href: NORMALIZE }
		document::Stylesheet { href: CSS }
		document::Stylesheet { href: LQIP_CSS }
		link { rel: "preconnect", href: "https://fonts.googleapis.com" }
		link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: true }
		link {
			href: GFONTS,
			rel: "stylesheet"
		}
		Router::<Route> {}
	}
}

#[component]
fn PageContainer(children: Element) -> Element {
	rsx! {
		div {

			width: "calc(100vw - 80px)",
			max_width: "1400px",
			margin: "40px auto",
			{ children }
		}
	}
}

#[component]
fn LoginPage() -> Element {
	rsx! {
		BackgroundDots {}
		Navbar {}
		div {
			padding: "64px",
			text_align: "center",
			h1 { "Login Page" }
		}
	}
}

#[component]
fn HomePage() -> Element {
	// let _search_result =
	// 	use_resource(|| async { api::search("angry birds".to_string()).await });

	rsx! {
		BackgroundDots {}
		Navbar {}
		PageContainer {
			CategoryContainer {
				ItemCategory { label: "Fruits".to_string(),
					KioskItem { label: "Apple".to_string(), price_trend: PriceTrend::Up, lowest_sell: 2.49, highest_buy: 2.99 }
					KioskItem { label: "Banana".to_string(), price_trend: PriceTrend::Down, lowest_sell: 1.19, highest_buy: 1.49 }
					KioskItem { label: "Orange".to_string(), price_trend: PriceTrend::Up, lowest_sell: 2.09, highest_buy: 2.59 }
					KioskItem { label: "Mango".to_string(), price_trend: PriceTrend::Down, lowest_sell: 3.49, highest_buy: 3.99 }
					KioskItem { label: "Pineapple".to_string(), price_trend: PriceTrend::Up, lowest_sell: 4.29, highest_buy: 4.79 }
				}
				ItemCategory { label: "Vegetables".to_string(),
					KioskItem { label: "Carrot".to_string(), price_trend: PriceTrend::Up, lowest_sell: 0.99, highest_buy: 1.29 }
					KioskItem { label: "Broccoli".to_string(), price_trend: PriceTrend::Down, lowest_sell: 1.79, highest_buy: 2.09 }
					KioskItem { label: "Spinach".to_string(), price_trend: PriceTrend::Up, lowest_sell: 1.59, highest_buy: 1.99 }
					KioskItem { label: "Potato".to_string(), price_trend: PriceTrend::Down, lowest_sell: 0.89, highest_buy: 1.19 }
				}
				ItemCategory { label: "Snacks".to_string(),
					KioskItem { label: "Chips".to_string(), price_trend: PriceTrend::Up, lowest_sell: 2.49, highest_buy: 2.99 }
					KioskItem { label: "Chocolate".to_string(), price_trend: PriceTrend::Down, lowest_sell: 1.99, highest_buy: 2.49 }
					KioskItem { label: "Pretzels".to_string(), price_trend: PriceTrend::Up, lowest_sell: 1.49, highest_buy: 1.89 }
					KioskItem { label: "Popcorn".to_string(), price_trend: PriceTrend::Down, lowest_sell: 1.29, highest_buy: 1.69 }
				}
				ItemCategory { label: "Drinks".to_string(),
					KioskItem { label: "Water".to_string(), price_trend: PriceTrend::Up, lowest_sell: 0.99, highest_buy: 1.19 }
					KioskItem { label: "Juice".to_string(), price_trend: PriceTrend::Down, lowest_sell: 2.29, highest_buy: 2.79 }
					KioskItem { label: "Soda".to_string(), price_trend: PriceTrend::Up, lowest_sell: 1.49, highest_buy: 1.99 }
					KioskItem { label: "Coffee".to_string(), price_trend: PriceTrend::Down, lowest_sell: 3.49, highest_buy: 3.99 }
				}
				ItemCategory { label: "Bakery".to_string(),
					KioskItem { label: "Bread".to_string(), price_trend: PriceTrend::Up, lowest_sell: 2.99, highest_buy: 3.49 }
					KioskItem { label: "Croissant".to_string(), price_trend: PriceTrend::Down, lowest_sell: 1.79, highest_buy: 2.19 }
					KioskItem { label: "Bagel".to_string(), price_trend: PriceTrend::Up, lowest_sell: 1.29, highest_buy: 1.69 }
					KioskItem { label: "Donut".to_string(), price_trend: PriceTrend::Down, lowest_sell: 1.09, highest_buy: 1.49 }
				}
				ItemCategory { label: "Dairy".to_string(),
					KioskItem { label: "Milk".to_string(), price_trend: PriceTrend::Up, lowest_sell: 2.19, highest_buy: 2.59 }
					KioskItem { label: "Cheese".to_string(), price_trend: PriceTrend::Down, lowest_sell: 3.99, highest_buy: 4.49 }
					KioskItem { label: "Yogurt".to_string(), price_trend: PriceTrend::Up, lowest_sell: 1.29, highest_buy: 1.69 }
					KioskItem { label: "Butter".to_string(), price_trend: PriceTrend::Down, lowest_sell: 2.49, highest_buy: 2.99 }
				}
				ItemCategory { label: "Frozen Foods".to_string(),
					KioskItem { label: "Ice Cream".to_string(), price_trend: PriceTrend::Up, lowest_sell: 3.49, highest_buy: 3.99 }
					KioskItem { label: "Frozen Pizza".to_string(), price_trend: PriceTrend::Down, lowest_sell: 4.99, highest_buy: 5.49 }
					KioskItem { label: "Frozen Vegetables".to_string(), price_trend: PriceTrend::Up, lowest_sell: 2.29, highest_buy: 2.79 }
					KioskItem { label: "Frozen Berries".to_string(), price_trend: PriceTrend::Down, lowest_sell: 3.99, highest_buy: 4.49 }
				}
				ItemCategory { label: "Household".to_string(),
					KioskItem { label: "Paper Towels".to_string(), price_trend: PriceTrend::Up, lowest_sell: 4.49, highest_buy: 4.99 }
					KioskItem { label: "Laundry Detergent".to_string(), price_trend: PriceTrend::Down, lowest_sell: 7.99, highest_buy: 8.49 }
					KioskItem { label: "Dish Soap".to_string(), price_trend: PriceTrend::Up, lowest_sell: 2.19, highest_buy: 2.69 }
					KioskItem { label: "Trash Bags".to_string(), price_trend: PriceTrend::Down, lowest_sell: 3.99, highest_buy: 4.49 }
				}
				ItemCategory { label: "Personal Care".to_string(),
					KioskItem { label: "Shampoo".to_string(), price_trend: PriceTrend::Up, lowest_sell: 3.49, highest_buy: 3.99 }
					KioskItem { label: "Toothpaste".to_string(), price_trend: PriceTrend::Down, lowest_sell: 2.29, highest_buy: 2.79 }
					KioskItem { label: "Soap".to_string(), price_trend: PriceTrend::Up, lowest_sell: 1.19, highest_buy: 1.59 }
					KioskItem { label: "Deodorant".to_string(), price_trend: PriceTrend::Down, lowest_sell: 2.99, highest_buy: 3.49 }
				}
				ItemCategory { label: "Pet Supplies".to_string(),
					KioskItem { label: "Dog Food".to_string(), price_trend: PriceTrend::Up, lowest_sell: 8.99, highest_buy: 9.49 }
					KioskItem { label: "Cat Litter".to_string(), price_trend: PriceTrend::Down, lowest_sell: 6.49, highest_buy: 6.99 }
					KioskItem { label: "Bird Seed".to_string(), price_trend: PriceTrend::Up, lowest_sell: 3.99, highest_buy: 4.49 }
					KioskItem { label: "Fish Flakes".to_string(), price_trend: PriceTrend::Down, lowest_sell: 2.49, highest_buy: 2.99 }
				}
			}
		}
	}
}

#[component]
fn SellPage() -> Element {
	use crate::components::search_bar::SearchBar;

	let onsearch = |query: String| {
		tracing::info!("Search query: {}", query);
	};

	rsx! {
		BackgroundDots {}
		Navbar {}
		PageContainer {
			SearchBar { onsearch: onsearch }
		}
	}
}

#[component]
fn BuyPage() -> Element {
	rsx! {
		BackgroundDots {}
		Navbar {}
		div {
			padding: "64px",
			text_align: "center",
			h1 { "Buy Page" }
			p { "This is where you will be able to buy items." }
		}
	}
}
