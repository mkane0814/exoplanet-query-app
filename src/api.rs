use leptos::*;
use leptos_meta::*;
use cfg_if::cfg_if;
use crate::model::input::Query;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use mongodb::{Client, options::ClientOptions, bson::Document};
        use crate::model::data::Data;

        pub async fn get_client() -> Result<Client, ServerFnError> {
            let uri = "mongodb://localhost:27017";
            let client_options = ClientOptions::parse(uri).await?;
            Ok(Client::with_options(client_options)?)

        }

        pub async fn find_records(query: Query) -> Result<Data, ServerFnError> {
            let client = get_client().await?; 
            let collection = client.database("exoplannetdata").collection::<Document>("data");
            Ok(Data {
                results: Vec::<Document>::new()
            })

        }
    }
}

#[server(QueryDb, "/api")]
pub async fn query_db(cx: Scope, query: Query) -> Result<(), ServerFnError> {
    Ok(())
}

