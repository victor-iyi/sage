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

//! A map of String to `sage::DType`.
//!
//! By default the map is backed by a [`BTreeMap`]. Enable the
//! `preserve_order` feature of sage to use [`IndexMap`] instead.
//!
//! [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
//! [`IndexMap`]: https://docs.rs/indexmap/*/indexmap/map/struct.IndexMap.html

use super::DType;
use std::iter::FusedIterator;
use std::ops;
use std::{
  borrow::Borrow,
  collections::btree_map::{self, BTreeMap},
  fmt,
  hash::Hash,
  iter::FromIterator,
};

#[cfg(feature = "preserve_order")]
use indexmap::{self, IndexMap};
use serde::de;

/// A key/value type representation.
pub struct Map<K, V> {
  map: MapImpl<K, V>,
}

#[cfg(not(feature = "preserve_order"))]
type MapImpl<K, V> = BTreeMap<K, V>;
#[cfg(feature = "preserve_order")]
type MapImpl<K, V> = IndexMap<K, V>;

impl Map<String, DType> {
  /// Makes a new empty Map.
  #[inline]
  pub fn new() -> Self {
    Map {
      map: MapImpl::new(),
    }
  }

  /// Makes a new empty Map the given initial capacity.
  #[inline]
  pub fn with_capacity(capacity: usize) -> Self {
    Map {
      #[cfg(not(feature = "preserve_order"))]
      map: {
        // does not support with_capacity
        let _ = capacity;
        BTreeMap::new()
      },
      #[cfg(feature = "preserve_order")]
      map: IndexMap::with_capacity(capacity),
    }
  }

  /// Clears the map, removing all values.
  #[inline]
  pub fn clear(&mut self) {
    self.map.clear();
  }

  /// Returns a reference to the value corresponding to the key.
  ///
  /// The key may be any borrowed form of the map's key type, but the ordering
  /// on the borrowed form *must* match the ordering on the key type.
  #[inline]
  pub fn get<Q>(&self, key: &Q) -> Option<&DType>
  where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash,
  {
    self.map.get(key)
  }

  /// Returns true if the map contains a value for the specific key.
  ///
  /// The key may be any borrowed form of the map's key type, but the ordering
  /// on the borrowed form *must* match the ordering on the key type.
  #[inline]
  pub fn contains_key<Q>(&self, key: &Q) -> bool
  where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash,
  {
    self.map.contains_key(key)
  }

  /// Returns a mutable reference to the value corresponding to the key.
  ///
  /// The key may be any borrowed form of the map's key type, but the ordering
  /// on the borrwed form *must* match the ordering on the key type.
  #[inline]
  pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut DType>
  where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash,
  {
    self.map.get_mut(key)
  }

  /// Inserts a key-value pair into the map.
  ///
  /// If the map did not have this key present, `None` is returned.
  ///
  /// If the map did have this key present, the value is updated, and the
  /// old value is returned.
  #[inline]
  pub fn insert(&mut self, k: String, v: DType) -> Option<DType> {
    self.map.insert(k, v)
  }

  /// Removes a key from the map, returning the value at the key if the key
  /// was previously in the map.
  ///
  /// The key may be any borrowed form of the map's key type, but the ordering
  /// on the borrowed form *must* match the ordering on the key type.
  pub fn remove<Q>(&mut self, key: &Q) -> Option<DType>
  where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash,
  {
    #[cfg(not(feature = "preserve_order"))]
    return self.map.remove(key);
    #[cfg(feature = "preserve_order")]
    return self.map.swap_remove(key);
  }

