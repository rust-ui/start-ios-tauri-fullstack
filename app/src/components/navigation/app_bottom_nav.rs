use leptos::prelude::*;

use crate::components::hooks::use_is_current_path;
use crate::components::ui::bottom_nav::{BottomNav, BottomNavGrid, BottomNavLabel, BottomNavLink};
use crate::routes::AppRoute;

#[component]
pub fn AppBottomNav() -> impl IntoView {
    let is_current_path = use_is_current_path();

    view! {
        <BottomNav class="fixed right-0 bottom-0 left-0 sm:hidden">
            <BottomNavGrid>
                {AppRoute::bottom_nav_routes()
                    .iter()
                    .map(|route| {
                        let path = route.path();
                        let is_current_path = is_current_path.clone();
                        view! {
                            <BottomNavLink attr:href=path attr:aria-current=move || is_current_path(path)>
                                {route.icon()}
                                <BottomNavLabel>{route.label()}</BottomNavLabel>
                            </BottomNavLink>
                        }
                    })
                    .collect_view()}
            </BottomNavGrid>
        </BottomNav>
    }
}
