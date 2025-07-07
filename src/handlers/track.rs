use super::handle_request;
use worker::{Request, Response, RouteContext};

pub async fn get_correction(
    req: Request,
    ctx: RouteContext<()>,
) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "track.getCorrection").await
}

pub async fn get_info(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "track.getInfo").await
}

pub async fn get_similar(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "track.getSimilar").await
}

pub async fn get_top_tags(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "track.getTopTags").await
}

pub async fn search(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "track.search").await
}
