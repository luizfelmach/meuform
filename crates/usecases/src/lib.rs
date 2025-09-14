mod customer;
mod flow;
mod form;
mod submission;

pub use customer::*;
pub use flow::*;
pub use form::*;
pub use submission::*;

#[macro_export]
macro_rules! usecase {
    ($handler:expr) => {
        ::std::sync::Arc::new($handler)
    };
}
