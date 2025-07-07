use worker::{Request, Response, RouteContext};
use super::handle_request;

pub async fn get_info(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "album.getInfo").await
}

pub async fn get_top_tags(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "album.getTopTags").await
}

pub async fn search(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "album.search").await
}