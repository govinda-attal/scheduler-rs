use crate::db::model::*;
use crate::prelude::*;
use crate::service::BucketSchedules;
use api_spec_v1::schedule_info;
use chrono::{DateTime, Utc};
use sqlx::types::Json;

impl TryFrom<Option<api_spec_v1::schedule_info::Run>> for ScheduleRunSpec {
    type Error = Error;

    fn try_from(val: Option<api_spec_v1::schedule_info::Run>) -> Result<Self> {
        let run = val.ok_or(Error::BadRquest(f!("run specification is required")))?;
        let run = match run {
            schedule_info::Run::Occurence(spec) => ScheduleRunSpec::Occurence(spec.try_into()?),
            schedule_info::Run::Reccurence(spec) => ScheduleRunSpec::Reccurence(spec.try_into()?),
            _ => todo!(),
        };
        Ok(run)
    }
}

impl TryFrom<api_spec_v1::OccurenceSpec> for OccurenceSpec {
    type Error = Error;

    fn try_from(spec: api_spec_v1::OccurenceSpec) -> Result<Self> {
        let dt = spec
            .date_time
            .ok_or(Error::BadRquest(f!("one off date_time is required")))?;
        Ok(OccurenceSpec {
            date_time: W(&dt).try_into()?,
            conditions: Some(spec.conditions),
        })
    }
}

impl TryFrom<api_spec_v1::ReccurenceSpec> for ReccurenceSpec {
    type Error = Error;

    fn try_from(spec: api_spec_v1::ReccurenceSpec) -> Result<Self> {
        let start: Option<DateTime<Utc>> = match spec.start {
            None => None,
            Some(start) => Some(W(&start).try_into()?),
        };
        let end: Option<DateTime<Utc>> = match spec.end {
            None => None,
            Some(end) => Some(W(&end).try_into()?),
        };
        Ok(ReccurenceSpec {
            expr: spec.expr.clone(),
            start,
            end,
            conditions: Some(spec.conditions),
        })
    }
}

impl TryFrom<BucketSchedules> for Vec<Schedule> {
    type Error = Error;

    fn try_from(value: BucketSchedules) -> Result<Self> {
        let BucketSchedules(bucket, reference, schedules) = value;
        let schedules = schedules
            .into_iter()
            .map(|s| -> Result<Schedule> {
                let run_spec = s.run.try_into()?;
                let s = Schedule {
                    bucket: bucket.clone(),
                    name: s.name,
                    reference: reference.clone(),
                    one_off: s.one_off,
                    status: ScheduleStatus::INACTIVE,
                    run_spec: Json(run_spec),
                    dependencies: Some(s.dependencies),
                    job_codes: Some(s.job_codes),
                    ..Default::default()
                };
                Ok(s)
            })
            .collect::<Result<Vec<Schedule>>>()?;

        Ok(schedules)
    }
}
