use leptos::prelude::*;

#[component]
pub fn LoginPage() -> AnyView {
    let pass_input = RwSignal::new(String::new());
    let uname_input = RwSignal::new(String::new());

    view! {
        <section class="flex justify-center items-center min-h-screen bg-base-200">
            <div class="grid overflow-hidden grid-cols-2 rounded-lg w-4xl bg-base-100">
                <div class="w-full h-32 md:h-auto">
                    <img
                        src="assets/images/login-office.jpeg"
                        alt="login office"
                        class="w-full h-full"
                    />
                </div>
                <form action="" class="flex flex-col gap-5 justify-center p-10">
                    <h1 class="text-2xl font-bold text-gray-700 dark:text-gray-300">
                        "Admin Login"
                    </h1>
                    <LoginInput
                        label="Username"
                        value=uname_input
                        placeholder="admin12344".into()
                    />
                    <LoginInput
                        label="Password"
                        value=pass_input
                        placeholder="********".into()
                    />

                    <button type="submit" class="btn btn-primary">
                        "Submit"
                    </button>
                </form>
            </div>
        </section>
    }
    .into_any()
}

#[component]
fn LoginInput(
    #[prop(into)] label: String,
    #[prop(into)] value: RwSignal<String>,
    #[prop(optional)] placeholder: MaybeProp<String>,
) -> AnyView {
    view! {
        <div class="flex flex-col w-full">
            <label
                for=label.clone().to_lowercase()
                class="text-lg text-gray-700 dark:text-gray-300"
            >
                {label.clone()}
            </label>
            <input
                id=label.to_lowercase()
                placeholder=move || {
                    placeholder.get_untracked().unwrap_or_default()
                }
                prop:value=move || value.get()
                on:input=move |e| value.set(event_target_value(&e))
                class="w-full input"
            />
        </div>
    }
    .into_any()
}
