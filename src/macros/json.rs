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

//! Sage implements a `json!` macro -- just like that of `serde_json`.
//! However, rather than using `serde_json::Value`, we use
//! `sage::DType` to represent data.

/// Construct a [`sage::DType`] from a JSON literal.
///
/// ```rust
/// # use sage::json;
/// #
/// let value = json!({
///   "code": 200,
///   "sucess": true,
/// });
/// ```
///
/// [`sage::DType`]: struct crate::DType.html
#[macro_export(local_inner_macros)]
macro_rules! json {
  // Hide distracting implementatino details from the generated rustdoc.
  ($($json:tt)+) => {
    json_internal!($($json)+)
  }
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! json_internal {
  //
  // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
  // of the elements.
  //
  // Must be invoked as: `json_internal!(@array [] $($tt)*)`.
  //

  // Done with trailing comma.
  (@array [$($elems:expr,)*]) => {
    json_internal_vec![$($elems,)*]
  };

  // Done without trailing comma.
  (@array [$($elems:expr),*]) => {
    json_internal_vec![$($elems),*]
  };

  // Next element is `null`.
  (@array [$($elems:expr,)*] null $($rest:tt)*) => {
    json_internal!(@array [$($elems,)* json_internal!(null)] $($rest)*)
  };

  // Next element is `true`.
  (@array [$($elems:expr,)*] true $($rest:tt)*) => {
    json_internal!(@array [$($elems,)* json_internal!(true)] $($rest)*)
  };

  // Next element is `false`.
  (@array [$($elems:expr,)*] false $($rest:tt)*) => {
    json_internal!(@array [$($elems,)* json_internal!(false)] $($rest)*)
  };

  // Next element is an array.
  (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
    json_internal!(@array [$($elems,)* json_internal!([$($array)*])] $($rest)*)
  };

  // Next element is a map.
  (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
    json_internal!(@array [$($elems,)* json_internal!({$($map)*})] $($rest)*)
  };

  // Next element is an expression followed by comma.
  (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
    json_internal!(@array [$($elems,)* json_internal!($next),] $($rest)*)
  };

  // Last element is an expression with no trailing comma.
  (@array [$($elems:expr,)*] $last:expr) => {
    json_internal!(@array [$($elems,)* json_internal!($last)])
  };

  // Comma after the most recent element.
  (@array [$($elems:expr),*] , $($rest:tt)*) => {
    json_internal!(@array [$($elems,)*] $($rest)*)
  };

  // Unexpected token after most recent element.
  (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
    json_unexpected!($unexpected)
  };

  //
  // TT muncher for parsing the inside of an object {...}. Each entry is
  // inserted into the given map variable.
  //
  // Must be invoked as: `json_internal!(@object $map () ($(%tt)*) ($($tt)*))`.
  //
  // We require two copies of the input tokens so that we can match on one
  // copy and trigger errors on the other copy.
  //

  // Done.
  (@object $object:ident () () ()) => {};

  // Insert the current entry followed by trailing comma.
  (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
    let _ = $object.insert(($($key)+).into(), $value);
    json_internal!(@object $object () ($($rest)*) ($($rest)*));
  };

  // Current entry followed by unexpected token.
  (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
    json_unexpected!($unexpected);
  };

  // Insert the last entry without trailing comma.
  (@object $object:ident [$($key:tt)+] ($value:expr)) => {
    let _ = $object.insert(($($key)+).into(), $value);
  };

  // Next value is `null`.
  (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
    json_internal!(@object $object [$($key)+] (json_internal!(null)) $($rest)*);
  };

  // Next value is `true`.
  (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
    json_internal!(@object $object [$($key)+] (json_internal!(true)) $($rest)*);
  };

  // Next value is `false`.
  (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
    json_internal!(@object $object [$($key)+] (json_internal!(false)) $($rest)*);
  };

  // Next value is an array.
  (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
    json_internal!(@object $object [$($key)+] (json_internal!([$($array)*])) $($rest)*);
  };

  // Next value is a map.
  (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
    json_internal!(@object $object [$($key)+] (json_internal!({$($map)*})) $($rest)*);
  };

  // Next value is an expression followed by comma.
  (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
    json_internal!(@object $object [$($key)+] (json_internal!($value)) , $($rest)*);
  };

  // Last value is an expression with no trailing comma.
  (@object $object:ident ($($key:tt)*) (: $value:expr) $copy:tt) => {
    json_internal!(@object $object [$($key)+] (json_internal!($value)));
  };

  // Missing value for last entry. Trigger a reasonable error messsage.
  (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
    // "unexpected end of macro invocation"
    json_internal!();
  };

  // Missing colon and value for last entry. Trigger a reasonable error message.
  (@object $object:ident ($($key:tt)+) () $copy:tt) => {
    // "unexpected end of macro invocation"
    json_internal!();
  };

  // Misplaced colon. Trigger a reasonable error message.
  (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
    // Takes no arguments so "no rules expected the token `:`".
    json_unexpected!($colon);
  };

  // Found a comma inside a key. Trigger a reasonable error message.
  (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
    // Takes no arguments so "no rules expected the token `,`".
    json_unexpected!($comma);
  };

  // Key is fully parenthesized. This avoids clippy double_parens flase psositive
  // because the parenthesization may be necessary here.
  (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
    json_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
  };

  // Refuse to absorb colon token into the current key.
  (@object $object:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
    json_expect_expr_comma!($($unexpected)+);
  };

  // Munch a token into the current key.
  (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
    json_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
  };

  //
  // The main implementation.
  //
  // Must be invoked as: `json_internal!($($json)+)
  //

  (null) => {
    $crate::DType::Null
  };

  (true) => {
    $crate::DType::Boolean(true)
  };

  (false) => {
    $crate::DType::Boolean(false)
  };

  ([]) => {
    $crate::DType::Array(json_internal_vec![])
  };

  ([ $($tt:tt)+ ]) => {
    $crate::DType::Array(json_internal!(@array [] $($tt)+))
  };

  ({}) => {
    $crate::DType::Object($crate::Map::new())
  };

  ({ $($tt:tt)+ }) => {
    $crate::DType::Object({
      let mut object = $crate::Map::new();
      json_internal!(@object object () ($($tt)+) ($($tt)+));
      object
    })
  };

  // Any Serialize type: numbers, strings, struct literals, variables etc.
  // must be below every other rule.
  ($other:expr) => {
    $crate::to_dtype(&$other).unwrap()
  };
}

// The `json_internal!` macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[macro_export]
#[doc(hidden)]
macro_rules! json_internal_vec {
  ($($content:tt)*) => {
    vec![$($content)*]
  };
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_unexpected {
  () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_expect_expr_comma {
  ($e:expr , $($tt:tt)*) => {};
}
