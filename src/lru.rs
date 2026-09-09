use std::{collections::HashMap, hash::Hash};

/// A least-recently-used cache with a fixed capacity.
///
/// The front of the queue is the most recently used entry and the back is the
/// least recently used entry.
pub struct LruCache<K, V> {
    capacity: usize,
    table: HashMap<K, LruCacheEntry<K, V>>,
    list_head: Option<K>,
    list_tail: Option<K>,
}

struct LruCacheEntry<K, V> {
    node: LruCacheNode<K>,
    value: V,
}

#[derive(Debug, Clone)]
struct LruCacheNode<K> {
    key: Option<K>,
    prev: Option<K>,
    next: Option<K>,
}

impl<K, V> LruCache<K, V>
where
    K: Hash,
    K: Eq,
    K: Clone,
{
    /// Creates an empty cache that can hold at most `capacity` entries.
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            table: HashMap::new(),
            list_head: None,
            list_tail: None,
        }
    }

    /// Returns the value for `key`, marking the entry as recently used.
    pub fn get(&mut self, key: &K) -> Option<&V>
    where
        K: PartialEq,
    {
        let node = self.table.get_mut(key)?.node.clone();
        self.detach_node(&node);

        if self.list_head.as_ref() != Some(key) {
            let old_head = self.list_head.clone();
            {
                let entry = self.table.get_mut(key)?;
                entry.node.prev = None;
                entry.node.next = old_head.clone();
            }

            if let Some(head_key) = old_head.as_ref() {
                if let Some(head_entry) = self.table.get_mut(head_key) {
                    head_entry.node.prev = Some(key.clone());
                }
            } else {
                self.list_tail = Some(key.clone());
            }

            self.list_head = Some(key.clone());
        }

        Some(&self.table.get(key)?.value)
    }

    /// Inserts `value` for `key`, returning the previous value if the key was
    /// already present.
    pub fn put(&mut self, key: K, value: V) -> Option<V>
    where
        K: PartialEq,
    {
        let prev_entry = if let Some(entry) = self.table.remove(&key) {
            Some((entry.value, entry.node))
        } else {
            None
        };

        if let Some(prev_node) = prev_entry.as_ref().map(|e| e.1.clone()).as_ref() {
            self.detach_node(prev_node);
        }

        let node = LruCacheNode::<K> {
            key: Some(key.clone()),
            next: self.list_head.clone(),
            prev: None,
        };

        if let Some(head_key) = self.list_head.as_ref() {
            if let Some(head_entry) = self.table.get_mut(head_key) {
                head_entry.node.prev = Some(key.clone());
            }
        } else {
            self.list_tail = Some(key.clone());
        }

        self.list_head = Some(key.clone());
        self.table.insert(key, LruCacheEntry { node, value });

        if self.is_above_capacity() {
            self.evict_least_recently_used();
        }

        prev_entry.map(|e| e.0)
    }

    fn detach_node(&mut self, node: &LruCacheNode<K>) {
        if let Some(prev_key) = node.prev.as_ref() {
            if let Some(before_entry) = self.table.get_mut(prev_key) {
                before_entry.node.next = node.next.clone();

                if self.list_tail == node.key {
                    self.list_tail = before_entry.node.key.clone();
                }
            }
        }
        if let Some(next_key) = node.next.as_ref() {
            if let Some(next_entry) = self.table.get_mut(next_key) {
                next_entry.node.prev = node.prev.clone();
            }
        }
    }

    fn evict_least_recently_used(&mut self) {
        if let Some(before_tail_entry) = self
            .list_tail
            .as_ref()
            .and_then(|k| self.table.remove(k))
            .and_then(|e| e.node.prev)
            .and_then(|k| self.table.get_mut(&k))
        {
            before_tail_entry.node.next = None;
            self.list_tail = before_tail_entry.node.key.clone();
        };
    }

    pub fn is_above_capacity(&self) -> bool {
        self.len() > self.capacity
    }

    /// Returns the number of entries currently in the cache.
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Returns whether the cache contains no entries.
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::LruCache;

    #[test]
    fn a_new_cache_is_empty() {
        let cache = LruCache::<&str, i32>::new(2);

        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn put_and_get_round_trip_a_value() {
        let mut cache = LruCache::new(2);

        assert_eq!(cache.put("answer", 42), None);
        assert_eq!(cache.get(&"answer"), Some(&42));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn inserting_beyond_capacity_evicts_the_least_recently_used_entry() {
        let mut cache = LruCache::new(2);

        cache.put("first", 1);
        cache.put("second", 2);
        cache.put("third", 3);

        assert_eq!(cache.get(&"first"), None);
        assert_eq!(cache.get(&"second"), Some(&2));
        assert_eq!(cache.get(&"third"), Some(&3));
    }

    #[test]
    fn getting_an_entry_makes_it_recently_used() {
        let mut cache = LruCache::new(2);

        cache.put("first", 1);
        cache.put("second", 2);
        assert_eq!(cache.get(&"first"), Some(&1));
        cache.put("third", 3);

        assert_eq!(cache.get(&"first"), Some(&1));
        assert_eq!(cache.get(&"second"), None);
    }

    #[test]
    fn putting_an_existing_key_replaces_its_value() {
        let mut cache = LruCache::new(2);

        assert_eq!(cache.put("key", 1), None);
        assert_eq!(cache.put("key", 2), Some(1));
        assert_eq!(cache.get(&"key"), Some(&2));
        assert_eq!(cache.len(), 1);
    }
}
