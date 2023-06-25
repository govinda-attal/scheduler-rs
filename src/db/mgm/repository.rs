use super::ScheduleManagement;
use crate::db::model::Schedule;
use crate::db::utils::{run_transaction, within_transaction};
use crate::prelude::*;
use async_trait::async_trait;
use futures::executor;
use futures_util::Future;
use sqlx::PgPool;
use std::sync::Arc;

use create_schedules;

use super::functions::*;

pub struct ScheduleManager {
    pool: Arc<PgPool>,
}

impl ScheduleManager {
    pub fn new(pool: PgPool) -> Self {
        ScheduleManager {
            pool: Arc::new(pool),
        }
    }
}

#[async_trait]
impl ScheduleManagement for ScheduleManager {
    async fn create_schedules(&self, schedules: Vec<Schedule>) -> Result<Vec<Schedule>> {
        let mut tx = self.pool.clone().begin().await?;
        let rs = create_schedules(&mut tx, &schedules).await?;
        tx.commit().await?;
        Ok(rs)
    }
}
