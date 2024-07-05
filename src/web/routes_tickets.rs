// use axum::extract::{FromRef, Path, State};
// use axum::{Json, Router};
// use axum::routing::{delete, get, post};
// use tracing::log::debug;
// use crate::ctx::Ctx;
//
// use crate::Error;
// use crate::model::{ModelController};
// use crate::model::{Ticket};
// use crate::model::{TicketForCreate};
//
// #[derive(Clone, FromRef)]
// struct AppState {
//     mc: ModelController,
// }
//
// pub fn routes(mc: ModelController) -> Router {
//     let app_state = AppState { mc };
//     Router::new()
//         .route("/tickets", post(create_ticket))
//         .route("/tickets", get(list_tickets))
//         .route("/tickets/:id", delete(delete_ticket))
//         .with_state(app_state)
// }
//
// async fn create_ticket(
//     State(mc): State<ModelController>,
//     ctx: Ctx,
//     Json(ticket_for_create): Json<TicketForCreate>, // Recover data from body
// ) -> Result<Json<Ticket>, Error> {
//     debug!("{:<12} - create_ticket", "HANDLER");
//
//     let ticket = mc.create_ticket(ctx, ticket_for_create).await?;
//
//     Ok(Json(ticket))
// }
//
// async fn list_tickets(
//     State(mc): State<ModelController>,
//     ctx: Ctx,
// ) -> Result<Json<Vec<Ticket>>, Error> {
//     debug!("{:<12} - list_tickets", "HANDLER");
//
//     let tickets = mc.list_tickets(ctx).await?;
//
//     Ok(Json(tickets))
// }
//
// async fn delete_ticket(
//     State(mc): State<ModelController>,
//     ctx: Ctx,
//     Path(id): Path<u64>, // recover data from path
// ) -> Result<Json<Ticket>, Error> {
//     debug!("{:<12} - delete_ticket", "HANDLER");
//
//     let ticket = mc.delete_ticket(ctx, id).await?;
//
//     Ok(Json(ticket))
// }