macro_rules! impl_nostr_database {
    ($type:ty, |$database:ident| $inner:expr) => {
        impl_nostr_database!(
            @impl
            []
            $crate::database::NostrDatabase,
            $type,
            |$database| $inner
        );
    };

    (@impl [$($attr:tt)*] $trait:path, $type:ty, |$database:ident| $inner:expr) => {
        $($attr)*
        #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
        #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
        impl $trait for $type {
            fn backend(&self) -> String {
                let $database = self;
                ::nostr_database::NostrDatabase::backend($inner).to_owned()
            }

            fn features(&self) -> $crate::database::NostrDatabaseFeatures {
                let $database = self;
                ::nostr_database::NostrDatabase::features($inner).into()
            }

            async fn save_event(
                &self,
                event: ::std::sync::Arc<$crate::protocol::event::Event>,
            ) -> $crate::error::Result<
                Option<::std::sync::Arc<$crate::database::SaveEventStatus>>,
            > {
                let $database = self;
                let status = ::nostr_database::NostrDatabase::save_event($inner, &**event).await?;
                Ok(Some(::std::sync::Arc::new(status.into())))
            }

            async fn check_id(
                &self,
                event_id: ::std::sync::Arc<$crate::protocol::event::EventId>,
            ) -> $crate::error::Result<$crate::database::traits::DatabaseEventStatus> {
                let $database = self;
                Ok(::nostr_database::NostrDatabase::check_id($inner, &**event_id)
                    .await?
                    .into())
            }

            async fn event_by_id(
                &self,
                event_id: ::std::sync::Arc<$crate::protocol::event::EventId>,
            ) -> $crate::error::Result<
                Option<::std::sync::Arc<$crate::protocol::event::Event>>,
            > {
                let $database = self;
                Ok(::nostr_database::NostrDatabase::event_by_id($inner, &**event_id)
                    .await?
                    .map(|event| ::std::sync::Arc::new(event.into())))
            }

            async fn count(
                &self,
                filter: ::std::sync::Arc<$crate::protocol::filter::Filter>,
            ) -> $crate::error::Result<u64> {
                let $database = self;
                Ok(::nostr_database::NostrDatabase::count($inner, (**filter).clone()).await? as u64)
            }

            async fn query(
                &self,
                filter: ::std::sync::Arc<$crate::protocol::filter::Filter>,
            ) -> $crate::error::Result<
                Vec<::std::sync::Arc<$crate::protocol::event::Event>>,
            > {
                let $database = self;
                Ok(::nostr_database::NostrDatabase::query($inner, (**filter).clone())
                    .await?
                    .into_iter()
                    .map(|event| ::std::sync::Arc::new(event.into()))
                    .collect())
            }

            async fn delete_events(
                &self,
                filter: ::std::sync::Arc<$crate::protocol::filter::Filter>,
            ) -> $crate::error::Result<()> {
                let $database = self;
                Ok(::nostr_database::NostrDatabase::delete($inner, (**filter).clone()).await?)
            }

            async fn wipe(&self) -> $crate::error::Result<()> {
                let $database = self;
                Ok(::nostr_database::NostrDatabase::wipe($inner).await?)
            }
        }
    };
}

#[cfg(all(
    feature = "native",
    not(target_arch = "wasm32"),
    any(feature = "lmdb", feature = "ndb")
))]
macro_rules! export_nostr_database {
    ($type:ty, |$database:ident| $inner:expr) => {
        $crate::database::impl_nostr_database!(
            @impl
            [
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    uniffi::export(async_runtime = "tokio")
                )]
                #[cfg_attr(target_arch = "wasm32", uniffi::export)]
            ]
            NostrDatabase,
            $type,
            |$database| $inner
        );
    };
}

#[cfg(all(
    feature = "native",
    not(target_arch = "wasm32"),
    any(feature = "lmdb", feature = "ndb")
))]
pub(crate) use export_nostr_database;
pub(crate) use impl_nostr_database;
