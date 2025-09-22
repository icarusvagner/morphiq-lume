use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::{header::HeaderComponent, sidebar::Sidebar};

#[component]
pub fn MainLayout() -> AnyView {
    let toggle_menu = RwSignal::new(false);

    view! {
        <section class="flex w-full min-h-screen bg-base-200">
            <Sidebar toggle_menu />
            <div class="flex flex-col w-full">
                <HeaderComponent toggle_menu />
                <Outlet />
            </div>
        </section>
    }
    .into_any()
}
