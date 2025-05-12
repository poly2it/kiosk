use super::constants::*;
use dioxus::prelude::*;

#[component]
pub fn SearchBar(onsearch: EventHandler<String>) -> Element {
	let mut query = use_signal(|| "".to_string());

	let oninput = move |evt: FormEvent| {
		let value = evt.value();
		query.set(value.clone());
		onsearch.call(value);
	};

	rsx! {
		div {
			display: "flex",
			align_items: "center",
			background: "{WHITE_PURE}",
			border_radius: "1000px",
			height: "34px",
			padding_left: "20px",
			padding_right: "20px",
			margin: "0 auto",
			border: "1px solid {COLOR_FG_ACCENT}",
			overflow: "hidden",

			input {
				flex: "2",
				border: "none",
				outline: "none",
				background: "transparent",
				font_size: FONT_SIZE_CONTENT,
				font_weight: "500",
				color: COLOR_TEXT_PRIMARY,
				width: "100%",
				line_height: "34px",
				r#type: "text",
				value: "{query}",
				oninput: oninput,
				placeholder: "Search...",
			}
		}
	}
}
