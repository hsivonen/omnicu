//! `icu-num-util` is one of the [`ICU4X`] components.
//!
//! It includes [`FixedDecimal`], a core API for representing numbers in a human-readable form
//! appropriate for formatting and plural rule selection. It is optimized for operations involving
//! the individual digits of a number.
//!
//! # Example
//!
//! ```
//! use icu_num_util::FixedDecimal;
//!
//! let dec = FixedDecimal::from(250)
//!     .multiplied_pow10(-2)
//!     .expect("Bounds are small");
//! assert_eq!("2.50", format!("{}", dec));
//!
//! #[derive(Debug, PartialEq)]
//! struct MagnitudeAndDigit(i16, u8);
//!
//! let digits: Vec<MagnitudeAndDigit> = dec
//!     .magnitude_range()
//!     .map(|m| MagnitudeAndDigit(m, dec.digit_at(m)))
//!     .collect();
//!
//! assert_eq!(
//!     vec![
//!         MagnitudeAndDigit(-2, 0),
//!         MagnitudeAndDigit(-1, 5),
//!         MagnitudeAndDigit(0, 2)
//!     ],
//!     digits
//! );
//! ```
//!
//! [`FixedDecimal`]: ./struct.FixedDecimal.html
//! [`ICU4X`]: https://github.com/unicode-org/icu4x

pub mod fixed_decimal;
mod uint_iterator;

pub use fixed_decimal::FixedDecimal;

#[derive(Debug, PartialEq)]
pub enum Error {
    /// The magnitude or number of digits exceeds the limit of the FixedDecimal. The highest
    /// magnitude of the most significant digit is std::i16::MAX, and the lowest magnitude of the
    /// least significant digit is std::i16::MIN.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_num_util::FixedDecimal;
    /// use icu_num_util::Error;
    ///
    /// let mut dec1 = FixedDecimal::from(123);
    /// assert_eq!(Error::Limit, dec1.multiply_pow10(std::i16::MAX).unwrap_err());
    /// ```
    Limit,
}
