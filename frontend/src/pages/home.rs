use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="px-4 py-6 sm:px-0">
            <div class="text-center">
                <h1 class="text-3xl font-bold text-gray-900 mb-8">
                    {"Work Session Tracker"}
                </h1>
                <p class="text-lg text-gray-600 mb-8">
                    {"Track your work sessions, add descriptions, and organize with tags"}
                </p>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-4xl mx-auto">
                    <div class="bg-white overflow-hidden shadow rounded-lg">
                        <div class="px-4 py-5 sm:p-6">
                            <h3 class="text-lg leading-6 font-medium text-gray-900">
                                {"Current Session"}
                            </h3>
                            <div class="mt-2 max-w-xl text-sm text-gray-500">
                                <p>{"Start tracking your work session now"}</p>
                            </div>
                            <div class="mt-5">
                                <Link<Route> to={Route::Sessions} classes="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                    {"Start Session"}
                                </Link<Route>>
                            </div>
                        </div>
                    </div>

                    <div class="bg-white overflow-hidden shadow rounded-lg">
                        <div class="px-4 py-5 sm:p-6">
                            <h3 class="text-lg leading-6 font-medium text-gray-900">
                                {"Manage Tags"}
                            </h3>
                            <div class="mt-2 max-w-xl text-sm text-gray-500">
                                <p>{"Create and organize your session tags"}</p>
                            </div>
                            <div class="mt-5">
                                <Link<Route> to={Route::Tags} classes="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-green-600 hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500">
                                    {"Manage Tags"}
                                </Link<Route>>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="mt-12">
                    <h2 class="text-2xl font-bold text-gray-900 mb-4">
                        {"Recent Sessions"}
                    </h2>
                    <div class="bg-white shadow overflow-hidden sm:rounded-md">
                        <div class="px-4 py-5 sm:px-6">
                            <p class="text-sm text-gray-600">
                                {"Your recent work sessions will appear here"}
                            </p>
                            <div class="mt-4">
                                <Link<Route> to={Route::Sessions} classes="text-indigo-600 hover:text-indigo-900">
                                    {"View all sessions →"}
                                </Link<Route>>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}