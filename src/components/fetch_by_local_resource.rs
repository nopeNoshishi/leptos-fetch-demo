use crate::api::get_api_data;
use leptos::prelude::*;

/// Component using only [`leptos::prelude::LocalResource`]
#[component]
pub fn FetchByLocalResource(
    selected_id: RwSignal<String>,
    fetch_count: WriteSignal<u32>,
) -> impl IntoView {
    let data = LocalResource::new(move || {
        fetch_count.update(|count| *count += 1);

        get_api_data(selected_id.get())
    });

    let refetch = move || {
        data.refetch();
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
                </div>
            </div>
        </Suspense>
    }
}
