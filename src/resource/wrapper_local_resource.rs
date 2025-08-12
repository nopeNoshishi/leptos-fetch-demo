use crate::api::ApiData;
use crate::api::{Client, ClientError};
use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryOptions, QueryScopeLocal};
use std::sync::Arc;

pub struct WrapperLocalResource<O> {
    pub resource: LocalResource<Result<O, ClientError>>,
    force_refetch: RwSignal<usize>,
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
    pub fn new<F, Fu>(action_fn: F, query_options: QueryOptions, keyer: Signal<String>) -> Self
    where
        F: Fn(Client) -> Fu + 'static,
        Fu: Future<Output = Result<O, ClientError>> + 'static,
    {
        let query_client: QueryClient = expect_context();
        let force_refetch = RwSignal::new(0);

        let action_fn = StoredValue::new_local(Arc::new(action_fn));

        let query_scope = QueryScopeLocal::new(move |_key| {
            let client = Client::new();
            action_fn.with_value(|f| f(client))
        })
        .with_options(query_options);

        let query_scope_clone = query_scope.clone();
        Effect::new(move || {
            let query_scope_clone = query_scope_clone.clone();
            let key = keyer.get_untracked();

            force_refetch.track();
            query_client.invalidate_query(query_scope_clone, key);
        });

        Self {
            resource: query_client.local_resource(query_scope, move || keyer.get()),
            force_refetch,
        }
    }

    pub fn force_refetch(&self) {
        self.force_refetch.try_update(|v| *v += 1);
    }
}

pub fn data_resource(id: Signal<String>) -> WrapperLocalResource<ApiData> {
    WrapperLocalResource::new(
        move |client| async move { client.fetch_data(id.get()).await },
        QueryOptions::default(),
        id,
    )
}
