// Authentication handlers for Last.fm auth methods

use worker::{Request, Response, RouteContext};

pub async fn get_session(
    req: Request,
    ctx: RouteContext<()>,
) -> Result<Response, worker::Error> {
    super::handle_auth_request(req, ctx, "auth.getSession").await
}

pub async fn get_mobile_session(
    req: Request,
    ctx: RouteContext<()>,
) -> Result<Response, worker::Error> {
    super::handle_auth_request(req, ctx, "auth.getMobileSession").await
}