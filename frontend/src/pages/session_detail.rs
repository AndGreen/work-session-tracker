use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use shared::*;
use uuid::Uuid;
use crate::api;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub session_id: String,
}

#[function_component(SessionDetail)]
pub fn session_detail(props: &Props) -> Html {
    let session = use_state(|| None::<WorkSessionWithTags>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Load session on mount
    {
        let session = session.clone();
        let loading = loading.clone();
        let error = error.clone();
        let session_id = props.session_id.clone();
        
        use_effect_with(session_id.clone(), move |session_id| {
            if let Ok(id) = Uuid::parse_str(session_id) {
                spawn_local(async move {
                    match api::get_session(id).await {
                        Ok(data) => {
                            session.set(Some(data));
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(e));
                            loading.set(false);
                        }
                    }
                });
            } else {
                error.set(Some("Invalid session ID".to_string()));
                loading.set(false);
            }
        });
    }

    let format_duration = |seconds: i32| -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        
        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, secs)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, secs)
        } else {
            format!("{}s", secs)
        }
    };

    html! {
        <div class="px-4 py-6 sm:px-0">
            if *loading {
                <div class="bg-white px-4 py-8 text-center rounded-lg shadow">
                    <p class="text-sm text-gray-500">{"Loading session..."}</p>
                </div>
            } else if let Some(err) = error.as_ref() {
                <div class="bg-red-50 border border-red-200 rounded-md p-4">
                    <p class="text-sm text-red-600">{format!("Error: {}", err)}</p>
                </div>
            } else if let Some(session_data) = session.as_ref() {
                <div class="bg-white shadow overflow-hidden sm:rounded-lg">
                    <div class="px-4 py-5 sm:px-6">
                        <h3 class="text-lg leading-6 font-medium text-gray-900">
                            {"Session Details"}
                        </h3>
                        <p class="mt-1 max-w-2xl text-sm text-gray-500">
                            {"Information about this work session"}
                        </p>
                    </div>
                    <div class="border-t border-gray-200">
                        <dl>
                            <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                <dt class="text-sm font-medium text-gray-500">{"Duration"}</dt>
                                <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                    {format_duration(session_data.duration_seconds)}
                                </dd>
                            </div>
                            <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                <dt class="text-sm font-medium text-gray-500">{"Description"}</dt>
                                <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                    {session_data.description.as_ref().unwrap_or(&"No description".to_string())}
                                </dd>
                            </div>
                            <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                <dt class="text-sm font-medium text-gray-500">{"Tags"}</dt>
                                <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                    if session_data.tags.is_empty() {
                                        <span class="text-gray-500">{"No tags"}</span>
                                    } else {
                                        <div class="flex flex-wrap gap-2">
                                            {for session_data.tags.iter().map(|tag| {
                                                html! {
                                                    <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800">
                                                        {if let Some(color) = &tag.color {
                                                            html! {
                                                                <>
                                                                    <div 
                                                                        class="h-2 w-2 rounded-full mr-2"
                                                                        style={format!("background-color: {}", color)}
                                                                    ></div>
                                                                    {&tag.name}
                                                                </>
                                                            }
                                                        } else {
                                                            html! { {&tag.name} }
                                                        }}
                                                    </span>
                                                }
                                            })}
                                        </div>
                                    }
                                </dd>
                            </div>
                            <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                <dt class="text-sm font-medium text-gray-500">{"Created"}</dt>
                                <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                    {session_data.created_at.format("%Y-%m-%d %H:%M:%S UTC").to_string()}
                                </dd>
                            </div>
                            <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                <dt class="text-sm font-medium text-gray-500">{"Last Updated"}</dt>
                                <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                    {session_data.updated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string()}
                                </dd>
                            </div>
                        </dl>
                    </div>
                </div>
            }
        </div>
    }
}