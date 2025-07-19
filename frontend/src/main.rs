use yew::prelude::*;
use yew_router::prelude::*;
use uuid::Uuid;

mod api;
mod pages;

use pages::{Sessions, Tags, SessionDetail};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/sessions")]
    Sessions,
    #[at("/sessions/:id")]
    SessionDetail { id: String },
    #[at("/tags")]
    Tags,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Redirect<Route> to={Route::Sessions}/> },
        Route::Sessions => html! { <Sessions /> },
        Route::SessionDetail { id } => {
            if let Ok(uuid) = Uuid::parse_str(&id) {
                html! { <SessionDetail id={uuid} /> }
            } else {
                html! { <div>{"Invalid session ID"}</div> }
            }
        },
        Route::Tags => html! { <Tags /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="min-h-screen bg-gray-50">
                <nav class="bg-white shadow">
                    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                        <div class="flex justify-between h-16">
                            <div class="flex">
                                <div class="flex-shrink-0 flex items-center">
                                    <h1 class="text-xl font-bold text-gray-900">{"Work Session Tracker"}</h1>
                                </div>
                                <div class="ml-6 flex space-x-8">
                                    <Link<Route> to={Route::Sessions} classes="inline-flex items-center px-1 pt-1 border-b-2 border-transparent text-sm font-medium text-gray-500 hover:text-gray-700 hover:border-gray-300">
                                        {"Sessions"}
                                    </Link<Route>>
                                    <Link<Route> to={Route::Tags} classes="inline-flex items-center px-1 pt-1 border-b-2 border-transparent text-sm font-medium text-gray-500 hover:text-gray-700 hover:border-gray-300">
                                        {"Tags"}
                                    </Link<Route>>
                                </div>
                            </div>
                        </div>
                    </div>
                </nav>
                
                <main class="py-10">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}