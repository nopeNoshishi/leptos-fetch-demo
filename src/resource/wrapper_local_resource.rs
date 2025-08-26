use crate::api::ApiData;
use crate::api::{Client, ClientError};
use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryOptions, QueryScopeLocal};
use std::sync::Arc;

/// Wrapper local resource that fetches data using [`QueryClient`]
pub struct WrapperLocalResource<O> {
    /// The local resource that holds the fetched data by [`QueryClient`]
    pub resource: LocalResource<Result<O, ClientError>>,

    /// Action to refetch the data
    refetch: Action<(), ()>,
}

impl<O> Clone for WrapperLocalResource<O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<O> Copy for WrapperLocalResource<O> {}

impl<O> WrapperLocalResource<O>
where
    O: std::fmt::Debug + Clone + 'static,
{
    /// Constructs a new [`WrapperLocalResource`]
    pub fn new<F, Fu>(action_fn: F, query_options: QueryOptions, keyer: Signal<String>) -> Self
    where
        F: Fn(Client) -> Fu + 'static,
        Fu: Future<Output = Result<O, ClientError>> + 'static,
    {
        // hooks
        let fetch_count_lf2: RwSignal<u32> = expect_context();
        let query_client: QueryClient = expect_context();

        let action_fn = Arc::new(action_fn);

        // Set up the query scope
        let query_scope = QueryScopeLocal::new(move |_key| {
            fetch_count_lf2.update(|count| *count += 1);

            let client = Client::new();
            action_fn(client)
        })
        .with_options(query_options);

        // Create a refetch action
        let query_scope_cloned = query_scope.clone();
        let refetch = Action::new_local(move |()| {
            let query_scope = query_scope_cloned.clone();
            let key = keyer.get_untracked();
            query_client.invalidate_query(query_scope, key);

            async move {}
        });

        WrapperLocalResource {
            resource: query_client.local_resource(query_scope, move || keyer.get()),
            refetch,
        }
    }

    /// Forces a refetch of the resource
    pub fn force_refetch(&self) {
        self.refetch.dispatch(());
    }
}

/// Creates a data resource for `ApiData`
pub fn data_resource(id: Signal<String>) -> WrapperLocalResource<ApiData> {
    WrapperLocalResource::new(
        move |client| async move { client.fetch_data(id.get_untracked()).await },
        QueryOptions::default(),
        id,
    )
}
