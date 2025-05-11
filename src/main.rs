#![allow(non_camel_case_types)]

#[allow(unused)]
use dioxus::logger::tracing;
use dioxus::prelude::*;
use web_sys;

const RED_DEEP: &str = "#BD0025";
const RED_BRIGHT: &str = "#F40030";
const RED_PINKISH: &str = "#F55272";
const RED_LIGHT: &str = "#FAA7B8";
const RED_NEAR_WHITE: &str = "#FEFEFE";

const ORANGE_DEEP: &str = "#CB7D00";
const ORANGE_BRIGHT: &str = "#FF9E00";
const ORANGE_YELLOW: &str = "#FFBE56";
const ORANGE_PALE: &str = "#FFDFAB";
const ORANGE_WHITE: &str = "#FFFFFF";

const BLUE_DEEP: &str = "#033C85";
const BLUE_BRIGHT: &str = "#1060C6";
const BLUE_MEDIUM: &str = "#5086CC";
const BLUE_LIGHT: &str = "#A0BEE5";
const BLUE_NEAR_WHITE: &str = "#FCFCFC";

const GREEN_DEEP: &str = "#39AF00";
const GREEN_BRIGHT: &str = "#4BE900";
const GREEN_LIGHT: &str = "#82EB4F";
const GREEN_PALE: &str = "#BEF5A4";
const GREEN_NEAR_WHITE: &str = "#FEFEFE";

const BLACK_DEEP: &str = "#181818";
const BLACK_SOFT: &str = "#222222";
const BLACK_LIGHT: &str = "#444444";
const WHITE_PURE: &str = "#FFFFFF";
const WHITE_WARM: &str = "#F9F6F2";
const WHITE_COOL: &str = "#F5F8FA";

const COLOR_TEXT_PRIMARY: &str = "#222";
const COLOR_TEXT_SECONDARY: &str = "#222";

const COLOR_FG_PRIMARY: &str = BLUE_MEDIUM;
const COLOR_FG_ACCENT: &str = BLUE_LIGHT;
const COLOR_FG_DIVIDER: &str = "#00000044";

const COLOR_BG_PRIMARY: &str = WHITE_PURE;

const COLOR_INDICATOR_UP: &str = GREEN_BRIGHT;
const COLOR_INDICATOR_DOWN: &str = RED_BRIGHT;

/// Font size for main item content: item names and prices in the kiosk list.
const FONT_SIZE_CONTENT: &str = "12px";
/// Font size for visual indicators: trend arrows, emoji, and small icons.
const FONT_SIZE_INDICATOR: &str = "12px";
/// Font size for impactful UI: navigation, headers, logo, and main UI controls.
const FONT_SIZE_IMPACTFUL: &str = "14px";

#[derive(PartialEq, Clone)]
pub enum PriceTrend {
	Up,
	Down,
}

#[component]
fn KioskItem(
	label: String,
	price_trend: PriceTrend,
	lowest_sell: f64,
	highest_buy: f64,
) -> Element {
	rsx! {
		div {
			display: "flex",
			align_items: "center",
			justify_content: "space-between",
			padding: "2px 0",
			font_family: "Montserrat",
			color: COLOR_TEXT_PRIMARY,

			KioskLabel { label: label.clone() }
			div {
				display: "flex",
				gap: "8px",
				align_items: "center",
				justify_content: "flex-end",
				min_width: "220px",

				KioskItemTrend { price_trend: price_trend.clone() }
				KioskItemPrice { value: lowest_sell }
				KioskItemPrice { value: highest_buy }
				KioskItemHypeButton {}
			}
		}
	}
}

#[component]
fn KioskLabel(label: String) -> Element {
	rsx! {
		div {
			display: "flex",
			gap: "8px",
			align_items: "center",
			flex: "1",
			min_width: "0",

			img {
				src: EMPTY_IMG,
				width: "20px",
				height: "20px",
				alt: "",
				background_color: "#000",
				flex_shrink: "0",
			}
			span {
				font_family: "Montserrat",
				font_weight: "500",
				font_size: FONT_SIZE_CONTENT,
				text_align: "left",
				white_space: "nowrap",
				line_height: "1.2",
				max_width: "90px",
				flex: "1",
				color: COLOR_TEXT_PRIMARY,
				"{label}"
			}
		}
	}
}

