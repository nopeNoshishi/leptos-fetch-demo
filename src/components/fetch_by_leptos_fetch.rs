#![allow(dead_code)]

use crate::api::get_api_data;
use crate::resource::data_resource;
use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryOptions, QueryScope};
use std::time::Duration;

/// Component using Leptos Fetch
#[component]
pub fn FetchByLeptosFetch(
    selected_id: RwSignal<String>,
    fetch_count: WriteSignal<u32>,
) -> impl IntoView {
    let client: QueryClient = expect_context();

    let options = QueryOptions::default()
        .with_stale_time(Duration::from_secs(60))
        .with_gc_time(Duration::from_secs(120))
        .with_refetch_interval(Duration::from_secs(60));

    let scope = QueryScope::new(move |key| {
        fetch_count.update(|count| *count += 1);

        get_api_data(key)
    })
    .with_options(options);

    let scope_cloned = scope.clone();
    let data = client.local_resource(scope_cloned, move || selected_id.get());

    let refetch = move || {
        data.refetch();
    };

    let force_refetch = move || {
        let scope = scope.clone();
        client.invalidate_query(scope, selected_id.get_untracked());
    };

    view! {
        <Suspense fallback=|| {
            view! { <p>"Loading..."</p> }
        }>
            <div>
                <p>"Fetched Data"</p>
                {move || {
                    data.get()
                        .map(|api_data| {
                            view! {
                                <div>
                                    <p>"ID: " {api_data.id}</p>
                                    <p>"Name: " {api_data.name}</p>
                                </div>
                            }
                        })
                }}
                <div class="flex gap-x-2 py-2">
                    <button
                        class="btn"
                        on:click=move |_| {
                            refetch();
                        }
                    >
                        "Refetch"
                    </button>

                    <button
                        class="btn"
                        on:click=move |_| {
                            force_refetch();
                        }
                    >
                        "Force Refetch"
                    </button>
                </div>
            </div>
        </Suspense>
    }
}

/// Component using Leptos Fetch and [`crate::resource::WrapperLocalResource`]
#[component]
pub fn FetchByLeptosFetchV2(selected_id: RwSignal<String>) -> impl IntoView {
    let data = data_resource(selected_id.into());

    let force_refetch = move || {
        data.force_refetch();
    };

    view! {
        <Suspense fallback=|| {
            view! { <p>"Loading..."</p> }
        }>
            <div>
                <p>"Fetched Data"</p>
                {move || {
                    data.resource
                        .get()
                        .map(|api_data| {
                            match api_data {
                                Ok(data) => {
                                    view! {
                                        <div>
                                            <p>"ID: " {data.id}</p>
                                            <p>"Name: " {data.name}</p>
                                        </div>
                                    }
                                        .into_any()
                                }
                                Err(_) => view! { <p class="text-red-500">"Error"</p> }.into_any(),
                            }
                        })
                }}
                <div class="flex gap-x-2 py-2">
                    <button
                        class="btn"
                        on:click=move |_| {
                            force_refetch();
                        }
                    >
                        "Force Refetch"
                    </button>
                </div>
            </div>
        </Suspense>
    }
}
