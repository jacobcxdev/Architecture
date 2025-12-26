use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq)]
pub struct Keyed<K, A> {
    pub key: K,
    pub action: A,
}

impl<K, A> Keyed<K, A> {
    pub fn new(key: K, action: A) -> Self {
        Self { key, action }
    }
}

/// A keyed collection of child `State`s.
///
/// This wrapper type exists so derive macros can reliably detect “keyed child state” fields.
///
/// - Use it when a parent reducer owns a dynamic collection of child reducers.
/// - Pair it with [`Keyed`] actions and `Effects::scope_keyed(…)`.
///
/// # Note
/// If you need multiple keyed collections of the same child action type under one parent,
/// prefer using distinct *key types* (newtype keys) so the routed action payload types differ.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyedState<K, V, Map = HashMap<K, V>>(pub Map, PhantomData<fn() -> (K, V)>);

impl<K, V, Map> KeyedState<K, V, Map> {
    pub fn new(map: Map) -> Self {
        Self(map, PhantomData)
    }
}

impl<K, V, Map> Default for KeyedState<K, V, Map>
where
    Map: Default,
{
    fn default() -> Self {
        Self(Map::default(), PhantomData)
    }
}

impl<K, V, Map> Deref for KeyedState<K, V, Map> {
    type Target = Map;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V, Map> DerefMut for KeyedState<K, V, Map> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V, Map> From<Map> for KeyedState<K, V, Map> {
    fn from(value: Map) -> Self {
        Self(value, PhantomData)
    }
}

impl<K, V, Map> KeyedState<K, V, Map>
where
    Map: KeyedMap<K, V>,
{
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        Map::get_mut(&mut self.0, key)
    }
}

pub trait KeyedMap<K, V> {
    fn get_mut<'a>(this: &'a mut Self, key: &K) -> Option<&'a mut V>;
}

impl<K, V> KeyedMap<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn get_mut<'a>(this: &'a mut Self, key: &K) -> Option<&'a mut V> {
        HashMap::get_mut(this, key)
    }
}

impl<K, V> KeyedMap<K, V> for BTreeMap<K, V>
where
    K: Ord,
{
    fn get_mut<'a>(this: &'a mut Self, key: &K) -> Option<&'a mut V> {
        BTreeMap::get_mut(this, key)
    }
}