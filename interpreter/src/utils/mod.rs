pub mod char_iter;
pub mod history_iter;

pub mod prelude {
    pub use super::char_iter::IntoCharIter;
    pub use super::history_iter::{HistoryIterContainer, IntoHistoryIter};
}
