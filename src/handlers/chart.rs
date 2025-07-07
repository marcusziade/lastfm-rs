use worker::{Request, Response, RouteContext};
use super::handle_request;

pub async fn get_top_artists(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "chart.getTopArtists").await
}

pub async fn get_top_tags(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "chart.getTopTags").await
}

pub async fn get_top_tracks(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "chart.getTopTracks").await
}