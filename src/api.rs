use crate::model::{data::Data, input::Input};
use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use mongodb::{Client, options::ClientOptions, bson::Document, bson::doc};
        use futures::stream::TryStreamExt;


        pub async fn get_client() -> Result<Client, ServerFnError> {
            let uri = "mongodb://localhost:27017";
            let client_options = ClientOptions::parse(uri).await?;
            Ok(Client::with_options(client_options)?)

        }

        pub async fn find_records(query: Vec<Input>) -> Result<Vec<Data>, ServerFnError> {
            let client = get_client().await?;
            let collection = client.database("exoplannetdata-href-extract").collection::<Data>("data");

            let mut query_doc = Document::new();

            let default_flag = doc! { "$eq" : "1" };
            query_doc.insert("default_flag", default_flag);

            for input in query {
                let doc = doc! { input.comparison_op : input.value };
                query_doc.insert(input.field, doc);
            }

            let mut cursor = collection.find(query_doc, None).await?;

            let mut data = Vec::new();

            while let Some(doc) = cursor.try_next().await? {
                data.push(doc);
            }

            data.dedup();

            Ok(data)

        }
    }
}

#[server(QueryDb, "/api")]
pub async fn query_db(query: Vec<Input>) -> Result<Vec<Data>, ServerFnError> {
    match find_records(query).await {
        Ok(results) => {
            log!("Found {} results!", results.len());
            Ok(results)
        }
        Err(error) => {
            leptos::error!("{}", error);
            Err(error)
        }
    }
}
