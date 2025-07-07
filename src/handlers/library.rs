use super::handle_request;
use worker::{Request, Response, RouteContext};

pub async fn get_artists(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "library.getArtists").await
}
