use crate::db::model::{Schedule, ScheduleStatus};
use crate::prelude::*;
use futures::executor;
use sqlx::{postgres::PgRow, PgPool, Postgres, Transaction};

pub async fn create_schedules(
    tx: &mut Transaction<'_, Postgres>,
    schedules: &Vec<Schedule>,
) -> Result<Vec<Schedule>> {
    let mut rs_schedules = vec![];
    for s in schedules {
        let row = sqlx::query_as::<_, Schedule>(
            r#"INSERT INTO SCHEDULES
            (BUCKET, REFERENCE, NAME, ONE_OFF, RUN_SPEC, STATUS)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *"#,
        )
        .bind(&s.bucket)
        .bind(&s.reference)
        .bind(&s.name)
        .bind(&s.one_off)
        .bind(&s.run_spec)
        .bind(&s.status)
        .fetch_one(&mut *tx)
        .await?;

        rs_schedules.push(row);
    }
    Ok(rs_schedules)
}
