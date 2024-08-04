#[cfg(test)]
mod tests;

mod parser;
mod result;

pub mod prelude {
    pub use crate::{parser::QueryParser, result::ParserResult};
}