#[component]
fn KioskItemTrend(price_trend: PriceTrend) -> Element {
	let (trend_icon, trend_color) = match price_trend {
		PriceTrend::Up => ("▲", COLOR_INDICATOR_UP),
		PriceTrend::Down => ("▼", COLOR_INDICATOR_DOWN),
	};
	rsx! {
		span {
			font_family: "Montserrat",
			color: "{trend_color}",
			font_size: FONT_SIZE_INDICATOR,
			font_weight: "bold",
			text_align: "center",
			white_space: "nowrap",
			line_height: "1.2",
			"{trend_icon}"
		}
	}
}

#[component]
fn KioskItemPrice(value: f64) -> Element {
	rsx! {
		span {
			font_family: "Montserrat",
			// Tabular figures, ie. monospaced numbers.
			font_feature_settings: "'tnum'",
			font_size: FONT_SIZE_CONTENT,
			color: COLOR_TEXT_SECONDARY,
			text_align: "right",
			white_space: "nowrap",
			line_height: "1.2",
			font_weight: "500",
			"{value:.2}"
		}
	}
}

#[component]
fn KioskItemHypeButton() -> Element {
	let hyped = use_signal(|| false);
	let spark = use_signal(|| false);

	let onclick = {
		to_owned![hyped, spark];
		move |_| {
			if hyped() {
				return;
			}
			hyped.set(true);
			spark.set(true);
		}
	};

	rsx! {
		button {
			font_family: "Montserrat",
			background: "none",
			border: "none",
			font_weight: "700",
			font_size: FONT_SIZE_INDICATOR,
			cursor: if hyped() { "default" } else { "pointer" },
			white_space: "nowrap",
			disabled: hyped(),
			text_overflow: "clip",
			onclick: onclick,
			span {
				class: {
					let mut c = String::from("hype-icon-spark-container");
					if spark() { c.push_str(" spark"); }
					c
				},
				display: "inline-block",
				position: "relative",
				span {
					class: {
						let mut c = String::from("hype-icon");
						if hyped() { c.push_str(" hyped"); }
						if spark() { c.push_str(" spark"); }
						c
					},
					display: "inline-block",
					"🔥"
				}
				for _ in 0..8 {
					span {
						class: "dazzle",
					}
				}
			}
		}
	}
}

mod window_events;
use window_events::use_on_window;

static CSS: Asset = asset!("resources/styles.css");
static LQIP_CSS: Asset = asset!("resources/lqip.css");
static NORMALIZE: Asset = asset!("resources/normalize.css");
static EMPTY_IMG: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNgYAAAAAMAASsJTYQAAAAASUVORK5CYII=";
static GFONTS: &str = "https://fonts.googleapis.com/css2?family=Dela+Gothic+One&family=Montserrat:ital,wght@0,100..900;1,100..900&family=Spline+Sans:wght@300..700&display=swap";

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
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
		BackgroundDots {}
		Navbar {}
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

#[component]
fn BackgroundDots() -> Element {
	let parallax_x = use_signal(|| 0.0);
	let parallax_y = use_signal(|| 0.0);

	to_owned![parallax_x, parallax_y];

	use_on_window("mousemove", move |evt: web_sys::MouseEvent| {
		let x = evt.x();
		let y = evt.y();
		let w = web_sys::window()
			.unwrap()
			.inner_width()
			.unwrap()
			.as_f64()
			.unwrap_or(1.0);
		let h = web_sys::window()
			.unwrap()
			.inner_height()
			.unwrap()
			.as_f64()
			.unwrap_or(1.0);
		let px = (x as f64 / w) * 16.0;
		let py = (y as f64 / h) * 16.0;
		parallax_x.set(px);
		parallax_y.set(py);
	});

	rsx! {
		div {
			position: "fixed",
			z_index: "-1",
			top: "-{parallax_y}px",
			left: "-{parallax_x}px",
			width: "100vw",
			height: "100vh",
			pointer_events: "none",
			overflow: "hidden",
			transition: "
				0.2s top linear,
				0.2s left linear
			",
			div {
				width: "140vw",
				height: "140vh",
				position: "absolute",
				left: "-20vw",
				top: "-20vh",
				background_color: COLOR_BG_PRIMARY,
				background_image: "radial-gradient(#00000033 1px, transparent 0)",
				background_size: "20px 20px",
				background_position: "-19px -19px",
				transform: "rotate(-8deg)",
			}
			div {
				position: "absolute",
				left: "0",
				top: "0",
				width: "100vw",
				height: "100vh",
				pointer_events: "none",
				background: "linear-gradient(to bottom, transparent, {COLOR_BG_PRIMARY})",
			}
		}
	}
}

