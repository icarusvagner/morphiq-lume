use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, StaticSegment};

// Modules
mod components;
mod layouts;
mod pages;

// Top-Level pages
use crate::{
    layouts::main_layout::MainLayout,
    pages::{home::Home, login::LoginPage},
};

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="dark" />

        // sets the document title
        <Title text="Welcome to Morphiq Lume" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { NotFound }>
                <Route path=StaticSegment("auth") view=LoginPage />
                <ParentRoute path=StaticSegment("/") view=MainLayout>
                    <Route path=StaticSegment("") view=Home />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
