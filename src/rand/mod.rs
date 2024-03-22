//! Pseudo-random element generation.

pub use winter_crypto::{DefaultRandomCoin as WinterRandomCoin, RandomCoin, RandomCoinError};
pub use winter_utils::Randomizable;

use crate::{Felt, FieldElement, Word, ZERO};

mod rpo;
pub use rpo::RpoRandomCoin;

mod rpx;
pub use rpx::RpxRandomCoin;

/// Pseudo-random element generator.
///
/// An instance can be used to draw, uniformly at random, base field elements as well as [Word]s.
pub trait FeltRng {
    /// Draw, uniformly at random, a base field element.
    fn draw_element(&mut self) -> Felt;

    /// Draw, uniformly at random, a [Word].
    fn draw_word(&mut self) -> Word;
}
