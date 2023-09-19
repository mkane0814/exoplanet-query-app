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

                    Ok(Data::build(planet_data))
                },
                PageKind::Prev => {
                    builder.push(" AND id < ");
                    builder.push_bind(anchor_id);
                    builder.push(" ORDER BY id DESC LIMIT 100;");
                    let mut conn = db().await?;
                    let planet_data = builder.build_query_as::<'_, PlanetData>().fetch_all(&mut conn).await?;

                    Ok(Data::build(planet_data))
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
