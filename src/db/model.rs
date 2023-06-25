use crate::prelude::*;
use std::default;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::{Json, Uuid};

#[derive(Clone, Debug, Serialize, Deserialize, Default, sqlx::FromRow)]
pub struct Schedule {
    pub id: Option<Uuid>,
    pub bucket: String,
    pub reference: String,
    pub name: String,
    pub one_off: bool,
    pub job_codes: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
    pub run_spec: Json<ScheduleRunSpec>,
    pub status: ScheduleStatus,
    pub created: Option<DateTime<chrono::Utc>>,
    pub last_updated: Option<DateTime<chrono::Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScheduleRunSpec {
    Occurence(OccurenceSpec),
    Reccurence(ReccurenceSpec),
}

impl Default for ScheduleRunSpec {
    fn default() -> Self {
        Self::Occurence(OccurenceSpec::default())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct OccurenceSpec {
    pub date_time: DateTime<chrono::Utc>,
    pub conditions: Option<Vec<String>>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReccurenceSpec {
    pub expr: String,
    pub start: Option<DateTime<chrono::Utc>>,
    pub end: Option<DateTime<chrono::Utc>>,
    pub conditions: Option<Vec<String>>,
}

#[derive(strum_macros::Display, Clone, Debug, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "SCHEDULE_STATUS")]
pub enum ScheduleStatus {
    #[default]
    INACTIVE,
    ACTIVE,
    PAUSED,
    CLOSED,
    FAILED,
}
