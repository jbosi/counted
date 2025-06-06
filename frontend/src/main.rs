use dioxus::prelude::*;
use shared::User;
use wasm_bindgen::prelude::*;

fn app(cx: Scope) -> Element {
    let user = use_state(cx, || None);

    use_future(cx, (), |_| {
        let user = user.clone();
        async move {
            let fetched: Result<User, _> = reqwest::get("http://localhost:3000/user")
                .await
                .unwrap()
                .json()
                .await;
            if let Ok(u) = fetched {
                user.set(Some(u));
            }
        }
    });

    cx.render(rsx! {
        div {
            h1 { "Dioxus Frontend" }
            match &*user.current() {
                Some(user) => rsx!(p { "User: {user.username} ({user.email})" }),
                None => rsx!(p { "Loading user..." })
            }
        }
    })
}

#[wasm_bindgen(start)]
pub fn start() {
    dioxus_web::launch(app);
}