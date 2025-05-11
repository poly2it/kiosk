use dioxus::prelude::*;
use super::constants::*;

#[component]
pub fn CategoryContainer(children: Element) -> Element {
	rsx! {
		div {
			columns: "420px",
			column_gap: "32px",
			width: "calc(100vw - 80px)",
			max_width: "1400px",
			margin: "40px auto",
			{children}
		}
	}
}

#[component]
pub fn ItemCategory(label: String, children: Element) -> Element {
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
				"{label}"
			}
			div {
				width: "100%",
				height: "1px",
				background: COLOR_FG_DIVIDER,
				margin: "5px 0 5px 0",
			}
			{children}
		}
	}
}
