use stylist::{yew::{styled_component, Global}, css};
use yew::prelude::*;
use stylist::style;

#[function_component(App)]
fn app() -> Html {
	html! {
		<div class={set_main_style()} id="main">
			<div id="container">
				<ProjectComponent />
			</div>
		</div>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}

#[function_component()]
fn ProjectComponent() -> Html {
	html! {
		<div class={css!("display: flex; flex-direction: row; gap: 10px; margin: 10px;")}>
			<img src="img/daniel-mingook-kim.jpg" alt="floating umbrellas" class={css!("max-width: 50px; max-height: 50px;")}/>
			<div>
				<span>{ "This is a project" }</span>
				<div>{ "Avatar list" }</div>
			</div>
		</div>
	}
}


fn set_main_style() -> stylist::Style {
	style!(
		r#"
			display: flex;
			justify-content: center;
			align-items: center;
		
			#container {
				background-color: lightgrey;
				max-width: 428px;
				max-height: 926px;
				height: 100vh;
				width: 100vw;
			}
		"#
	 ).expect("Failed to mount style")
}