  /// Removes a key from the map, returning the stored key and value if the
  /// key was previously in the map.
  ///
  /// The key may be any borrowed form of the map's key type, but the ordering
  /// on the borrowed form *must* match the ordering on the key type.
  pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, DType)>
  where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash,
  {
    #[cfg(any(feature = "preserve_order", not(no_btreemap_remove_entry)))]
    return self.map.remove_entry(key);
    #[cfg(all(
      not(feature = "preserve_order"),
      no_btreemap_remove_entry,
      not(no_btreemap_get_key_value),
    ))]
    {
      let (key, _value) = self.map.get_key_value(key)?;
      let key = key.clone();
      let value = self.map.remove::<String>(&key)?;
      Some((key, value))
    }
    #[cfg(all(
      not(feature = "preserve_order"),
      no_btreemap_remove_entry,
      no_btreemap_get_key_value,
    ))]
    {
      struct Key<'a, Q: ?Sized>(&'a Q);

      impl<'a, Q: ?Sized> RangeBounds<Q> for Key<'a, Q> {
        fn start_bound(&self) -> Bound<&Q> {
          Bound::Included(self.0)
        }
        fn end_bound(&self) -> Bound<&Q> {
          Bound::Included(self.0)
        }
      }

      let mut range = self.map.range(Key(key));
      let (key, _value) = range.next()?;
      let key = key.clone();
      let value = self.map.remove::<String>(&key)?;
      Some((key, value))
    }
  }

  /// Moves all elements from other into Self, leaving other empty.
  #[inline]
  pub fn append(&mut self, other: &mut Self) {
    #[cfg(feature = "preserve_order")]
    for (k, v) in std::mem::replace(&mut other.map, MapImpl::default()) {
      self.map.insert(k, v);
    }
    #[cfg(not(feature = "preserve_order"))]
    self.map.append(&mut other.map);
  }

  /// Gets the given key's corresponding entry in the map for in-place
  /// manipulation.
  pub fn entry<S>(&mut self, key: S) -> Entry
  where
    S: Into<String>,
  {
    #[cfg(feature = "preserve_order")]
    use indexmap::map::Entry as EntryImpl;
    #[cfg(not(feature = "preserve_order"))]
    use std::collections::btree_map::Entry as EntryImpl;

    match self.map.entry(key.into()) {
      EntryImpl::Vacant(vacant) => Entry::Vacant(VacantEntry { vacant }),
      EntryImpl::Occupied(occupied) => {
        Entry::Occupied(OccupiedEntry { occupied })
      }
    }
  }

  /// Returns the number of elements in the map.
  #[inline]
  pub fn len(&self) -> usize {
    self.map.len()
  }

  /// Returns true if the map contains no elements.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.map.is_empty()
  }

  /// Gets an iterator over the entries of the map.
  #[inline]
  pub fn iter(&self) -> Iter {
    Iter {
      iter: self.map.iter(),
    }
  }

  /// Gets a mutable iterator over the entries of the map.
  #[inline]
  pub fn iter_mut(&mut self) -> IterMut {
    IterMut {
      iter: self.map.iter_mut(),
    }
  }

  /// Gets an iterator over the keys of the map.
  #[inline]
  pub fn keys(&self) -> Keys {
    Keys {
      iter: self.map.keys(),
    }
  }

  /// Gets an iterator over the values of the map.
  #[inline]
  pub fn values(&self) -> Values {
    Values {
      iter: self.map.values(),
    }
  }

  /// Gets an iterator over mutable values of the map.
  #[inline]
  pub fn values_mut(&mut self) -> ValuesMut {
    ValuesMut {
      iter: self.map.values_mut(),
    }
  }
}

impl Default for Map<String, DType> {
  #[inline]
  fn default() -> Self {
    Map {
      map: MapImpl::new(),
    }
  }
}

impl Clone for Map<String, DType> {
  #[inline]
  fn clone(&self) -> Self {
    Map {
      map: self.map.clone(),
    }
  }
}

impl PartialEq for Map<String, DType> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.map.eq(&other.map)
  }
}

impl Eq for Map<String, DType> {}

/// Access an element of this map. Panics if the given key is not present in the
/// map.
///
/// ```
/// # use sage::DType;
/// #
/// # let val = &DType::String("".to_owned());
/// # let _ =
/// match *val {
///     DType::String(ref s) => Some(s.as_str()),
///     DType::Array(ref arr) => arr[0].as_str(),
///     DType::Object(ref map) => map["type"].as_str(),
///     _ => None,
/// }
/// # ;
/// ```
impl<'a, Q> ops::Index<&'a Q> for Map<String, DType>
where
  String: Borrow<Q>,
  Q: ?Sized + Ord + Eq + Hash,
{
  type Output = DType;

  fn index(&self, index: &Q) -> &DType {
    self.map.index(index)
  }
}

