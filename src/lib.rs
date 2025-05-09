use bonfida_utils::declare_id_with_central_state;

#[doc(hidden)]
pub mod entrypoint;
#[doc(hidden)]
pub mod error;
/// Program instructions and their CPI-compatible bindings
pub mod instruction;
/// Describes the different data structres that the program uses to encode state
pub mod state;

pub mod utils;

#[doc(hidden)]
pub(crate) mod processor;

#[allow(missing_docs)]
pub mod cpi;

declare_id_with_central_state!("9JzGVN9y1BgjCWKp4nJmeUnud3FGsP1zKV4VaarqwucZ");


