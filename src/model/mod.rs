use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use crate::ctx::Ctx;
use crate::model::store::{Db, new_db_pool};

mod unit_test;
mod error;
mod base;
mod store;
pub mod task;
pub mod user;

pub use self::error::{Error, Result};

#[derive(Clone)]
pub struct DbContext {
    db: Db,
}

impl DbContext {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        Ok(DbContext { db })
    }

    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}

// #[derive(Clone, Debug, Serialize)]
// pub struct Ticket {
//     pub id: u64,
//     pub cid: u64,
//     pub title: String,
// }
//
// #[derive(Deserialize)]
// pub struct TicketForCreate {
//     pub title: String,
// }

// #[derive(Clone)]
// pub struct ModelController {
//     tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
// }

// impl ModelController {
//     pub async fn new() -> Result<Self> {
//         Ok(Self {
//             tickets_store: Arc::default()
//         })
//     }
//
//     pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
//         let mut store = self.tickets_store.lock().unwrap();
//
//         let id = store.len() as u64;
//         let ticket = Ticket {
//             id,
//             cid: ctx.user_id(),
//             title: ticket_fc.title,
//         };
//         store.push(Some(ticket.clone()));
//
//         Ok(ticket)
//     }
//
//     pub async fn list_tickets(&self, ctx: Ctx) -> Result<Vec<Ticket>> {
//         let store = self.tickets_store.lock().unwrap();
//
//         let tickets = store.iter()
//             .filter_map(|t| t.clone())
//             .collect();
//
//         Ok(tickets)
//     }
//
//     pub async fn delete_ticket(&self, ctx: Ctx, id: u64) -> Result<Ticket> {
//         let mut store = self.tickets_store.lock().unwrap();
//
//         let ticket = store.get_mut(id as usize).and_then(|t| t.take());
//
//         ticket.ok_or(Error::TicketDeleteIdNotFound { id })
//     }
// }