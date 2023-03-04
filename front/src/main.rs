use gloo_net::http::{Request, Response};
use stylist::{yew::{styled_component, Global}, css};
use yew::prelude::*;
use stylist::style;
use gloo_console::log;
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveDateTime};

#[function_component(App)]
fn app() -> Html {
	html! {
		<div class={set_main_style()} id="main">
			<div id="container">
				<ProjectComponent />
				<ProjectComponent />
				<ProjectComponent />
				<ProjectComponent />
				<ProjectComponent />
				<ProjectComponent />
			</div>
		</div>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}

async fn get_projects_async() -> () {
	let response = Request::get("/projects")
		.send()
		.await
		.unwrap();

	let projects_list = response.json::<Vec<Project>>().await.unwrap();
	return projects_list;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Project {
	pub id: i32,
	pub name: String,
	pub created_at: NaiveDateTime,
	// pub total_expenses: f64,
	pub currency: String,
	pub users: Vec<Option<i32>>,
}

#[function_component()]
fn ProjectComponent() -> Html {
	let projects: UseStateHandle<Option<Project>> = use_state(|| None);
	html! {
		<div class={css!("display: flex; flex-direction: row; gap: 10px; margin: 10px;")}>
			<img src="img/daniel-mingook-kim.jpg" alt="floating umbrellas" class={css!("max-width: 50px; max-height: 50px;")}/>
			<div>
				<span>{ "This is a project" }</span>
				<div>
					<AvatarComponent />
					<AvatarComponent />
					<AvatarComponent />
				</div>
			</div>
		</div>
	}
	let projects = projects.clone();

}

#[function_component()]
fn AvatarComponent() -> Html {
	html! {
		<img src="img/avatar.png" alt="avatar" class="avatar"/>
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

			.avatar {
				width: 25px;
				height: 25px;
				border-radius: 50%;
			}
		"#
	 ).expect("Failed to mount style")
}