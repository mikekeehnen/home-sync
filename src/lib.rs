use anyhow::Result;
use libsql_client::{Config, Client};
use spin_sdk::{
    http::{IntoResponse, Request, Response},
    http_component, variables,
};

#[http_component]
async fn handle_request(_req: Request) -> Result<impl IntoResponse> {
    let turso_url = variables::get("turso_url")?;
    let turso_auth_token = variables::get("turso_auth_token")?;
    let config = Config::new(turso_url.as_str())?.with_auth_token(&turso_auth_token);
    let connection = Client::from_config(config).await.expect("Couldn't connect to db");
    let rowset = connection.execute("SELECT 'Hello, Turso'").await.unwrap();
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(rowset.rows[0].values[0].to_string())
        .build())
}

