// Copyright 2021 Victor I. Afolabi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(clippy::absurd_extreme_comparisons)]

#[cfg(feature = "arbitrary_precision")]
pub(crate) const TOKEN: &str = "$sage::private::Number";

/// Represents a number, whether integer or floating point.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Number {
  /// Number enum implementation.
  n: NumImpl,
}

/// Number implementation without arbitrary precision.
#[cfg(not(feature = "arbitrary_precision"))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NumImpl {
  /// Floating point number (always finite).
  Float(f64),
  /// Greater or equal to zero.
  PositiveInt(i64),
  /// Always less than zero.
  NegativeInt(u64),
}

// Implementing Eq is fine since any float values are always finite.
#[cfg(not(feature = "arbitrary_precision"))]
impl Eq for NumImpl {}

/// Number representation with arbitrary precision.
#[cfg(feature = "arbitrary_precision")]
type NumImpl = String;

impl Number {
  /// REturns true if the `Number` is an integer between `i64::MIN` & `i64::MAX`.
  ///
  /// For any `Number` on which `is_i64` returns true, `as_i64` is guaranteed to
  /// return the integer value.
  ///
  #[inline]
  pub fn is_i64(&self) -> bool {
    match self.n {
      NumImpl::PositiveInt(n) => n <= i64::MAX,
      NumImpl::NegativeInt(_) => true,
      NumImpl::Float(_) => false,
    }
  }

  #[inline]
  pub fn is_u64(&self) -> bool {
    matches!(self.n, NumImpl::PositiveInt(_))
  }

  #[inline]
  pub fn is_f64(&self) -> bool {
    match self.n {
      NumImpl::Float(_) => true,
      NumImpl::PositiveInt(_) | NumImpl::NegativeInt(_) => false,
    }
  }
}
