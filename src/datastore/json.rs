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

mod de;
mod iter;
mod raw;
mod read;
mod ser;

// Re-export public APIs.

// Deserializer
pub use de::{
  from_reader, from_slice, from_str, Deserializer, StreamDeserializer,
};

// Serializer.
pub use ser::{
  to_string, to_string_pretty, to_vec, to_vec_pretty, to_writer,
  to_writer_pretty, CharEscape, CompactFormatter, Compound, Formatter,
  PrettyFormatter, Serializer, State,
};

// Raw dtype.
pub use raw::{
  to_raw_dtype, BorrowedRawDeserializer, OwnedRawDeserializer, RawDType, TOKEN,
};

// IO operations.
pub use read::{Fused, IoRead, Position, Read, Reference, SliceRead, StrRead};
