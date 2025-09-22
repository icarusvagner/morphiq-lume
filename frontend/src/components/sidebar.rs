use leptos::{either::Either, prelude::*};
use phosphor_leptos::{Icon, SIGN_OUT};

#[component]
pub fn Sidebar(#[prop(into, optional)] toggle_menu: RwSignal<bool>) -> AnyView {
    view! {
		<aside class=move || {
			format!(
				"flex relative flex-col py-5 bg-base-100 {}",
				if !toggle_menu.get() {
					"w-sm px-8 duration-200 ease-in-out transition-all"
				} else {
					"w-20 px-3 duration-200 ease-in-out transition-all"
				},
			)
		}>
			<div class="flex gap-5 justify-center items-center">
				<img
					src="assets/logos/icons/raw/icon.png"
					class="w-8 h-8"
					alt="lume logo"
				/>
				{move || {
					if !toggle_menu.get() {
						Either::Left(
							view! {
								<h1 class="text-2xl font-bold transition-all duration-200 ease-in-out delay-300 font-raleway">
									"Morphiq Lume"
								</h1>
							},
						)
					} else {
						Either::Right(())
					}
				}}
			</div>
			<div class="my-auto"></div>
			<button class="btn btn-primary">
				<Icon icon=SIGN_OUT attr:class="h-5 w-5" />
				{move || {
					if !toggle_menu.get() {
						Either::Left("Log Out")
					} else {
						Either::Right(())
					}
				}}
			</button>
		</aside>
	}
    .into_any()
}
