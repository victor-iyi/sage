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

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
  /// Floating point number.
  Float(f64),
  /// Signed integer - with -ve & +ve values.
  PositiveInt(i64),
  /// Unsigned integer - with only positive values.
  NegativeInt(u64),
}

impl Eq for Number {}

impl Number {
  pub fn is_i64(&self) -> bool {
    match *self {
      Number::PositiveInt(n) => n <= i64::MAX,
      Number::NegativeInt(_) => true,
      Number::Float(_) => false,
    }
  }

  pub fn is_u64(&self) -> bool {
    matches!(*self, Number::PositiveInt(_))
  }
}
