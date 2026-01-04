use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

use crate::components::navigation::reload_button::ReloadButton;
use crate::components::navigation::theme_toggle::ThemeToggle;
use crate::routes::AppRoute;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        // Mobile: absolute positioned
        <div class="absolute top-4 right-4 sm:hidden z-100">
            <ThemeToggle />
        </div>
        <div class="absolute top-2.5 right-8 sm:hidden z-100">
            <ReloadButton />
        </div>

        // Desktop: inside header with justify-between
        <header class="hidden sticky top-0 justify-between items-center p-4 border-b sm:flex z-100 bg-background">
            <nav class="flex justify-between items-center w-full">
                <div class="flex gap-4 items-center">
                    <MenuLink route=AppRoute::Home />
                    <MenuLink route=AppRoute::Templates />
                </div>

                <ThemeToggle />
            </nav>
        </header>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn MenuLink(route: AppRoute) -> impl IntoView {
    let location = use_location();
    let path = route.path();
    let is_active = Memo::new(move |_| location.pathname.get() == path);

    view! {
        <A class:font-bold=move || is_active.get() href=path>
            {route.label()}
        </A>
    }
}
