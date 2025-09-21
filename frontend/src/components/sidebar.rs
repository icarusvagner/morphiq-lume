use leptos::prelude::*;
use phosphor_leptos::{Icon, SIGN_OUT};

#[component]
pub fn Sidebar() -> AnyView {
    view! {
        <aside class="flex relative flex-col p-8 bg-base-100">
            <div class="flex gap-5 items-center">
                <img
                    src="assets/logos/icons/raw/icon.png"
                    class="w-10 h-10"
                    alt="lume logo"
                />
                <h1 class="text-2xl font-bold font-raleway">"Morphiq Lume"</h1>
            </div>
            <div class="my-auto"></div>
            <button class="btn btn-primary">
                <Icon icon=SIGN_OUT attr:class="h-5 w-5" />
                "Log Out"
            </button>
        </aside>
    }
    .into_any()
}
