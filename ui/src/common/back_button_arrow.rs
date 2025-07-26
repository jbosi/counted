use dioxus::prelude::*;

pub fn BackButtonArrow() -> Element {
    rsx! {
        button { type: "button", class: "btn btn-circle",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                polyline { points: "15 18 9 12 15 6" }
            }
        }
    }
}
