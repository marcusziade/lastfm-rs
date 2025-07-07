use super::handle_request;
use worker::{Request, Response, RouteContext};

pub async fn get_top_artists(
    req: Request,
    ctx: RouteContext<()>,
) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "geo.getTopArtists").await
}

pub async fn get_top_tracks(
    req: Request,
    ctx: RouteContext<()>,
) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "geo.getTopTracks").await
}
