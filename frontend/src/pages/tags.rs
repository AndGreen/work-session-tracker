use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use shared::*;
use uuid::Uuid;
use crate::api;

#[function_component(Tags)]
pub fn tags() -> Html {
    let tags = use_state(|| Vec::<Tag>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let show_form = use_state(|| false);
    let editing_tag = use_state(|| None::<Tag>);

    // Form state
    let tag_name = use_state(|| String::new());
    let tag_color = use_state(|| String::new());

    // Load tags on mount
    {
        let tags = tags.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with((), move |_| {
            spawn_local(async move {
                match api::get_tags().await {
                    Ok(data) => {
                        tags.set(data);
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

    let on_create_tag = {
        let tags = tags.clone();
        let show_form = show_form.clone();
        let tag_name = tag_name.clone();
        let tag_color = tag_color.clone();
        
        Callback::from(move |_| {
            let tags = tags.clone();
            let show_form = show_form.clone();
            let name = (*tag_name).clone();
            let color = (*tag_color).clone();
            
            spawn_local(async move {
                let req = CreateTagRequest {
                    name,
                    color: if color.is_empty() { None } else { Some(color) },
                };
                
                if let Ok(_) = api::create_tag(req).await {
                    // Reload tags
                    if let Ok(data) = api::get_tags().await {
                        tags.set(data);
                    }
                    show_form.set(false);
                }
            });
        })
    };

    let on_update_tag = {
        let tags = tags.clone();
        let editing_tag = editing_tag.clone();
        let tag_name = tag_name.clone();
        let tag_color = tag_color.clone();
        
        Callback::from(move |_| {
            let tags = tags.clone();
            let editing_tag = editing_tag.clone();
            let name = (*tag_name).clone();
            let color = (*tag_color).clone();
            
            if let Some(tag) = editing_tag.as_ref() {
                let tag_id = tag.id;
                
                spawn_local(async move {
                    let req = UpdateTagRequest {
                        name: if name.is_empty() { None } else { Some(name) },
                        color: if color.is_empty() { None } else { Some(color) },
                    };
                    
                    if let Ok(_) = api::update_tag(tag_id, req).await {
                        // Reload tags
                        if let Ok(data) = api::get_tags().await {
                            tags.set(data);
                        }
                        editing_tag.set(None);
                    }
                });
            }
        })
    };

    let on_delete_tag = {
        let tags = tags.clone();
        
        Callback::from(move |id: Uuid| {
            let tags = tags.clone();
            
            spawn_local(async move {
                if let Ok(_) = api::delete_tag(id).await {
                    // Reload tags
                    if let Ok(data) = api::get_tags().await {
                        tags.set(data);
                    }
                }
            });
        })
    };

    let on_edit_tag = {
        let editing_tag = editing_tag.clone();
        let show_form = show_form.clone();
        let tag_name = tag_name.clone();
        let tag_color = tag_color.clone();
        
        Callback::from(move |tag: Tag| {
            tag_name.set(tag.name.clone());
            tag_color.set(tag.color.clone().unwrap_or_default());
            editing_tag.set(Some(tag));
            show_form.set(true);
        })
    };

    let on_cancel_form = {
        let show_form = show_form.clone();
        let editing_tag = editing_tag.clone();
        let tag_name = tag_name.clone();
        let tag_color = tag_color.clone();
        
        Callback::from(move |_| {
            show_form.set(false);
            editing_tag.set(None);
            tag_name.set(String::new());
            tag_color.set(String::new());
        })
    };

    html! {
        <div class="px-4 py-6 sm:px-0">
            <div class="sm:flex sm:items-center">
                <div class="sm:flex-auto">
                    <h1 class="text-xl font-semibold text-gray-900">{"Tags"}</h1>
                    <p class="mt-2 text-sm text-gray-700">{"Manage your session tags"}</p>
                </div>
                <div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
                    <button
                        type="button"
                        class="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto"
                        onclick={let show_form = show_form.clone(); Callback::from(move |_| show_form.set(true))}
                    >
                        {"Add Tag"}
                    </button>
                </div>
            </div>

            if *show_form {
                <div class="mt-8 bg-white shadow sm:rounded-lg">
                    <div class="px-4 py-5 sm:p-6">
                        <h3 class="text-lg leading-6 font-medium text-gray-900">
                            {if editing_tag.is_some() { "Edit Tag" } else { "Create New Tag" }}
                        </h3>
                        <div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                            <div class="sm:col-span-4">
                                <label class="block text-sm font-medium text-gray-700">{"Name"}</label>
                                <input
                                    type="text"
                                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                                    value={(*tag_name).clone()}
                                    onchange={let tag_name = tag_name.clone(); Callback::from(move |e: Event| {
                                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                        tag_name.set(input.value());
                                    })}
                                />
                            </div>
                            <div class="sm:col-span-2">
                                <label class="block text-sm font-medium text-gray-700">{"Color"}</label>
                                <input
                                    type="color"
                                    class="mt-1 block w-full h-10 rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
                                    value={(*tag_color).clone()}
                                    onchange={let tag_color = tag_color.clone(); Callback::from(move |e: Event| {
                                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                        tag_color.set(input.value());
                                    })}
                                />
                            </div>
                        </div>
                        <div class="mt-5 sm:mt-6 sm:grid sm:grid-cols-2 sm:gap-3 sm:grid-flow-row-dense">
                            <button
                                type="button"
                                class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-indigo-600 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:col-start-2 sm:text-sm"
                                onclick={if editing_tag.is_some() { on_update_tag } else { on_create_tag }}
                            >
                                {if editing_tag.is_some() { "Update Tag" } else { "Create Tag" }}
                            </button>
                            <button
                                type="button"
                                class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:col-start-1 sm:text-sm"
                                onclick={on_cancel_form}
                            >
                                {"Cancel"}
                            </button>
                        </div>
                    </div>
                </div>
            }

            <div class="mt-8">
                if *loading {
                    <div class="bg-white px-4 py-8 text-center rounded-lg shadow">
                        <p class="text-sm text-gray-500">{"Loading tags..."}</p>
                    </div>
                } else if let Some(err) = error.as_ref() {
                    <div class="bg-red-50 border border-red-200 rounded-md p-4">
                        <p class="text-sm text-red-600">{format!("Error: {}", err)}</p>
                    </div>
                } else if tags.is_empty() {
                    <div class="bg-white px-4 py-8 text-center rounded-lg shadow">
                        <p class="text-sm text-gray-500">{"No tags yet. Create your first tag!"}</p>
                    </div>
                } else {
                    <div class="bg-white shadow overflow-hidden sm:rounded-md">
                        <ul class="divide-y divide-gray-200">
                            {for tags.iter().map(|tag| {
                                let tag_id = tag.id;
                                let tag_clone = tag.clone();
                                let delete_callback = on_delete_tag.clone();
                                let edit_callback = on_edit_tag.clone();
                                
                                html! {
                                    <li class="px-6 py-4">
                                        <div class="flex items-center justify-between">
                                            <div class="flex items-center">
                                                if let Some(color) = &tag.color {
                                                    <div 
                                                        class="h-4 w-4 rounded-full mr-3 border border-gray-300"
                                                        style={format!("background-color: {}", color)}
                                                    ></div>
                                                } else {
                                                    <div class="h-4 w-4 rounded-full mr-3 bg-gray-200 border border-gray-300"></div>
                                                }
                                                <div>
                                                    <p class="text-sm font-medium text-gray-900">{&tag.name}</p>
                                                    <p class="text-sm text-gray-500">
                                                        {format!("Created: {}", tag.created_at.format("%Y-%m-%d %H:%M"))}
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="flex items-center space-x-2">
                                                <button
                                                    type="button"
                                                    class="text-indigo-600 hover:text-indigo-900 text-sm"
                                                    onclick={Callback::from(move |_| edit_callback.emit(tag_clone.clone()))}
                                                >
                                                    {"Edit"}
                                                </button>
                                                <button
                                                    type="button"
                                                    class="text-red-600 hover:text-red-900 text-sm"
                                                    onclick={Callback::from(move |_| delete_callback.emit(tag_id))}
                                                >
                                                    {"Delete"}
                                                </button>
                                            </div>
                                        </div>
                                    </li>
                                }
                            })}
                        </ul>
                    </div>
                }
            </div>
        </div>
    }
}
            