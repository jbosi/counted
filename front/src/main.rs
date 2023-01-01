use stylist::{yew::{styled_component, Global}, css};
use yew::prelude::*;
use stylist::style;

#[function_component(App)]
fn app() -> Html {
	let main_style = set_main_style();
	html! {
		<div class={main_style} id="main">
			<div id="container">
			</div>
		</div>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}

#[function_component()]
fn main_container() -> Html {
	html! {
		<h1>{ "Hello Céline <3" }</h1>
	}
}


fn set_main_style() -> stylist::Style {
	style!(
		r#"
			display: flex;
			justify-content: center;
			align-items: center;
		
			#container {
				background-color: red;
				max-width: 428px;
				max-height: 926px;
				height: 100vh;
				width: 100vw;
			}
		"#
	 ).expect("Failed to mount style")
}