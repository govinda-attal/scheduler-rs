use super::model::Schedule;
use crate::prelude::*;
use async_trait::async_trait;

pub mod functions;
pub mod repository;

pub use repository::ScheduleManager as ScheduleManagerRepo;

#[async_trait]
pub trait ScheduleManagement {
    async fn create_schedules(&self, schdules: Vec<Schedule>) -> Result<Vec<Schedule>>;
}