/// Mutably access an element of this map. Panics if the given key is not
/// present in the map.
///
/// ```
/// # use sage::json;
/// #
/// # let mut map = sage::Map::new();
/// # map.insert("key".to_owned(), sage::DType::Null);
/// #
/// map["key"] = json!("value");
/// ```
impl<'a, Q> ops::IndexMut<&'a Q> for Map<String, DType>
where
  String: Borrow<Q>,
  Q: ?Sized + Ord + Eq + Hash,
{
  fn index_mut(&mut self, index: &Q) -> &mut DType {
    self.map.get_mut(index).expect("no entry found for key")
  }
}

impl fmt::Debug for Map<String, DType> {
  #[inline]
  fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    self.map.fmt(formatter)
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl serde::ser::Serialize for Map<String, DType> {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    use serde::ser::SerializeMap;
    let mut map = tri!(serializer.serialize_map(Some(self.len())));
    for (k, v) in self {
      tri!(map.serialize_entry(k, v));
    }
    map.end()
  }
}

impl<'de> de::Deserialize<'de> for Map<String, DType> {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
      type Value = Map<String, DType>;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
      }

      #[inline]
      fn visit_unit<E>(self) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Map::new())
      }

      #[cfg(any(feature = "std", feature = "alloc"))]
      #[inline]
      fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
      where
        V: de::MapAccess<'de>,
      {
        let mut values = Map::new();

        while let Some((key, value)) = tri!(visitor.next_entry()) {
          values.insert(key, value);
        }

        Ok(values)
      }
    }

    deserializer.deserialize_map(Visitor)
  }
}

impl FromIterator<(String, DType)> for Map<String, DType> {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = (String, DType)>,
  {
    Map {
      map: FromIterator::from_iter(iter),
    }
  }
}

impl Extend<(String, DType)> for Map<String, DType> {
  fn extend<T>(&mut self, iter: T)
  where
    T: IntoIterator<Item = (String, DType)>,
  {
    self.map.extend(iter);
  }
}

macro_rules! delegate_iterator {
    (($name:ident $($generics:tt)*) => $item:ty) => {
        impl $($generics)* Iterator for $name $($generics)* {
            type Item = $item;
            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }
            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.iter.size_hint()
            }
        }

        impl $($generics)* DoubleEndedIterator for $name $($generics)* {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.iter.next_back()
            }
        }

        impl $($generics)* ExactSizeIterator for $name $($generics)* {
            #[inline]
            fn len(&self) -> usize {
                self.iter.len()
            }
        }

        impl $($generics)* FusedIterator for $name $($generics)* {}
    }
}

/// A view into a single entry in a map, which may either be vacant or occupied.
/// This enum is constructed from the [`entry`] method on [`Map`].
///
/// [`entry`]: struct.Map.html#method.entry
/// [`Map`]: struct.Map.html
pub enum Entry<'a> {
  /// A vacant Entry.
  Vacant(VacantEntry<'a>),
  /// An occupied Entry.
  Occupied(OccupiedEntry<'a>),
}

/// A vacant Entry. It is part of the [`Entry`] enum.
///
/// [`Entry`]: enum.Entry.html
pub struct VacantEntry<'a> {
  vacant: VacantEntryImpl<'a>,
}

/// An occupied Entry. It is part of the [`Entry`] enum.
///
/// [`Entry`]: enum.Entry.html
pub struct OccupiedEntry<'a> {
  occupied: OccupiedEntryImpl<'a>,
}

#[cfg(not(feature = "preserve_order"))]
type VacantEntryImpl<'a> = btree_map::VacantEntry<'a, String, DType>;
#[cfg(feature = "preserve_order")]
type VacantEntryImpl<'a> = indexmap::map::VacantEntry<'a, String, DType>;

