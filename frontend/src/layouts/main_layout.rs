use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::sidebar::Sidebar;

#[component]
pub fn MainLayout() -> AnyView {
    view! {
        <section class="flex w-full min-h-screen bg-base-200">
            <Sidebar />
            <div class="flex flex-col">
                <Outlet />
            </div>
        </section>
    }
    .into_any()
}
