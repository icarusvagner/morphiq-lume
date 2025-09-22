use leptos::prelude::*;
use phosphor_leptos::{Icon, BELL_SIMPLE_RINGING, LIST, PALETTE};

#[component]
pub fn HeaderComponent(#[prop(into, optional)] toggle_menu: RwSignal<bool>) -> AnyView {
    view! {
        <header class="flex py-5 px-12 w-full bg-base-100">
            <button
                class="p-0 m-0 bg-transparent cursor-pointer"
                on:click=move |_| toggle_menu.set(!toggle_menu.get())
            >
                <Icon icon=LIST attr:class="text-primary h-6 w-6" />
            </button>
            <div class="mx-auto"></div>
            <div class="flex gap-5 items-center">
                <button class="p-0 m-0 bg-transparent cursor-pointer">
                    <Icon icon=PALETTE attr:class="h-6 w-6 text-primary" />
                </button>
                <button class="p-0 m-0 bg-transparent">
                    <Icon
                        icon=BELL_SIMPLE_RINGING
                        attr:class="text-primary h-6 w-6"
                    />
                </button>
            </div>
        </header>
    }
    .into_any()
}
