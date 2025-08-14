#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod api;
mod components;
pub mod resource;

use components::{FetchByLeptosFetch, FetchByLeptosFetchV2, FetchByLocalResource};
use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryDevtools};

/// Example application using Leptos Fetch and LocalResource
pub fn example_app() -> impl IntoView {
    // Provide the QueryClient to the context
    let client = QueryClient::new().provide();

    // Hooks
    let (mount_lr, fetch_count_lr) = use_count_for_fetch();
    let (mount_lf, fetch_count_lf) = use_count_for_fetch();
    let (mount_lf2, fetch_count_lf2) = use_count_for_fetch();

    // Reactive value
    let selected_id = RwSignal::new("initial_id".to_string());
    let total_count =
        Signal::derive(move || fetch_count_lr.get() + fetch_count_lf.get() + fetch_count_lf2.get());

    // Provide the count signals to `FetchByLeptosFetchV2``
    provide_context(fetch_count_lf2);

    view! {
        <div class="min-h-screen bg-gray-50 text-gray-900">
            <header class="border-b bg-white">
                <div class="mx-auto max-w-6xl px-4 py-4 flex items-center justify-between">
                    <h1 class="text-xl font-semibold">"Leptos Fetch Demo"</h1>

                    <div class="flex items-center gap-2">
                        <span class="badge-outline px-3 py-1">
                            <span class="w-2 h-2 rounded-full bg-indigo-400"></span>
                            "Total Fetch: "
                            {move || total_count.get()}
                        </span>
                        <span class="badge-outline px-3 py-1">
                            "leptos only: " {move || fetch_count_lr.get()}
                        </span>
                        <span class="badge-outline px-3 py-1">
                            "leptos-fetch: " {move || fetch_count_lf.get()}
                        </span>
                        <span class="badge-outline px-3 py-1">
                            "leptos-fetch V2: " {move || fetch_count_lf2.get()}
                        </span>
                    </div>
                </div>
            </header>

            <div class="mx-auto max-w-6xl px-4 py-4">
                <div class="flex items-center gap-4">
                    <label class="text-sm font-medium text-gray-600">"Target ID"</label>
                    <select
                        class="select"

                        prop:value=Signal::derive(move || selected_id.get())
                        on:change=move |ev| {
                            selected_id.set(event_target_value(&ev));
                        }
                    >
                        <option value="initial_id">"initial_id"</option>
                        <option value="second_id">"second_id"</option>
                        <option value="third_id">"third_id"</option>
                    </select>
                    <p class="text-xs text-gray-500">
                        "Changing select will also update the fetch conditions (keys) for both panels."
                    </p>
                </div>
            </div>

            <main class="mx-auto max-w-6xl px-4 pb-10">
                <div class="grid grid-cols-1 gap-6 md:grid-cols-2">

                    <div class="card">
                        <header class="flex items-center justify-between">
                            <h2 class="text-sm font-semibold">"Leptos Only"</h2>
                            <button
                                class="rounded-md border px-3 py-1.5 text-sm hover:bg-gray-50"
                                on:click=move |_| {
                                    mount_lr.update(|v| *v = !*v);
                                }
                            >
                                {move || if mount_lr.get() { "Unmount" } else { "Mount" }}
                            </button>
                        </header>

                        <div class="p-4">
                            <p class="mb-3 text-sm text-gray-600">
                                "Fetch Count: " <strong>{move || fetch_count_lr.get()}</strong>
                            </p>

                            {move || {
                                if mount_lr.get() {
                                    view! {
                                        <FetchByLocalResource
                                            selected_id=selected_id
                                            fetch_count=fetch_count_lr.write_only()
                                        />
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <p class="text-sm text-gray-500">
                                            "LocalResource is not mounted"
                                        </p>
                                    }
                                        .into_any()
                                }
                            }}
                        </div>
                    </div>

                    <div class="card">
                        <header class="flex items-center justify-between">
                            <h2 class="text-sm font-semibold">"Leptos Fetch"</h2>
                            <button
                                class="rounded-md border px-3 py-1.5 text-sm hover:bg-gray-50"
                                on:click=move |_| {
                                    mount_lf.update(|v| *v = !*v);
                                }
                            >
                                {move || if mount_lf.get() { "Unmount" } else { "Mount" }}
                            </button>
                        </header>

                        <div class="p-4">
                            <p class="mb-3 text-sm text-gray-600">
                                "Fetch Count: " <strong>{move || fetch_count_lf.get()}</strong>
                            </p>

                            {move || {
                                if mount_lf.get() {
                                    view! {
                                        <FetchByLeptosFetch
                                            selected_id=selected_id
                                            fetch_count=fetch_count_lf.write_only()
                                        />
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <p class="text-sm text-gray-500">
                                            "leptos-fetch is not mounted"
                                        </p>
                                    }
                                        .into_any()
                                }
                            }}
                        </div>
                    </div>

                    <div class="card">
                        <header class="flex items-center justify-between">
                            <h2 class="text-sm font-semibold">"Leptos Fetch V2"</h2>
                            <button
                                class="rounded-md border px-3 py-1.5 text-sm hover:bg-gray-50"
                                on:click=move |_| {
                                    mount_lf2.update(|v| *v = !*v);
                                }
                            >
                                {move || if mount_lf2.get() { "Unmount" } else { "Mount" }}
                            </button>
                        </header>

                        <div class="p-4">
                            <p class="mb-3 text-sm text-gray-600">
                                "Fetch Count: " <strong>{move || fetch_count_lf2.get()}</strong>
                            </p>

                            {move || {
                                if mount_lf2.get() {
                                    view! { <FetchByLeptosFetchV2 selected_id=selected_id /> }
                                        .into_any()
                                } else {
                                    view! {
                                        <p class="text-sm text-gray-500">
                                            "leptos-fetch V2 is not mounted"
                                        </p>
                                    }
                                        .into_any()
                                }
                            }}
                        </div>
                    </div>
                </div>
            </main>

            <QueryDevtools client=client />
        </div>
    }
}

/// Custom hook to manage fetch count and mount state
fn use_count_for_fetch() -> (RwSignal<bool>, RwSignal<u32>) {
    let mount = RwSignal::new(false);
    let fetch_count = RwSignal::new(0);
    (mount, fetch_count)
}
