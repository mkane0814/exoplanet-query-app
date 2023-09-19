use crate::model::{
    data::Data,
    input::{Input, PageKind},
};
use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{ Connection, SqliteConnection, QueryBuilder, Sqlite };

        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            use dotenvy::dotenv;
            use std::env;

            dotenv()?;
            Ok(SqliteConnection::connect(&env::var("DATABASE_URL")?).await?)
        }

        pub async fn find_records(query: Vec<Input>, anchor_id: i64, page_direction: PageKind,) -> Result<Option<Data>, ServerFnError> {
            use crate::model::data::PlanetData;

            let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new("select discoverymethod as discovery_method, releasedate as release_date, * from exoplanet_data WHERE default_flag = true");

            for input in query {
                builder.push(format!(" AND {} {} ", input.field.as_str(), input.comparison_op.as_str()));
                builder.push_bind(input.value);
            }

            match page_direction {
                PageKind::Next => {
                    builder.push(" AND id > ");
                    builder.push_bind(anchor_id);
                    builder.push(" ORDER BY id LIMIT 100;");
                    let mut conn = db().await?;
                    let planet_data = builder.build_query_as::<'_, PlanetData>().fetch_all(&mut conn).await?;

                    if let Some(last_entry) = planet_data.last() {
                        if let Some(first_entry) = planet_data.first() {
                            let last_id = last_entry.id;
                            let first_id = first_entry.id;
                            leptos::logging::log!("First ID: {}", first_id);
                            leptos::logging::log!("Last ID: {}", last_id);
                            let data = Data {
                                planet_data,
                                last_id,
                                first_id,
                            };
                            Ok(Some(data))
                        } else {
                            Ok(None)
                        }
                    } else {
                        Ok(None)
                    }
                },
                PageKind::Prev => {
                    builder.push(" AND id < ");
                    builder.push_bind(anchor_id);
                    builder.push(" ORDER BY id DESC LIMIT 100;");
                    let mut conn = db().await?;
                    let mut planet_data = builder.build_query_as::<'_, PlanetData>().fetch_all(&mut conn).await?;

                    if let Some(last_entry) = planet_data.last() {
                        if let Some(first_entry) = planet_data.first() {
                            let last_id = last_entry.id;
                            let first_id = first_entry.id;
                            leptos::logging::log!("First ID: {}", last_id);
                            leptos::logging::log!("Last ID: {}", first_id);
                            planet_data.sort_unstable_by(|a, b| a.id.cmp(&b.id));
                            let data = Data {
                                planet_data,
                                first_id: last_id,
                                last_id: first_id,
                            };
                            Ok(Some(data))
                        } else {
                            Ok(None)
                        }
                    } else {
                        Ok(None)
                    }
                },
            }
        }
    }
}

#[server(QueryDb, "/api")]
pub async fn query_db(
    query: Vec<Input>,
    anchor_id: i64,
    page_direction: PageKind,
) -> Result<Option<Data>, ServerFnError> {
    match find_records(query, anchor_id, page_direction).await {
        Ok(results) => Ok(results),
        Err(error) => {
            leptos::logging::error!("{}", error);
            Err(error)
        }
    }
}
