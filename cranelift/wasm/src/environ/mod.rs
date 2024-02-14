//! Support for configurable wasm translation.

mod dummy;
#[macro_use]
mod spec;
mod zkasm;

pub use crate::environ::dummy::{
    DummyEnvironment, DummyFuncEnvironment, DummyModuleInfo, ExpectedReachability,
};
pub use crate::environ::spec::{
    FuncEnvironment, GlobalVariable, ModuleEnvironment, TargetEnvironment,
};
pub use crate::environ::zkasm::ZkasmEnvironment;