#[cfg(not(feature = "preserve_order"))]
type OccupiedEntryImpl<'a> = btree_map::OccupiedEntry<'a, String, DType>;
#[cfg(feature = "preserve_order")]
type OccupiedEntryImpl<'a> = indexmap::map::OccupiedEntry<'a, String, DType>;

impl<'a> Entry<'a> {
  /// Returns a reference to this entry's key.
  ///
  /// # Examples
  ///
  /// ```
  /// let mut map = sage::Map::new();
  /// assert_eq!(map.entry("sage").key(), &"sage");
  /// ```
  pub fn key(&self) -> &String {
    match *self {
      Entry::Vacant(ref e) => e.key(),
      Entry::Occupied(ref e) => e.key(),
    }
  }

  /// Ensures a value is in the entry by inserting the default if empty, and
  /// returns a mutable reference to the value in the entry.
  ///
  /// # Examples
  ///
  /// ```
  /// # use sage::json;
  /// #
  /// let mut map = sage::Map::new();
  /// map.entry("sage").or_insert(json!(12));
  ///
  /// assert_eq!(map["sage"], 12);
  /// ```
  pub fn or_insert(self, default: DType) -> &'a mut DType {
    match self {
      Entry::Vacant(entry) => entry.insert(default),
      Entry::Occupied(entry) => entry.into_mut(),
    }
  }

  /// Ensures a value is in the entry by inserting the result of the default
  /// function if empty, and returns a mutable reference to the value in the
  /// entry.
  ///
  /// # Examples
  ///
  /// ```
  /// # use sage::json;
  /// #
  /// let mut map = sage::Map::new();
  /// map.entry("sage").or_insert_with(|| json!("hoho"));
  ///
  /// assert_eq!(map["sage"], "hoho".to_owned());
  /// ```
  pub fn or_insert_with<F>(self, default: F) -> &'a mut DType
  where
    F: FnOnce() -> DType,
  {
    match self {
      Entry::Vacant(entry) => entry.insert(default()),
      Entry::Occupied(entry) => entry.into_mut(),
    }
  }

  /// Provides in-place mutable access to an occupied entry before any
  /// potential inserts into the map.
  ///
  /// # Examples
  ///
  /// ```
  /// # use sage::json;
  /// #
  /// let mut map = sage::Map::new();
  /// map.entry("sage")
  ///     .and_modify(|e| *e = json!("rust"))
  ///     .or_insert(json!("cpp"));
  ///
  /// assert_eq!(map["sage"], "cpp");
  ///
  /// map.entry("sage")
  ///     .and_modify(|e| *e = json!("rust"))
  ///     .or_insert(json!("cpp"));
  ///
  /// assert_eq!(map["sage"], "rust");
  /// ```
  pub fn and_modify<F>(self, f: F) -> Self
  where
    F: FnOnce(&mut DType),
  {
    match self {
      Entry::Occupied(mut entry) => {
        f(entry.get_mut());
        Entry::Occupied(entry)
      }
      Entry::Vacant(entry) => Entry::Vacant(entry),
    }
  }
}

impl<'a> VacantEntry<'a> {
  /// Gets a reference to the key that would be used when inserting a value
  /// through the VacantEntry.
  ///
  /// # Examples
  ///
  /// ```
  /// use sage::map::Entry;
  ///
  /// let mut map = sage::Map::new();
  ///
  /// match map.entry("sage") {
  ///     Entry::Vacant(vacant) => {
  ///         assert_eq!(vacant.key(), &"sage");
  ///     }
  ///     Entry::Occupied(_) => unimplemented!(),
  /// }
  /// ```
  #[inline]
  pub fn key(&self) -> &String {
    self.vacant.key()
  }

  /// Sets the value of the entry with the VacantEntry's key, and returns a
  /// mutable reference to it.
  ///
  /// # Examples
  ///
  /// ```
  /// # use sage::json;
  /// #
  /// use sage::map::Entry;
  ///
  /// let mut map = sage::Map::new();
  ///
  /// match map.entry("sage") {
  ///     Entry::Vacant(vacant) => {
  ///         vacant.insert(json!("hoho"));
  ///     }
  ///     Entry::Occupied(_) => unimplemented!(),
  /// }
  /// ```
  #[inline]
  pub fn insert(self, value: DType) -> &'a mut DType {
    self.vacant.insert(value)
  }
}

