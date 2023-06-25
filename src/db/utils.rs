use std::sync::Arc;

use crate::prelude::*;
use futures_util::Future;
use sqlx::{Database, PgPool, Postgres};

pub async fn within_transaction<'tx, R>(
    pool: &'tx Arc<PgPool>,
    f: impl FnOnce(&mut sqlx::Transaction<Postgres>) -> Result<R>,
) -> Result<R> {
    let mut tx = pool.clone().begin().await?;
    let rs = f(&mut tx)?;
    tx.commit().await?;
    Ok(rs)
}

pub async fn run_transaction<'a, R, C, F>(pool: &'a Arc<PgPool>, closure: C) -> Result<R>
where
    C: FnOnce(&mut sqlx::Transaction<Postgres>) -> F,
    F: Future<Output = Result<R>>,
{
    // start transaction
    let mut tx = pool.clone().begin().await?;
    let rs = closure(&mut tx).await?;
    tx.commit().await?;
    Ok(rs)
}
