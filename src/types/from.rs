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

use super::{map::Map, number::Number, DType};
use std::{borrow::Cow, iter::FromIterator};

macro_rules! from_integer {
  ($($ty:ident)*) => {
    $(
      impl From<$ty> for DType {
        fn from(n: $ty) -> Self {
          DType::Number(n.into())
        }
      }
    )*
  };
}

from_integer! {
  i8 i16 i32 i64 isize
  u8 u16 u32 u64 usize
}

impl From<f32> for DType {
  /// Convert 32-bit floating point number to `DType`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::DType;
  ///
  /// let f: f32 = 13.37;
  /// let x: DType = f.into();
  /// ```
  fn from(f: f32) -> Self {
    From::from(f as f64)
  }
}

impl From<f64> for DType {
  /// Convert 64-bit floating point number to `DType`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::DType;
  ///
  /// let f: f64 = 3.14;
  /// let x: DType = f.into();
  /// ```
  fn from(f: f64) -> Self {
    Number::from_f64(f).map_or(DType::Null, DType::Number)
  }
}

impl From<bool> for DType {
  /// Convert boolean to `DType`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use sage::DType;
  ///
  /// let b = false;
  /// let x: DType = b.into();
  /// ```
  fn from(f: bool) -> Self {
    DType::Boolean(f)
  }
}

impl From<String> for DType {
  /// Convert `String` to `DType`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use sage::DType;
  ///
  /// let s: String = "lorem".to_string();
  /// let x: DType = s.into();
  /// ```
  fn from(f: String) -> Self {
    DType::String(f)
  }
}

impl<'a> From<&'a str> for DType {
  /// Convert string to slice to `DType`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use sage::DType;
  ///
  /// let s: &str = "lorem";
  /// let x: DType = s.into();
  /// ```
  fn from(f: &str) -> Self {
    DType::String(f.to_string())
  }
}

impl<'a> From<Cow<'a, str>> for DType {
  /// Convert copy-on-write string to `DType`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use sage::DType;
  /// use std::borrow::Cow;
  ///
  /// let s: Cow<str> = Cow::Borrowed("lorem");
  /// let x: DType = s.into();
  /// ```
  ///
  /// ```rust
  /// use sage::DType;
  /// use std::borrow::Cow;
  ///
  /// let s: Cow<str> = Cow::Owned("lorem".to_string());
  /// let x: DType = s.into();
  /// ```
  fn from(f: Cow<'a, str>) -> Self {
    DType::String(f.into_owned())
  }
}

impl From<Number> for DType {
  /// Convert `Number` to `DType`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use sage::{Number, DType};
  ///
  /// let n = Number::from(7);
  /// let x: DType = n.into();
  /// ```
  fn from(f: Number) -> Self {
    DType::Number(f)
  }
}

impl From<Map<String, DType>> for DType {
  /// Convert map (with string keys) to `Dtype`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::{Map, DType};
  ///
  /// let mut m = Map::new();
  /// m.insert("Lorem".to_string(), "ipsum".into());
  /// let x: DType = m.into();
  /// ```
  fn from(f: Map<String, DType>) -> Self {
    DType::Object(f)
  }
}

impl<T: Into<DType>> From<Vec<T>> for DType {
  /// Convert a `Vec` to `DType`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use sage::DType;
  ///
  /// let v = vec!["lorem", "ipsum", "dolor"];
  /// let x: DType = v.into();
  /// ```
  fn from(f: Vec<T>) -> Self {
    DType::Array(f.into_iter().map(Into::into).collect())
  }
}

impl<'a, T: Clone + Into<DType>> From<&'a [T]> for DType {
  /// Convert a slice to `DType`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::DType;
  ///
  /// let v: &[&str] = &["lorem", "ipsum", "dolor"];
  /// let x: DType = v.into();
  /// ```
  fn from(f: &'a [T]) -> Self {
    DType::Array(f.iter().cloned().map(Into::into).collect())
  }
}

impl<T: Into<DType>> FromIterator<T> for DType {
  /// Convert an iteratable type to a `DType`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use sage::DType;
  ///
  /// let v = std::iter::repeat(42).take(5);
  /// let x: DType = v.collect();
  /// ```
  ///
  /// ```rust
  /// use sage::DType;
  ///
  /// let v: Vec<_> = vec!["lorem", "ipsum", "dolor"];
  /// let x: DType = v.into_iter().collect();
  /// ```
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use sage::DType;
  ///
  /// let x: DType = DType::from_iter(vec!["lorem", "ipsum", "dolor"]);
  /// ```
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    DType::Array(iter.into_iter().map(Into::into).collect())
  }
}

impl<K: Into<String>, V: Into<DType>> FromIterator<(K, V)> for DType {
  /// Convert an iteratable type to a `DType`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::DType;
  ///
  /// let v: Vec<_> = vec![("lorem", 5), ("ipsum", 2)];
  /// let x: DType = v.into_iter().collect();
  /// ```
  fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
    DType::Object(
      iter
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect(),
    )
  }
}

impl From<()> for DType {
  /// Convert `()` to `DType`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::DType;
  ///
  /// let u = ();
  /// let x: DType = u.into();
  /// ```
  fn from((): ()) -> Self {
    DType::Null
  }
}

// use serde_json::Value;

// impl From<Value> for DType {
//   fn from(f: Value) -> Self {
//     match f {
//       Value::Null => DType::Null,
//       Value::Array(ref a) => DType::Array(a),
//       Value::Bool(b) => DType::Boolean(b),
//     }
//   }
// }
