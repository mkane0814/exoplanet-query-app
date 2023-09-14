use crate::model::{data::Data, input::Input};
use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use futures::stream::TryStreamExt;
        use sqlx::{ Connection, SqliteConnection, QueryBuilder, Sqlite };

        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            use dotenvy::dotenv;
            use std::env;

            dotenv()?;
            Ok(SqliteConnection::connect(&env::var("DATABASE_URL")?).await?)
        }

        pub async fn find_records(mut query: Vec<Input>) -> Result<Vec<Data>, ServerFnError> {
            let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new("select * from exoplanet_data");
            if !query.is_empty() {
                builder.push(" where");
                let first = query.pop().unwrap();
                builder.push(" ".to_owned() + first.field.as_str() + " " + first.comparison_op.as_str() + " ");
                builder.push_bind(first.value);
            }

            for input in query {
                builder.push(" and ".to_owned() + input.field.as_str() + " " + input.comparison_op.as_str() + " ");
                builder.push_bind(input.value);
            }

            let mut conn = db().await?;
            let mut data = Vec::new();
            let mut rows = builder.build_query_as::<'_, Data>().fetch(&mut conn);

            while let Some(row) = rows.try_next().await? {
                data.push(row);
            }

            Ok(data)
        }
    }
}

#[server(QueryDb, "/api")]
pub async fn query_db(query: Vec<Input>) -> Result<Vec<Data>, ServerFnError> {
    match find_records(query).await {
        Ok(results) => {
            leptos::logging::log!("Found {} results!", results.len());
            Ok(results)
        }
        Err(error) => {
            leptos::logging::error!("{}", error);
            Err(error)
        }
    }
}
