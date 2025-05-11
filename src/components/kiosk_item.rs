use dioxus::prelude::*;

use crate::components::constants::*;

#[derive(PartialEq, Clone)]
pub enum PriceTrend {
	Up,
	Down,
}

#[component]
pub fn KioskItem(
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
