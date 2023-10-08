use crate::components::input::Home;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/exoplanet_query_app.css"/>
        <Link rel="icon" type_="image/x-icon" href="/assets/favicon.ico"/>
        // sets the document title
        <Title text="Exoplanet Query App"/>
        <Router>
            <nav>
                <div class="navbar bg-neutral text-neutral-content">
                    <A href="/" class="btn btn-ghost normal-case text-xl">
                        "Home"
                    </A>
                </div>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                </Routes>
            </main>
        </Router>
    }
}
