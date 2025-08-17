use dioxus::document::Eval;
use dioxus::prelude::*;

pub fn close_dropdown() -> Eval {
    document::eval(
        "document.activeElement.closest('.dropdown').removeAttribute('open'); return null",
    )
}
