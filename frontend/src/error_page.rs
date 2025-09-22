use leptos::prelude::*;

#[component]
pub fn ErrorPage(#[prop(into)] errors: ArcRwSignal<Errors>) -> AnyView {
    view! {
        <section class="flex flex-col justify-center items-center w-full min-h-screen bg-base-200">
            <div class="flex flex-col">
                <h1 class="mb-5 text-xl font-bold text-base-content">
                    "Uh oh! Something went wrong!"
                </h1>
                <div class="flex flex-col py-2 px-5 text-red-500 bg-error">
                    <p>"Errors:"</p>
                    <ul>
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect_view()
                        }}
                    </ul>
                </div>
            </div>
        </section>
    }
    .into_any()
}
