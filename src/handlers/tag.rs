use worker::{Request, Response, RouteContext};
use super::handle_request;

pub async fn get_info(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "tag.getInfo").await
}

pub async fn get_similar(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "tag.getSimilar").await
}

pub async fn get_top_albums(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "tag.getTopAlbums").await
}

pub async fn get_top_artists(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "tag.getTopArtists").await
}

pub async fn get_top_tags(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "tag.getTopTags").await
}

pub async fn get_top_tracks(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "tag.getTopTracks").await
}

pub async fn get_weekly_chart_list(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "tag.getWeeklyChartList").await
}