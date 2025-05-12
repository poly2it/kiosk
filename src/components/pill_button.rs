use super::constants::*;
use dioxus::prelude::*;

#[component]
pub fn PillButton(
	label: String,
	#[props(default = WHITE_PURE)] color: &'static str,
	#[props(default = COLOR_FG_ACCENT)] border: &'static str,
	onclick: EventHandler<MouseEvent>,
) -> Element {
	rsx! {
		button {
			onclick: move |evt| onclick.call(evt),
			display: "flex",
			align_items: "center",
			justify_content: "center",
			height: "34px",
			padding_left: "20px",
			padding_right: "20px",
			background_color: "{color}",
			border: "1px solid {border}",
			border_radius: "1000px",
			font_weight: "700",
			font_size: FONT_SIZE_IMPACTFUL,
			line_height: "34px",
			color: COLOR_TEXT_PRIMARY,
			cursor: "pointer",
			user_select: "none",
			white_space: "nowrap",
			"{label}"
		}
	}
}