impl<'a> OccupiedEntry<'a> {
  /// Gets a reference to the key in the entry.
  ///
  /// # Examples
  ///
  /// ```
  /// # use sage::json;
  /// #
  /// use sage::map::Entry;
  ///
  /// let mut map = sage::Map::new();
  /// map.insert("sage".to_owned(), json!(12));
  ///
  /// match map.entry("sage") {
  ///     Entry::Occupied(occupied) => {
  ///         assert_eq!(occupied.key(), &"sage");
  ///     }
  ///     Entry::Vacant(_) => unimplemented!(),
  /// }
  /// ```
  #[inline]
  pub fn key(&self) -> &String {
    self.occupied.key()
  }

  /// Gets a reference to the value in the entry.
  ///
  /// # Examples
  ///
  /// ```
  /// # use sage::json;
  /// #
  /// use sage::map::Entry;
  ///
  /// let mut map = sage::Map::new();
  /// map.insert("sage".to_owned(), json!(12));
  ///
  /// match map.entry("sage") {
  ///     Entry::Occupied(occupied) => {
  ///         assert_eq!(occupied.get(), 12);
  ///     }
  ///     Entry::Vacant(_) => unimplemented!(),
  /// }
  /// ```
  #[inline]
  pub fn get(&self) -> &DType {
    self.occupied.get()
  }

  /// Gets a mutable reference to the value in the entry.
  ///
  /// # Examples
  ///
  /// ```
  /// # use sage::json;
  /// #
  /// use sage::map::Entry;
  ///
  /// let mut map = sage::Map::new();
  /// map.insert("sage".to_owned(), json!([1, 2, 3]));
  ///
  /// match map.entry("sage") {
  ///     Entry::Occupied(mut occupied) => {
  ///         occupied.get_mut().as_array_mut().unwrap().push(json!(4));
  ///     }
  ///     Entry::Vacant(_) => unimplemented!(),
  /// }
  ///
  /// assert_eq!(map["sage"].as_array().unwrap().len(), 4);
  /// ```
  #[inline]
  pub fn get_mut(&mut self) -> &mut DType {
    self.occupied.get_mut()
  }

  /// Converts the entry into a mutable reference to its value.
  ///
  /// # Examples
  ///
  /// ```
  /// # use sage::json;
  /// #
  /// use sage::map::Entry;
  ///
  /// let mut map = sage::Map::new();
  /// map.insert("sage".to_owned(), json!([1, 2, 3]));
  ///
  /// match map.entry("sage") {
  ///     Entry::Occupied(mut occupied) => {
  ///         occupied.into_mut().as_array_mut().unwrap().push(json!(4));
  ///     }
  ///     Entry::Vacant(_) => unimplemented!(),
  /// }
  ///
  /// assert_eq!(map["sage"].as_array().unwrap().len(), 4);
  /// ```
  #[inline]
  pub fn into_mut(self) -> &'a mut DType {
    self.occupied.into_mut()
  }

  /// Sets the value of the entry with the `OccupiedEntry`'s key, and returns
  /// the entry's old value.
  ///
  /// # Examples
  ///
  /// ```
  /// # use sage::json;
  /// #
  /// use sage::map::Entry;
  ///
  /// let mut map = sage::Map::new();
  /// map.insert("sage".to_owned(), json!(12));
  ///
  /// match map.entry("sage") {
  ///     Entry::Occupied(mut occupied) => {
  ///         assert_eq!(occupied.insert(json!(13)), 12);
  ///         assert_eq!(occupied.get(), 13);
  ///     }
  ///     Entry::Vacant(_) => unimplemented!(),
  /// }
  /// ```
  #[inline]
  pub fn insert(&mut self, value: DType) -> DType {
    self.occupied.insert(value)
  }

  /// Takes the value of the entry out of the map, and returns it.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// use sage::map::Entry;
  ///
  /// let mut map = sage::Map::new();
  /// map.insert("sage".to_owned(), json!(12));
  ///
  /// match map.entry("sage") {
  ///     Entry::Occupied(occupied) => {
  ///         assert_eq!(occupied.remove(), 12);
  ///     }
  ///     Entry::Vacant(_) => unimplemented!(),
  /// }
  /// ```
  #[inline]
  pub fn remove(self) -> DType {
    #[cfg(feature = "preserve_order")]
    return self.occupied.swap_remove();
    #[cfg(not(feature = "preserve_order"))]
    return self.occupied.remove();
  }
}

