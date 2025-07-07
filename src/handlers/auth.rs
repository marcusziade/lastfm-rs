// Authentication handlers for Last.fm auth methods

use serde_json::json;
use worker::{Request, Response, RouteContext};

pub async fn get_session(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    super::handle_auth_request(req, ctx, "auth.getSession").await
}

pub async fn get_mobile_session(
    req: Request,
    ctx: RouteContext<()>,
) -> Result<Response, worker::Error> {
    super::handle_auth_request(req, ctx, "auth.getMobileSession").await
}

pub async fn get_auth_url(_req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    // Get API key from environment
    let api_key = ctx
        .env
        .secret("LASTFM_API_KEY")
        .map_err(|_| worker::Error::from("LASTFM_API_KEY not configured"))?
        .to_string();

    let callback = "http://localhost:41419/auth/callback";
    let auth_url = format!("https://www.last.fm/api/auth/?api_key={api_key}&cb={callback}");

    let response = json!({
        "auth_url": auth_url
    });

    let mut resp = Response::ok(response.to_string())?;
    resp.headers_mut().set("Content-Type", "application/json")?;
    super::add_cors_headers(resp)
}
