pub mod array_vec;
pub mod char_iter;
pub mod variant_extract;

pub mod prelude {
    pub use super::char_iter::IntoCharIter;
    pub use super::variant_extract::*;
}
