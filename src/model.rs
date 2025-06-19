//! Model layer with model-store layer

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Region - Ticket types
#[derive(Debug, Clone, Serialize)] // serialize from rust to json
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// region - model controller
#[derive(Clone)]
pub struct ModelController {
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(ModelController {
            ticket_store: Arc::default(),
        })
    }
}

// CRUD implementation
impl ModelController {
    pub async fn create_ticket(&self, ticket: TicketForCreate) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let id = store.len() as u64 + 1;
        let new_ticket = Ticket {
            id,
            title: ticket.title,
        };

        store.push(Some(new_ticket.clone()));

        Ok(new_ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();
        let tickets: Vec<Ticket> = store.iter().filter_map(|ticket| ticket.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let ticket = store.get_mut(id as usize - 1).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}
