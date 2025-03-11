use leptos::*;
use leptos_router::*;
use dotenvy::dotenv;

mod components;
mod services;
mod utils;
mod models;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="container mx-auto">
                <Routes>
                    <Route 
                        path="/vacation" 
                        view=|| view! { <components::vacation_request::VacationRequestForm/> }
                    />
                    // Other routes...
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    // Load environment variables
    dotenv().ok();

    // Configure logging
    tracing_subscriber::fmt::init();

    // Mount the application
    mount_to_body(|| view! { <App/> })
}