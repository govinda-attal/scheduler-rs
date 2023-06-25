use std::sync::Arc;

use crate::db::mgm::ScheduleManagement;
use crate::db::model::Schedule;
use crate::prelude::*;
use api_spec_v1::management_service_server::ManagementService;
use api_spec_v1::{
    ApplyScheduleActionRequest, ApplyScheduleActionResponse, CloseSchedulesRequest,
    CloseSchedulesResponse, RegisterSchedulesRequest, RegisterSchedulesResponse,
    RemoveScheduleActionRequest, RemoveScheduleActionResponse,
};
use async_trait::async_trait;
use sqlx::types::Json;

use crate::db;
use tonic::{transport::Server, Request, Response, Status};

pub struct BucketSchedules(pub String, pub String, pub Vec<api_spec_v1::ScheduleInfo>);
pub struct BucketScheduleRecs(pub String, pub String, pub Vec<api_spec_v1::ScheduleRec>);

pub struct ScheduleMgm<R: ScheduleManagement> {
    repo: R,
}

impl<R> ScheduleMgm<R>
where
    R: ScheduleManagement + Sync + Send,
{
    pub fn new(repo: R) -> Self {
        return ScheduleMgm { repo };
    }
}

#[async_trait]
impl<R> ManagementService for ScheduleMgm<R>
where
    R: ScheduleManagement + Sync + Send + 'static,
{
    async fn register_schedules(
        &self,
        rq: Request<RegisterSchedulesRequest>,
    ) -> RpcResult<RegisterSchedulesResponse> {
        let rq = rq.into_inner();
        let bucket_schedules =
            BucketSchedules(rq.bucket.clone(), rq.reference.clone(), rq.schedules);

        let schedules: Vec<Schedule> = bucket_schedules.try_into()?;

        let schedules = self.repo.create_schedules(schedules).await?;

        let rs = RegisterSchedulesResponse {
            bucket: rq.bucket,
            reference: rq.reference,
            schedules: W(schedules).into(),
            ..RegisterSchedulesResponse::default()
        };

        Ok(Response::new(rs))
    }

    async fn close_schedules(
        &self,
        rq: Request<CloseSchedulesRequest>,
    ) -> RpcResult<CloseSchedulesResponse> {
        let rs = CloseSchedulesResponse::default();
        Ok(Response::new(rs))
    }

    async fn apply_schedule_action(
        &self,
        rq: Request<ApplyScheduleActionRequest>,
    ) -> RpcResult<ApplyScheduleActionResponse> {
        let rs = ApplyScheduleActionResponse::default();
        Ok(Response::new(rs))
    }

    async fn remove_schedule_action(
        &self,
        rs: Request<RemoveScheduleActionRequest>,
    ) -> RpcResult<RemoveScheduleActionResponse> {
        let rs = RemoveScheduleActionResponse::default();
        Ok(Response::new(rs))
    }
}
