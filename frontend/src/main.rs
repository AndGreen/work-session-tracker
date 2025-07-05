use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod api;

use pages::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/sessions")]
    Sessions,
    #[at("/tags")]
    Tags,
    #[at("/sessions/:id")]
    SessionDetail { id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Sessions => html! { <Sessions /> },
        Route::Tags => html! { <Tags /> },
        Route::SessionDetail { id } => html! { <SessionDetail session_id={id} /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="min-h-screen bg-gray-50">
                <nav class="bg-white shadow-sm">
                    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                        <div class="flex justify-between h-16">
                            <div class="flex">
                                <div class="flex-shrink-0 flex items-center">
                                    <h1 class="text-xl font-semibold text-gray-900">
                                        {"Work Session Tracker"}
                                    </h1>
                                </div>
                                <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                                    <Link<Route> to={Route::Home} classes="border-indigo-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">
                                        {"Home"}
                                    </Link<Route>>
                                    <Link<Route> to={Route::Sessions} classes="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">
                                        {"Sessions"}
                                    </Link<Route>>
                                    <Link<Route> to={Route::Tags} classes="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">
                                        {"Tags"}
                                    </Link<Route>>
                                </div>
                            </div>
                        </div>
                    </div>
                </nav>
                <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}