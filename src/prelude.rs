//! Crate Prelude
pub use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;
pub type RpcResult<T> = std::result::Result<tonic::Response<T>, tonic::Status>;
// Generic Wrapper Tuple Struct for new type pattern
// to implement external traits on external types
pub struct W<T>(pub T);

pub use std::format as f;

pub use crate::api::cmn::v1 as cmn_v1;
pub use crate::api::scheduler::v1 as api_spec_v1;
