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

use super::DType;

#[inline]
fn eq_i64(value: &DType, other: i64) -> bool {
  value.as_i64().map_or(false, |i| i == other)
}

#[inline]
fn eq_u64(value: &DType, other: u64) -> bool {
  value.as_u64().map_or(false, |i| i == other)
}

#[inline]
fn eq_f64(value: &DType, other: f64) -> bool {
  value.as_f64().map_or(false, |i| i == other)
}

#[inline]
fn eq_bool(value: &DType, other: bool) -> bool {
  value.as_bool().map_or(false, |i| i == other)
}

#[inline]
fn eq_str(value: &DType, other: &str) -> bool {
  value.as_str().map_or(false, |i| i == other)
}

impl PartialEq<str> for DType {
  fn eq(&self, other: &str) -> bool {
    eq_str(self, other)
  }
}

impl<'a> PartialEq<&'a str> for DType {
  fn eq(&self, other: &&str) -> bool {
    eq_str(self, *other)
  }
}

impl PartialEq<DType> for str {
  fn eq(&self, other: &DType) -> bool {
    eq_str(other, self)
  }
}

impl<'a> PartialEq<DType> for &'a str {
  fn eq(&self, other: &DType) -> bool {
    eq_str(other, *self)
  }
}

impl PartialEq<String> for DType {
  fn eq(&self, other: &String) -> bool {
    eq_str(self, other.as_str())
  }
}

impl PartialEq<DType> for String {
  fn eq(&self, other: &DType) -> bool {
    eq_str(other, self.as_str())
  }
}

macro_rules! partialeq_numeric {
  ($($eq:ident [$($ty:ty)*])*) => {
    $($(
      impl PartialEq<$ty> for DType {
        fn eq(&self, other: &$ty) -> bool {
          $eq(self, *other as _)
        }
      }

      impl PartialEq<DType> for $ty {
        fn eq(&self, other: &DType) -> bool {
          $eq(other, *self as _)
        }
      }

      impl<'a> PartialEq<$ty> for &'a DType {
        fn eq(&self, other: &$ty) -> bool {
          $eq(*self, *other as _)
        }
      }

      impl<'a> PartialEq<$ty> for &'a mut DType {
        fn eq(&self, other: &$ty) -> bool {
          $eq(*self, *other as _)
        }
      }
    )*)*
  }
}

partialeq_numeric! {
  eq_i64[i8 i16 i32 i64 isize]
  eq_u64[u8 u16 u32 u64 usize]
  eq_f64[f32 f64]
  eq_bool[bool]
}
