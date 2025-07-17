use yew::prelude::*;
use yew_router::prelude::*;
use uuid::Uuid;
use shared::WorkSessionWithTags;
use crate::api;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(SessionDetail)]
pub fn session_detail(props: &Props) -> Html {
    let session = use_state(|| None::<WorkSessionWithTags>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let navigator = use_navigator().unwrap();

    // Load session on component mount
    {
        let session = session.clone();
        let loading = loading.clone();
        let error = error.clone();
        let session_id = props.id;

        use_effect_with(session_id, move |_| {
            let session = session.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                match api::get_session(session_id).await {
                    Ok(data) => session.set(Some(data)),
                    Err(e) => error.set(Some(e)),
                }

                loading.set(false);
            });

            || {}
        });
    }

    fn format_duration(duration_seconds: i32) -> String {
        let hours = duration_seconds / 3600;
        let minutes = (duration_seconds % 3600) / 60;
        let secs = duration_seconds % 60;
        
        if hours > 0 {
            format!("{hours}h {minutes}m {secs}s")
        } else if minutes > 0 {
            format!("{minutes}m {secs}s")
        } else {
            format!("{secs}s")
        }
    }

    let on_back = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Sessions);
        })
    };

    html! {
        <div class="container mx-auto p-4">
            <div class="mb-6">
                <button
                    onclick={on_back}
                    class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                >
                    {"← Back to Sessions"}
                </button>
            </div>

            if *loading {
                <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 text-center">
                    <p class="text-gray-600">{"Loading session..."}</p>
                </div>
            } else if let Some(error_msg) = error.as_ref() {
                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                    {error_msg}
                </div>
            } else if let Some(session_data) = session.as_ref() {
                <div class="bg-white shadow-md rounded px-8 pt-6 pb-8">
                    <h1 class="text-3xl font-bold mb-6 text-gray-900">
                        {session_data.description.as_ref().unwrap_or(&"No description".to_string())}
                    </h1>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <h2 class="text-xl font-semibold mb-4 text-gray-800">{"Session Details"}</h2>
                            <div class="space-y-3">
                                <div>
                                    <span class="font-medium text-gray-700">{"Duration: "}</span>
                                    <span class="text-lg font-semibold text-blue-600">
                                        {format_duration(session_data.duration_seconds)}
                                    </span>
                                </div>
                                <div>
                                    <span class="font-medium text-gray-700">{"Created: "}</span>
                                    <span class="text-gray-600">
                                        {session_data.created_at.format("%Y-%m-%d %H:%M:%S").to_string()}
                                    </span>
                                </div>
                                <div>
                                    <span class="font-medium text-gray-700">{"Updated: "}</span>
                                    <span class="text-gray-600">
                                        {session_data.updated_at.format("%Y-%m-%d %H:%M:%S").to_string()}
                                    </span>
                                </div>
                            </div>
                        </div>
                        
                        <div>
                            <h2 class="text-xl font-semibold mb-4 text-gray-800">{"Tags"}</h2>
                            if session_data.tags.is_empty() {
                                <p class="text-gray-500 italic">{"No tags assigned to this session"}</p>
                            } else {
                                <div class="flex flex-wrap gap-2">
                                    {for session_data.tags.iter().map(|tag| {
                                        let default_color = "#6B7280".to_string();
                                        let color = tag.color.as_ref().unwrap_or(&default_color);
                                        html! {
                                            <span 
                                                class="inline-block rounded-full px-4 py-2 text-sm font-semibold text-white shadow-sm"
                                                style={format!("background-color: {color}")}
                                            >
                                                {&tag.name}
                                            </span>
                                        }
                                    })}
                                </div>
                            }
                        </div>
                    </div>
                </div>
            } else {
                <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 text-center">
                    <p class="text-gray-600">{"Session not found"}</p>
                </div>
            }
        </div>
    }
}