#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {};
}

mod object;
mod auto;
use ffi;
pub use auto::*;
pub use object::Object;

pub mod functions {
    pub use super::auto::functions::*;
}
