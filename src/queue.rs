use std::sync::{RwLock};

use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{itx::{InterstellarTransaction, Interstellar}, error::NotFoundError};

static ITX_QUEUE: RwLock<Vec<InterstellarTransaction<'static, Postgres>>> = RwLock::new(Vec::new());
const MUTEX_ERROR_MESSAGE: &str = "RwLock poisoned. This should never happen.";

/// Attempts to find an Itx by the uuid.
pub fn query_queue(id: &Uuid) -> Option<usize> {
    let read_lock = ITX_QUEUE.read().expect(MUTEX_ERROR_MESSAGE);
    for (index, itx) in read_lock.iter().enumerate() {
        if id == itx.get_identifier() {
            return Some(index)
        }
    }
    None
}

/// Creates an Itx for the queue
pub fn add_to_queue(tx: Transaction<'static, Postgres>) {
    let mut write_lock = ITX_QUEUE.write().expect(MUTEX_ERROR_MESSAGE);
    write_lock.push(InterstellarTransaction::new(tx));
}

// Removes the Itx from the queue, then commits the inner tx.
pub async fn commit(id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
    let mut write_lock = ITX_QUEUE.write().expect(MUTEX_ERROR_MESSAGE);
    let index_opt = query_queue(id);
    match index_opt {
        Some(index) => match write_lock.remove(index).complete().await {
            Ok(_) => Ok(()),
            Err(error) => Err(Box::new(error)),
        },
        None => Err(Box::new(NotFoundError)),
    }
}

// Removes the Itx from the queue, then aborts the inner tx.
pub async fn abort(id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
    let mut write_lock = ITX_QUEUE.write().expect(MUTEX_ERROR_MESSAGE);
    let index_opt = query_queue(id);
    match index_opt {
        Some(index) => match write_lock.remove(index).abort().await {
            Ok(_) => Ok(()),
            Err(error) => Err(Box::new(error)),
        },
        None => Err(Box::new(NotFoundError)),
    }
}