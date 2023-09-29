pub mod buffered;
mod client;

pub use self::{
    buffered::Buffered,
    client::{Client, Error},
};
pub use reqwest;
