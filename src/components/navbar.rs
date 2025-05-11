use dioxus::prelude::*;

use super::constants::*;

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
			box_shadow: "",
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
					box_shadow: "0 0 8px 0px #ffffffcc inset,",
				}
			}
		}
	}
}
