use crate::api::scheduler::v1::ScheduleKeys;
use crate::prelude::*;
use crate::{api::scheduler::v1::ScheduleRec, db::model::*};
use api_spec_v1::schedule_info;
use chrono::{DateTime, Timelike, Utc};
use prost_types::Timestamp;

impl From<OccurenceSpec> for api_spec_v1::OccurenceSpec {
    fn from(spec: OccurenceSpec) -> Self {
        api_spec_v1::OccurenceSpec {
            date_time: Some(W(&spec.date_time).into()),
            conditions: spec.conditions.unwrap_or_default(),
        }
    }
}

impl From<ReccurenceSpec> for api_spec_v1::ReccurenceSpec {
    fn from(spec: ReccurenceSpec) -> Self {
        api_spec_v1::ReccurenceSpec {
            expr: spec.expr,
            start: spec.start.map(|dt| W(&dt).into()),
            end: spec.end.map(|dt| W(&dt).into()),
            conditions: spec.conditions.unwrap_or_default(),
        }
    }
}

impl From<W<&DateTime<Utc>>> for Timestamp {
    fn from(val: W<&DateTime<Utc>>) -> Self {
        let val = val.0;
        Timestamp {
            seconds: val.timestamp(),
            nanos: val.nanosecond() as i32,
        }
    }
}

impl From<ScheduleRunSpec> for schedule_info::Run {
    fn from(run: ScheduleRunSpec) -> Self {
        match run {
            ScheduleRunSpec::Occurence(spec) => schedule_info::Run::Occurence(spec.into()),
            ScheduleRunSpec::Reccurence(spec) => schedule_info::Run::Reccurence(spec.into()),
        }
    }
}

impl From<Schedule> for ScheduleRec {
    fn from(s: Schedule) -> Self {
        ScheduleRec {
            keys: Some(ScheduleKeys {
                bucket: s.bucket.clone(),
                reference: s.reference.clone(),
                name: s.name.clone(),
            }),
            info: Some(api_spec_v1::ScheduleInfo {
                name: s.name,
                one_off: s.one_off,
                job_codes: s.job_codes.unwrap_or_default(),
                dependencies: s.dependencies.unwrap_or_default(),
                run: Some(s.run_spec.0.into()),
            }),
            status: api_spec_v1::Status::from(s.status) as i32,
            action_recs: vec![],
        }
    }
}

impl From<ScheduleStatus> for api_spec_v1::Status {
    fn from(s: ScheduleStatus) -> Self {
        use api_spec_v1::Status::*;
        match s {
            ScheduleStatus::ACTIVE => Self::Active,
            ScheduleStatus::PAUSED => Self::Paused,
            ScheduleStatus::INACTIVE => Self::Inactive,
            ScheduleStatus::CLOSED => Self::Closed,
            ScheduleStatus::FAILED => Self::Failed,
        }
    }
}

impl From<W<Vec<Schedule>>> for Vec<ScheduleRec> {
    fn from(v: W<Vec<Schedule>>) -> Self {
        v.0.into_iter()
            .map(|s| -> ScheduleRec { ScheduleRec::from(s) })
            .collect()
    }
}
