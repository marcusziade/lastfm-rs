use super::handle_request;
use worker::{Request, Response, RouteContext};

pub async fn get_correction(
    req: Request,
    ctx: RouteContext<()>,
) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "artist.getCorrection").await
}

pub async fn get_info(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "artist.getInfo").await
}

pub async fn get_similar(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "artist.getSimilar").await
}

pub async fn get_top_albums(
    req: Request,
    ctx: RouteContext<()>,
) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "artist.getTopAlbums").await
}

pub async fn get_top_tags(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "artist.getTopTags").await
}

pub async fn get_top_tracks(
    req: Request,
    ctx: RouteContext<()>,
) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "artist.getTopTracks").await
}

pub async fn search(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "artist.search").await
}
