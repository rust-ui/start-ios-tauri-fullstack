use leptos::prelude::*;

use crate::components::ui::button::Button;

#[component]
pub fn DemoButtonReactive() -> impl IntoView {
    let count = RwSignal::new(0);
    let increment = move |_| *count.write() += 1;

    view! { <Button on:click=increment>"Click Me: " {count}</Button> }
}
