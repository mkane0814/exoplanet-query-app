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
                    <Route path="/" view=QueryTabWrapper/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn QueryTabWrapper() -> impl IntoView {
    let tab_initial_id: usize = 1;
    let mut next_tab_id = tab_initial_id;
    let tab_ids = (0..tab_initial_id).collect::<Vec<_>>();

    let current_active_tab = create_rw_signal(tab_initial_id);

    let (tabs, set_tabs) = create_signal(tab_ids);
    let add_tab = move |_| {
        set_tabs.update(move |tabs| tabs.push(next_tab_id));
        current_active_tab.update(move |stored_id| *stored_id = next_tab_id);
        next_tab_id += 1;
    };

    view! {
        <div class="tabs border-b-1 border-">
            <For each=tabs key=|tab| *tab let:id>
                <button
                    id=id
                    class="tab tab-lifted"
                    class:tab-active=move || current_active_tab.get() == id
                    on:click=move |_| {
                        current_active_tab.update(move |stored_id| *stored_id = id)
                    }
                >

                    {format!("Tab {}", id + 1)}
                </button>
                <button
                    id=id
                    class="tab tab-lifted"
                    on:click=move |_| {
                        set_tabs.update(move |tabs| tabs.retain(|tab_id| tab_id != &id));
                    }
                >

                    "x"
                </button>

            </For>

            <button class="tab tab-lifted" on:click=add_tab>
                "+"
            </button>
        </div>
        <For each=tabs key=|tab| *tab let:id>
            <Show when=move || current_active_tab.get() == id fallback=|| ()>
                <Home id/>
            </Show>
        </For>
    }
}
