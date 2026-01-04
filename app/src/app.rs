use leptos::prelude::*;
use leptos_meta::{Html, Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};

use crate::components::hooks::ThemeMode;
use crate::components::navigation::app_bottom_nav::AppBottomNav;
use crate::components::navigation::header::Header;
use crate::domain::settings::page_settings::PageSettings;
use crate::domain::template::page::TemplatePage;
use crate::routes::AppRoute;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let theme_mode = ThemeMode::init();

    view! {
        <Title text="Rust/UI Starter — iOS" />

        <Html {..} class=move || if theme_mode.is_dark() { "dark" } else { "" } />

        <Router>
            // Empty touchstart enables CSS :active on iOS
            <div class="flex flex-col pb-16 sm:pb-0 h-dvh" on:touchstart=|_| {}>
                <Header />

                <main class="overflow-y-auto flex-1 p-2">
                    <Routes fallback=|| view! { <NotFound /> }>
                        <Route path=AppRoute::Home.segment() view=PageHome />
                        <Route path=AppRoute::Templates.segment() view=TemplatePage />
                        <Route path=AppRoute::Settings.segment() view=PageSettings />
                    </Routes>
                </main>
            </div>

            <AppBottomNav />
        </Router>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn PageHome() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center items-center px-6 h-full text-center">
            <img src="/icons/logo.png" alt="Rust-UI Logo" class="mb-6 rounded-2xl size-20" />
            <h1 class="text-2xl font-bold tracking-tight">"Rust/UI Stack"</h1>
            <p class="mt-2 max-w-xs text-muted-foreground text-pretty">
                "Build cross-platform apps with Rust. By Rustify.rs Bootcamp, powered by "
                <a
                    href="https://www.rust-ui.com"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="underline text-primary underline-offset-4 hover:text-primary/80"
                >
                    "Rust/UI"
                </a> "."
            </p>
        </div>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! { "Not found" }
}
