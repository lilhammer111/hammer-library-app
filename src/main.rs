use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        view! {cx,
            div(class="") {"hello world"}
        }
    });
}
