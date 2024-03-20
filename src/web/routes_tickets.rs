use crate::{Error, Result};
use crate::ctx::Ctx;
use axum::extract::{State, Path};
use axum::{Json, Router};
use axum::routing::{post, delete, get};

use crate::model::{ModelController, Ticket, TicketForCreate};


pub fn routes(mc: ModelController) -> Router{
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}



async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    let ticket = mc.create_ticket(ctx, ticket_fc).await?;
    return Ok(Json(ticket));
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");
    let list = mc.list_tickets(ctx).await?;
    return Ok(Json(list));
}

async fn delete_ticket(State(mc): State<ModelController>, ctx: Ctx, Path(ticket_id): Path<u64>) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(ctx, ticket_id).await?;
    return Ok(Json(ticket));
}
