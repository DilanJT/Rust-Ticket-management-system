use crate::{
    Result,
    model::{ModelController, Ticket, TicketForCreate},
};
use axum::{
    extract::{Path, State}, middleware, routing::{delete, get, post}, Json, Router
};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/{id}", delete(delete_ticket))
        .route_layer(middleware::from_fn(super::mw_auth::mw_require_auth))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("CREATE Ticket");

    let ticket = mc.create_ticket(ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("List tickets");

    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("Delete ticket");

    let ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}
