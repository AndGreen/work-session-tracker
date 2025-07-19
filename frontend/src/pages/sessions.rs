use yew::prelude::*;
use uuid::Uuid;
use shared::{CreateSessionRequest, WorkSessionWithTags, Tag};
use crate::api;

#[function_component(Sessions)]
pub fn sessions() -> Html {
    let sessions = use_state(Vec::<WorkSessionWithTags>::new);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    // Form states
    let description = use_state(String::new);
    let selected_tags = use_state(Vec::<Uuid>::new);
    let available_tags = use_state(Vec::<Tag>::new);

    // Load sessions and tags on component mount
    {
        let sessions = sessions.clone();
        let loading = loading.clone();
        let error = error.clone();
        let available_tags = available_tags.clone();

        use_effect_with((), move |_| {
            let sessions = sessions.clone();
            let loading = loading.clone();
            let error = error.clone();
            let available_tags = available_tags.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                // Load sessions
                match api::get_sessions().await {
                    Ok(data) => sessions.set(data),
                    Err(e) => error.set(Some(e)),
                }

                // Load tags
                match api::get_tags().await {
                    Ok(data) => available_tags.set(data),
                    Err(e) => error.set(Some(e)),
                }

                loading.set(false);
            });

            || {}
        });
    }

    let on_create_session = {
        let description = description.clone();
        let selected_tags = selected_tags.clone();
        let sessions = sessions.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let description = description.clone();
            let selected_tags = selected_tags.clone();
            let sessions = sessions.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if description.is_empty() {
                    error.set(Some("Description is required".to_string()));
                    return;
                }

                loading.set(true);
                error.set(None);

                let req = CreateSessionRequest {
                    duration_seconds: 0, // Will be set by backend based on actual work time
                    description: Some((*description).clone()),
                    tag_ids: (*selected_tags).clone(),
                };

                match api::create_session(req).await {
                    Ok(_) => {
                        // Session created successfully, now refresh the list
                        match api::get_sessions().await {
                            Ok(data) => {
                                sessions.set(data);
                                // Clear the form only after successful creation and refresh
                                description.set(String::new());
                                selected_tags.set(Vec::new());
                            }
                            Err(e) => error.set(Some(format!("Session created but failed to refresh list: {}", e))),
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to create session: {}", e)));
                    }
                }

                loading.set(false);
            });
        })
    };

    let on_delete_session = {
        let sessions = sessions.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |id: Uuid| {
            let sessions = sessions.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                if (api::delete_session(id).await).is_ok() {
                    match api::get_sessions().await {
                        Ok(data) => sessions.set(data),
                        Err(e) => error.set(Some(e)),
                    }
                }

                loading.set(false);
            });
        })
    };

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

    html! {
        <div class="container mx-auto p-4">
            <h1 class="text-3xl font-bold mb-6">{"Work Sessions"}</h1>
            
            if let Some(error_msg) = error.as_ref() {
                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                    {error_msg}
                </div>
            }

            // Create new session form
            <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-6">
                <h2 class="text-xl font-semibold mb-4">{"Create New Session"}</h2>
                <form onsubmit={on_create_session}>
                    <div class="mb-4">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="description">
                            {"Description"}
                        </label>
                        <input
                            id="description"
                            type="text"
                            value={(*description).clone()}
                            oninput={
                                let description = description.clone();
                                Callback::from(move |e: InputEvent| {
                                    if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                        description.set(input.value());
                                    }
                                })
                            }
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Enter session description"
                        />
                    </div>
                    
                    <div class="mb-4">
                        <label class="block text-gray-700 text-sm font-bold mb-2">
                            {"Tags"}
                        </label>
                        <div class="flex flex-wrap gap-2">
                            {for available_tags.iter().map(|tag| {
                                let tag_id = tag.id;
                                let is_selected = selected_tags.contains(&tag_id);
                                let selected_tags = selected_tags.clone();
                                
                                html! {
                                    <button
                                        type="button"
                                        class={classes!(
                                            "px-3", "py-1", "rounded", "text-sm", "border",
                                            if is_selected { "bg-blue-500 text-white border-blue-500" } else { "bg-gray-200 text-gray-700 border-gray-300" }
                                        )}
                                        onclick={
                                            Callback::from(move |_| {
                                                let mut tags = (*selected_tags).clone();
                                                if let Some(pos) = tags.iter().position(|&x| x == tag_id) {
                                                    tags.remove(pos);
                                                } else {
                                                    tags.push(tag_id);
                                                }
                                                selected_tags.set(tags);
                                            })
                                        }
                                    >
                                        {&tag.name}
                                    </button>
                                }
                            })}
                        </div>
                    </div>
                    
                    <button
                        type="submit"
                        disabled={*loading}
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline disabled:opacity-50"
                    >
                        {if *loading { "Creating..." } else { "Create Session" }}
                    </button>
                </form>
            </div>

            // Sessions list
            <div class="bg-white shadow-md rounded">
                <div class="px-6 py-4 border-b">
                    <h2 class="text-xl font-semibold">{"Sessions"}</h2>
                </div>
                
                if sessions.is_empty() && !*loading {
                    <div class="px-6 py-4 text-gray-500 text-center">
                        {"No sessions found. Create your first session above!"}
                    </div>
                } else {
                    <div class="divide-y divide-gray-200">
                        {for sessions.iter().map(|session| {
                            let session_id = session.id;
                            let on_delete = on_delete_session.clone();
                            
                            html! {
                                <div class="px-6 py-4">
                                    <div class="flex justify-between items-start">
                                        <div class="flex-1">
                                            <h3 class="text-lg font-medium text-gray-900 mb-2">
                                                {session.description.as_ref().unwrap_or(&"No description".to_string())}
                                            </h3>
                                            <div class="text-sm text-gray-600 space-y-1">
                                                <p>{"Duration: "}{format_duration(session.duration_seconds)}</p>
                                                <p>{"Created: "}{session.created_at.format("%Y-%m-%d %H:%M:%S").to_string()}</p>
                                                <p>{"Updated: "}{session.updated_at.format("%Y-%m-%d %H:%M:%S").to_string()}</p>
                                            </div>
                                            if !session.tags.is_empty() {
                                                <div class="mt-2">
                                                    <div class="flex flex-wrap gap-1">
                                                        {for session.tags.iter().map(|tag| {
                                                            let default_color = "#6B7280".to_string();
                                                            let color = tag.color.as_ref().unwrap_or(&default_color);
                                                            html! {
                                                                <span 
                                                                    class="inline-block bg-gray-200 rounded-full px-3 py-1 text-sm font-semibold text-gray-700"
                                                                    style={format!("background-color: {color}")}
                                                                >
                                                                    {&tag.name}
                                                                </span>
                                                            }
                                                        })}
                                                    </div>
                                                </div>
                                            }
                                        </div>
                                        <div class="ml-4 flex space-x-2">
                                            <button
                                                class="text-red-600 hover:text-red-800"
                                                onclick={
                                                    Callback::from(move |_| {
                                                        on_delete.emit(session_id);
                                                    })
                                                }
                                            >
                                                {"Delete"}
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                }
            </div>
        </div>
    }
}