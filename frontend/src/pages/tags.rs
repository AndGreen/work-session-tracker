use yew::prelude::*;
use uuid::Uuid;
use shared::{CreateTagRequest, UpdateTagRequest, Tag};
use crate::api;

#[function_component(Tags)]
pub fn tags() -> Html {
    let tags = use_state(Vec::<Tag>::new);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let editing_tag = use_state(|| None::<Uuid>);

    // Form states
    let tag_name = use_state(String::new);
    let tag_color = use_state(String::new);

    // Load tags on component mount
    {
        let tags = tags.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            let tags = tags.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                match api::get_tags().await {
                    Ok(data) => tags.set(data),
                    Err(e) => error.set(Some(e)),
                }

                loading.set(false);
            });

            || {}
        });
    }

    let on_create_tag = {
        let tag_name = tag_name.clone();
        let tag_color = tag_color.clone();
        let tags = tags.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let tag_name = tag_name.clone();
            let tag_color = tag_color.clone();
            let tags = tags.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if tag_name.is_empty() {
                    error.set(Some("Tag name is required".to_string()));
                    return;
                }

                loading.set(true);
                error.set(None);

                let req = CreateTagRequest {
                    name: (*tag_name).clone(),
                    color: if tag_color.is_empty() { 
                        Some("#3B82F6".to_string()) 
                    } else { 
                        Some((*tag_color).clone()) 
                    },
                };

                if (api::create_tag(req).await).is_ok() {
                    match api::get_tags().await {
                        Ok(data) => {
                            tags.set(data);
                            tag_name.set(String::new());
                            tag_color.set(String::new());
                        }
                        Err(e) => error.set(Some(e)),
                    }
                }

                loading.set(false);
            });
        })
    };

    let on_update_tag = {
        let editing_tag = editing_tag.clone();
        let tag_name = tag_name.clone();
        let tag_color = tag_color.clone();
        let tags = tags.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(tag_id) = *editing_tag {
                let tag_name = tag_name.clone();
                let tag_color = tag_color.clone();
                let tags = tags.clone();
                let loading = loading.clone();
                let error = error.clone();
                let editing_tag = editing_tag.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if tag_name.is_empty() {
                        error.set(Some("Tag name is required".to_string()));
                        return;
                    }

                    loading.set(true);
                    error.set(None);

                    let req = UpdateTagRequest {
                        name: Some((*tag_name).clone()),
                        color: if tag_color.is_empty() { 
                            Some("#3B82F6".to_string()) 
                        } else { 
                            Some((*tag_color).clone()) 
                        },
                    };

                    if (api::update_tag(tag_id, req).await).is_ok() {
                        match api::get_tags().await {
                            Ok(data) => {
                                tags.set(data);
                                tag_name.set(String::new());
                                tag_color.set(String::new());
                                editing_tag.set(None);
                            }
                            Err(e) => error.set(Some(e)),
                        }
                    }

                    loading.set(false);
                });
            }
        })
    };

    let on_delete_tag = {
        let tags = tags.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |id: Uuid| {
            let tags = tags.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                if (api::delete_tag(id).await).is_ok() {
                    match api::get_tags().await {
                        Ok(data) => tags.set(data),
                        Err(e) => error.set(Some(e)),
                    }
                }

                loading.set(false);
            });
        })
    };

    let on_edit_tag = {
        let editing_tag = editing_tag.clone();
        let tag_name = tag_name.clone();
        let tag_color = tag_color.clone();
        let tags = tags.clone();

        Callback::from(move |id: Uuid| {
            if let Some(tag) = tags.iter().find(|t| t.id == id) {
                editing_tag.set(Some(id));
                tag_name.set(tag.name.clone());
                tag_color.set(tag.color.as_ref().unwrap_or(&"#3B82F6".to_string()).clone());
            }
        })
    };

    let on_cancel_edit = {
        let editing_tag = editing_tag.clone();
        let tag_name = tag_name.clone();
        let tag_color = tag_color.clone();

        Callback::from(move |_| {
            editing_tag.set(None);
            tag_name.set(String::new());
            tag_color.set(String::new());
        })
    };

    html! {
        <div class="container mx-auto p-4">
            <h1 class="text-3xl font-bold mb-6">{"Tags"}</h1>
            
            if let Some(error_msg) = error.as_ref() {
                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                    {error_msg}
                </div>
            }

            // Create/Edit tag form
            <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-6">
                <h2 class="text-xl font-semibold mb-4">
                    {if editing_tag.is_some() { "Edit Tag" } else { "Create New Tag" }}
                </h2>
                <form onsubmit={if editing_tag.is_some() { on_update_tag } else { on_create_tag }}>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                        <div>
                            <label class="block text-gray-700 text-sm font-bold mb-2" for="tag-name">
                                {"Tag Name"}
                            </label>
                            <input
                                id="tag-name"
                                type="text"
                                value={(*tag_name).clone()}
                                oninput={
                                    let tag_name = tag_name.clone();
                                    Callback::from(move |e: InputEvent| {
                                        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                            tag_name.set(input.value());
                                        }
                                    })
                                }
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                placeholder="Enter tag name"
                            />
                        </div>
                        <div>
                            <label class="block text-gray-700 text-sm font-bold mb-2" for="tag-color">
                                {"Color"}
                            </label>
                            <input
                                id="tag-color"
                                type="color"
                                value={if tag_color.is_empty() { "#3B82F6".to_string() } else { (*tag_color).clone() }}
                                oninput={
                                    let tag_color = tag_color.clone();
                                    Callback::from(move |e: InputEvent| {
                                        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                            tag_color.set(input.value());
                                        }
                                    })
                                }
                                class="shadow appearance-none border rounded w-full h-10 py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            />
                        </div>
                    </div>
                    
                    <div class="flex space-x-2">
                        <button
                            type="submit"
                            disabled={*loading}
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline disabled:opacity-50"
                        >
                            {if *loading { 
                                if editing_tag.is_some() { "Updating..." } else { "Creating..." }
                            } else if editing_tag.is_some() { "Update Tag" } else { "Create Tag" }}
                        </button>
                        
                        if editing_tag.is_some() {
                            <button
                                type="button"
                                onclick={on_cancel_edit}
                                class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            >
                                {"Cancel"}
                            </button>
                        }
                    </div>
                </form>
            </div>

            // Tags list
            <div class="bg-white shadow-md rounded">
                <div class="px-6 py-4 border-b">
                    <h2 class="text-xl font-semibold">{"All Tags"}</h2>
                </div>
                
                if tags.is_empty() && !*loading {
                    <div class="px-6 py-4 text-gray-500 text-center">
                        {"No tags found. Create your first tag above!"}
                    </div>
                } else {
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 p-6">
                        {for tags.iter().map(|tag| {
                            let tag_id = tag.id;
                            let on_delete = on_delete_tag.clone();
                            let on_edit = on_edit_tag.clone();
                            let default_color = "#6B7280".to_string();
                            let color = tag.color.as_ref().unwrap_or(&default_color);
                            
                            html! {
                                <div class="border rounded-lg p-4 hover:shadow-md transition-shadow">
                                    <div class="flex items-center justify-between mb-2">
                                        <div class="flex items-center space-x-2">
                                            <div 
                                                class="w-4 h-4 rounded-full" 
                                                style={format!("background-color: {color}")}
                                            ></div>
                                            <h3 class="text-lg font-medium text-gray-900">
                                                {&tag.name}
                                            </h3>
                                        </div>
                                        <div class="flex space-x-2">
                                            <button
                                                class="text-blue-600 hover:text-blue-800 text-sm"
                                                onclick={
                                                    Callback::from(move |_| {
                                                        on_edit.emit(tag_id);
                                                    })
                                                }
                                            >
                                                {"Edit"}
                                            </button>
                                            <button
                                                class="text-red-600 hover:text-red-800 text-sm"
                                                onclick={
                                                    Callback::from(move |_| {
                                                        on_delete.emit(tag_id);
                                                    })
                                                }
                                            >
                                                {"Delete"}
                                            </button>
                                        </div>
                                    </div>
                                    <div class="text-sm text-gray-600">
                                        {"Color: "}{tag.color.as_ref().unwrap_or(&"Default".to_string())}
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
            