use crate::components::constants::*;
#[cfg(feature = "web")]
use crate::window_events::use_on_window;
use dioxus::prelude::*;

#[component]
pub fn BackgroundDots() -> Element {
	let parallax_x = use_signal(|| 0.0);
	let parallax_y = use_signal(|| 0.0);

	to_owned![parallax_x, parallax_y];

	#[cfg(feature = "web")]
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
