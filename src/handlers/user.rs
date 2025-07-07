use worker::{Request, Response, RouteContext};
use super::handle_request;

pub async fn get_friends(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getFriends").await
}

pub async fn get_info(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getInfo").await
}

pub async fn get_loved_tracks(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getLovedTracks").await
}

pub async fn get_personal_tags(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getPersonalTags").await
}

pub async fn get_recent_tracks(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getRecentTracks").await
}

pub async fn get_top_albums(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getTopAlbums").await
}

pub async fn get_top_artists(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getTopArtists").await
}

pub async fn get_top_tags(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getTopTags").await
}

pub async fn get_top_tracks(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getTopTracks").await
}

pub async fn get_weekly_album_chart(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getWeeklyAlbumChart").await
}

pub async fn get_weekly_artist_chart(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getWeeklyArtistChart").await
}

pub async fn get_weekly_chart_list(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getWeeklyChartList").await
}

pub async fn get_weekly_track_chart(req: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    handle_request(req, ctx, "user.getWeeklyTrackChart").await
}