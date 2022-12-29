use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
	html! {
		<h1>{ "Hello Céline <3" }</h1>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}