#[component]
fn NavbarButton(href: String, label: String) -> Element {
	rsx! {
		a {
			href: "{href}",
			text_decoration: "none",
			color: "#000000",
			font_weight: "700",
			font_size: FONT_SIZE_IMPACTFUL,
			line_height: "64px",
			user_select: "none",
			white_space: "nowrap",
			"{label}"
		}
	}
}

#[derive(PartialEq, Props, Clone)]
struct CategoryContainerProps {
	children: Element,
}

#[component]
fn CategoryContainer(props: CategoryContainerProps) -> Element {
	rsx! {
		div {
			columns: "420px",
			column_gap: "32px",
			width: "calc(100vw - 80px)",
			max_width: "1400px",
			margin: "40px auto",
			{props.children}
		}
	}
}

#[derive(PartialEq, Props, Clone)]
struct ItemCategoryProps {
	label: String,
	children: Element,
}

#[component]
fn ItemCategory(props: ItemCategoryProps) -> Element {
	rsx! {
		div {
			display: "inline-block",
			vertical_align: "top",
			width: "100%",
			min_width: "220px",
			break_inside: "avoid",
			margin_bottom: "32px",
			border_radius: "8px",
			box_sizing: "border-box",

			span {
				display: "flex",
				align_items: "center",
				justify_content: "space-between",
				width: "100%",
				font_weight: "700",
				color: COLOR_TEXT_PRIMARY,
				user_select: "none",
				font_family: "Montserrat",
				font_size: FONT_SIZE_IMPACTFUL,
				text_align: "left",
				flex: "1",
				"{props.label}"
			}
			div {
				width: "100%",
				height: "1px",
				background: COLOR_FG_DIVIDER,
				margin: "5px 0 5px 0",
			}
			{
				props.children
			}
		}
	}
}

#[component]
fn NavbarLogo() -> Element {
	rsx! {
		a {
			class: "navbar-logo",
			href: "#",
			text_decoration: "none",
			color: "#000000",
			font_weight: "700",
			font_size: FONT_SIZE_IMPACTFUL,
			line_height: "64px",
			user_select: "none",
			white_space: "nowrap",
			font_family: "Dela Gothic One",
			"Spetskiosken"
		}
	}
}

#[component]
fn NavbarDivider() -> Element {
	rsx! {
		div {
			flex_shrink: "0",
			width: "2px",
			height: "24px",
			background: COLOR_FG_DIVIDER,
			margin_left: "-8px",
			margin_right: "-8px"
		}
	}
}

#[component]
fn LogInButton() -> Element {
	rsx! {
		li {
			user_select: "none",
			display: "flex",
			flex_direction: "row",
			height: "34px",
			background_color: "white",
			margin_right: "6px",
			border_radius: "1000px",
			a {
				padding_left: "20px",
				padding_right: "2px",
				href: "#",
				text_decoration: "none",
				color: "#000",
				font_weight: "700",
				font_size: FONT_SIZE_IMPACTFUL,
				line_height: "34px",
				user_select: "none",
				white_space: "nowrap",
				"Log in"
			}
			div {
				margin: "5px 6px",
				img {
					class: "force-lqip",
					src: EMPTY_IMG,
					width: "24px",
					height: "24px",
					style: "--lqip:-151216",
					border_radius: "1000px",
					box_shadow: "
						0 0 8px 0px #ffffffcc inset,
					",
				}
			}
		}
	}
}

#[component]
pub fn Navbar() -> Element {
	rsx! {
		nav {
			class: "navbar",
			font_family: "Montserrat",
			background: "
				linear-gradient(
					0deg,
					{COLOR_FG_PRIMARY} 0%,
					{COLOR_FG_ACCENT} 100%
				)
			",
			box_shadow: "
				0 8px 32px 0px #0004,
				0 0 16px 0px {COLOR_FG_ACCENT} inset
			",
			display: "flex",
			width: "80%",
			max_width: "800px",
			height: "48px",
			margin_left: "auto",
			margin_right: "auto",
			margin_top: "5vh",
			margin_bottom: "5vh",
			border_radius: "1000px",
			ul {
				display: "flex",
				align_items: "center",
				width: "100%",
				list_style: "none",
				gap: "32px",

				NavbarLogo { }
				NavbarDivider {}
				NavbarButton { href: "#".to_string(), label: "Buy".to_string() }
				NavbarButton { href: "#".to_string(), label: "Sell".to_string() }
				li {
					margin_left: "auto"
				}
				LogInButton { }
			}
		}
	}
}