impl<'a> IntoIterator for &'a Map<String, DType> {
  type Item = (&'a String, &'a DType);
  type IntoIter = Iter<'a>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    Iter {
      iter: self.map.iter(),
    }
  }
}

/// An iterator over a `sage::Map`'s entries.
pub struct Iter<'a> {
  iter: IterImpl<'a>,
}

#[cfg(not(feature = "preserve_order"))]
type IterImpl<'a> = btree_map::Iter<'a, String, DType>;
#[cfg(feature = "preserve_order")]
type IterImpl<'a> = indexmap::map::Iter<'a, String, DType>;

delegate_iterator!((Iter<'a>) => (&'a String, &'a DType));

impl<'a> IntoIterator for &'a mut Map<String, DType> {
  type Item = (&'a String, &'a mut DType);
  type IntoIter = IterMut<'a>;
  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    IterMut {
      iter: self.map.iter_mut(),
    }
  }
}

/// A mutable iterator over a sage::Map's entries.
pub struct IterMut<'a> {
  iter: IterMutImpl<'a>,
}

#[cfg(not(feature = "preserve_order"))]
type IterMutImpl<'a> = btree_map::IterMut<'a, String, DType>;
#[cfg(feature = "preserve_order")]
type IterMutImpl<'a> = indexmap::map::IterMut<'a, String, DType>;

delegate_iterator!((IterMut<'a>) => (&'a String, &'a mut DType));

impl IntoIterator for Map<String, DType> {
  type Item = (String, DType);
  type IntoIter = IntoIter;
  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    IntoIter {
      iter: self.map.into_iter(),
    }
  }
}

/// An owning iterator over a sage::Map's entries.
pub struct IntoIter {
  iter: IntoIterImpl,
}

#[cfg(not(feature = "preserve_order"))]
type IntoIterImpl = btree_map::IntoIter<String, DType>;
#[cfg(feature = "preserve_order")]
type IntoIterImpl = indexmap::map::IntoIter<String, DType>;

delegate_iterator!((IntoIter) => (String, DType));

/// An iterator over a sage::Map's keys.
pub struct Keys<'a> {
  iter: KeysImpl<'a>,
}

#[cfg(not(feature = "preserve_order"))]
type KeysImpl<'a> = btree_map::Keys<'a, String, DType>;
#[cfg(feature = "preserve_order")]
type KeysImpl<'a> = indexmap::map::Keys<'a, String, DType>;

delegate_iterator!((Keys<'a>) => &'a String);

/// An iterator over a sage::Map's values.
pub struct Values<'a> {
  iter: ValuesImpl<'a>,
}

#[cfg(not(feature = "preserve_order"))]
type ValuesImpl<'a> = btree_map::Values<'a, String, DType>;
#[cfg(feature = "preserve_order")]
type ValuesImpl<'a> = indexmap::map::Values<'a, String, DType>;

delegate_iterator!((Values<'a>) => &'a DType);

//////////////////////////////////////////////////////////////////////////////

/// A mutable iterator over a sage::Map's values.
pub struct ValuesMut<'a> {
  iter: ValuesMutImpl<'a>,
}

#[cfg(not(feature = "preserve_order"))]
type ValuesMutImpl<'a> = btree_map::ValuesMut<'a, String, DType>;
#[cfg(feature = "preserve_order")]
type ValuesMutImpl<'a> = indexmap::map::ValuesMut<'a, String, DType>;

delegate_iterator!((ValuesMut<'a>) => &'a mut DType);
