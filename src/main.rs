#![allow(unused)]
mod api;
mod db;
mod error;
mod prelude;
mod service;
mod utils;

use crate::prelude::*;
use api_spec_v1::management_service_server::ManagementServiceServer as ScheduleMgmServer;
use db::mgm::{ScheduleManagement, ScheduleManagerRepo};
use db::model::{Schedule, ScheduleRunSpec, ScheduleStatus};
use tonic::transport::Server;

use service::ScheduleMgm;
use sqlx::{types::Json, PgPool};

const V1_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("scheduler_v1_descriptor");

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres").await?;

    sqlx::migrate!("db/migrations").run(&pool).await?;

    let mut s = Schedule::default();
    s.bucket = "bucket5".to_string();
    s.reference = "reference".to_string();
    s.name = "some".to_string();
    s.one_off = false;
    s.status = ScheduleStatus::INACTIVE;
    s.run_spec = sqlx::types::Json(ScheduleRunSpec::Reccurence(db::model::ReccurenceSpec {
        expr: "".to_string(),
        start: None,
        end: None,
        conditions: None,
    }));

    let mgm_repo = ScheduleManagerRepo::new(pool);

    // let ss = mgm_repo.create_schedules(vec![s]).await?;

    // let ss = db::functions::create_given_schedules(&pool, vec![s]).await?;

    // println!("{ss:?}");

    let addr = "127.0.0.1:50501".parse().unwrap();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(V1_FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let mgm = ScheduleMgm::new(mgm_repo);

    Server::builder()
        .add_service(service)
        .add_service(ScheduleMgmServer::new(mgm))
        .serve(addr)
        .await?;

    Ok(())
}
