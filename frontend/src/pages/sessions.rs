use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use shared::*;
use uuid::Uuid;
use crate::api;

#[function_component(Sessions)]
pub fn sessions() -> Html {
    let sessions = use_state(|| Vec::<WorkSessionWithTags>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let show_form = use_state(|| false);

    // Form state
    let duration_seconds = use_state(|| 0i32);
    let description = use_state(|| String::new());
    let selected_tags = use_state(|| Vec::<Uuid>::new());
    let available_tags = use_state(|| Vec::<Tag>::new());

    // Load sessions on mount
    {
        let sessions = sessions.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with((), move |_| {
            spawn_local(async move {
                match api::get_sessions().await {
                    Ok(data) => {
                        sessions.set(data);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
        });
    }

    // Load tags when form is shown
    {
        let available_tags = available_tags.clone();
        let show_form = show_form.clone();
        
        use_effect_with(*show_form, move |show_form| {
            if *show_form {
                spawn_local(async move {
                    if let Ok(tags) = api::get_tags().await {
                        available_tags.set(tags);
                    }
                });
            }
        });
    }

    let on_create_session = {
        let sessions = sessions.clone();
        let show_form = show_form.clone();
        let duration_seconds = duration_seconds.clone();
        let description = description.clone();
        let selected_tags = selected_tags.clone();
        
        Callback::from(move |_| {
            let sessions = sessions.clone();
            let show_form = show_form.clone();
            let duration = *duration_seconds;
            let desc = (*description).clone();
            let tags = (*selected_tags).clone();
            
            spawn_local(async move {
                let req = CreateSessionRequest {
                    duration_seconds: duration,
                    description: if desc.is_empty() { None } else { Some(desc) },
                    tag_ids: tags,
                };
                
                if let Ok(_) = api::create_session(req).await {
                    // Reload sessions
                    if let Ok(data) = api::get_sessions().await {
                        sessions.set(data);
                    }
                    show_form.set(false);
                }
            });
        })
    };

    let on_delete_session = {
        let sessions = sessions.clone();
        
        Callback::from(move |id: Uuid| {
            let sessions = sessions.clone();
            
            spawn_local(async move {
                if let Ok(_) = api::delete_session(id).await {
                    // Reload sessions
                    if let Ok(data) = api::get_sessions().await {
                        sessions.set(data);
                    }
                }
            });
        })
    };

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
            <div class="sm:flex sm:items-center">
                <div class="sm:flex-auto">
                    <h1 class="text-xl font-semibold text-gray-900">{"Work Sessions"}</h1>
                    <p class="mt-2 text-sm text-gray-700">{"Track and manage your work sessions"}</p>
                </div>
                <div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
                    <button
                        type="button"
                        class="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto"
                        onclick={let show_form = show_form.clone(); Callback::from(move |_| show_form.set(true))}
                    >
                        {"Add Session"}
                    </button>
                </div>
            </div>

            if *show_form {
                <div class="mt-8 bg-white shadow sm:rounded-lg">
                    <div class="px-4 py-5 sm:p-6">
                        <h3 class="text-lg leading-6 font-medium text-gray-900">{"Create New Session"}</h3>
                        <div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                            <div class="sm:col-span-3">
                                <label class="block text-sm font-medium text-gray-700">{"Duration (seconds)"}</label>
                                <input
                                    type="number"
                                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                                    value={duration_seconds.to_string()}
                                    onchange={let duration_seconds = duration_seconds.clone(); Callback::from(move |e: Event| {
                                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                        if let Ok(val) = input.value().parse::<i32>() {
                                            duration_seconds.set(val);
                                        }
                                    })}
                                />
                            </div>
                            <div class="sm:col-span-6">
                                <label class="block text-sm font-medium text-gray-700">{"Description"}</label>
                                <textarea
                                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                                    rows="3"
                                    value={(*description).clone()}
                                    onchange={let description = description.clone(); Callback::from(move |e: Event| {
                                        let textarea = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
                                        description.set(textarea.value());
                                    })}
                                />
                            </div>
                            <div class="sm:col-span-6">
                                <label class="block text-sm font-medium text-gray-700">{"Tags"}</label>
                                <div class="mt-2 space-y-2">
                                    {for available_tags.iter().map(|tag| {
                                        let tag_id = tag.id;
                                        let is_selected = selected_tags.contains(&tag_id);
                                        let selected_tags = selected_tags.clone();
                                        
                                        html! {
                                            <label class="flex items-center">
                                                <input
                                                    type="checkbox"
                                                    class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
                                                    checked={is_selected}
                                                    onchange={Callback::from(move |e: Event| {
                                                        let checkbox = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                        let mut tags = (*selected_tags).clone();
                                                        if checkbox.checked() {
                                                            tags.push(tag_id);
                                                        } else {
                                                            tags.retain(|&id| id != tag_id);
                                                        }
                                                        selected_tags.set(tags);
                                                    })}
                                                />
                                                <span class="ml-2 text-sm text-gray-700">{&tag.name}</span>
                                            </label>
                                        }
                                    })}
                                </div>
                            </div>
                        </div>
                        <div class="mt-5 sm:mt-6 sm:grid sm:grid-cols-2 sm:gap-3 sm:grid-flow-row-dense">
                            <button
                                type="button"
                                class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-indigo-600 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:col-start-2 sm:text-sm"
                                onclick={on_create_session}
                            >
                                {"Create Session"}
                            </button>
                            <button
                                type="button"
                                class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:col-start-1 sm:text-sm"
                                onclick={let show_form = show_form.clone(); Callback::from(move |_| show_form.set(false))}
                            >
                                {"Cancel"}
                            </button>
                        </div>
                    </div>
                </div>
            }

            <div class="mt-8 flex flex-col">
                <div class="-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8">
                    <div class="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8">
                        <div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
                            if *loading {
                                <div class="bg-white px-4 py-8 text-center">
                                    <p class="text-sm text-gray-500">{"Loading sessions..."}</p>
                                </div>
                            } else if let Some(err) = error.as_ref() {
                                <div class="bg-red-50 border border-red-200 rounded-md p-4">
                                    <p class="text-sm text-red-600">{format!("Error: {}", err)}</p>
                                </div>
                            } else {
                                <table class="min-w-full divide-y divide-gray-300">
                                    <thead class="bg-gray-50">
                                        <tr>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                {"Duration"}
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                {"Description"}
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                {"Tags"}
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                {"Created"}
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                {"Actions"}
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="bg-white divide-y divide-gray-200">
                                        {for sessions.iter().map(|session| {
                                            let session_id = session.id;
                                            let delete_callback = on_delete_session.clone();
                                            
                                            html! {
                                                <tr>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                                        {format_duration(session.duration_seconds)}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {session.description.as_ref().unwrap_or(&"No description".to_string())}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        <div class="flex flex-wrap gap-1">
                                                            {for session.tags.iter().map(|tag| {
                                                                html! {
                                                                    <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                                                                        {&tag.name}
                                                                    </span>
                                                                }
                                                            })}
                                                        </div>
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {session.created_at.format("%Y-%m-%d %H:%M").to_string()}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        <button
                                                            type="button"
                                                            class="text-red-600 hover:text-red-900"
                                                            onclick={Callback::from(move |_| delete_callback.emit(session_id))}
                                                        >
                                                            {"Delete"}
                                                        </button>
                                                    </td>
                                                </tr>
                                            }
                                        })}
                                    </tbody>
                                </table>
                            }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}