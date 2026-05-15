// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::{collections::HashMap};
use crate::{error::TectonicError, utility::{typings::{Hash64, TectonicResult}, utils::UniqueID}};

// ============================================================
// GLOBAL LOCATION ARENA/SLAB
// ============================================================
// Global index-slab mapping vector IDs and vector hashes to physical cache locations.
// ---
// `LocationSlab` is the source of truth for resolving:
// - `UniqueID      => LocationEntry`
// - `vector_hash   => UniqueID         => LocationEntry`
// ---
// Location-slab supports pending entries so vectors can be inserted into the arena during
// repository bootstrap before partition/shard/slot locations exist.

#[derive(Debug)]
pub struct LocationSlab {
    storage: HashMap<UniqueID, LocationEntry>,
    hashes: HashMap<u64, UniqueID>
}

// ============================================================
// LOCATION SLAB: CONSTRUCTORS
// ============================================================

impl Default for LocationSlab {
    #[inline]
    fn default() -> Self {
        Self { 
            storage: HashMap::new(),
            hashes: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
impl LocationSlab {

    // ============================================================
    // LOCATION SLAB: INSERTION
    // ============================================================

    #[inline]
    pub fn insert_routed(
        &mut self, 
        id: UniqueID, 
        hash: Hash64,
        arena_index: usize,
        partition_index: usize,
        shard_index: usize,
        slot_index: usize,
    ) -> TectonicResult<()> {
        if self.storage.contains_key(&id) {
            return Err(TectonicError::location(
                "Location ID already exists in location slab"
            ));
        };

        if self.hashes.contains_key(&hash) {
            return Err(TectonicError::location(
                "Vector hash already exists in location slab"
            ));
        };

        let entry = LocationEntry::new_routed(
            id, 
            hash, 
            arena_index, 
            partition_index, 
            shard_index, 
            slot_index
        );

        self.storage.insert(id, entry);
        self.hashes.insert(hash, id);

        #[cfg(debug_assertions)]
        self.validate_integrity();

        Ok(())
    }

    #[inline]
    pub fn insert_pending(&mut self, id: UniqueID, hash: Hash64, arena_index: usize) -> TectonicResult<()> {
        if self.storage.contains_key(&id) {
            return Err(TectonicError::location(
                "Location ID already exists in location slab"
            ));
        };

        if self.hashes.contains_key(&hash) {
            return Err(TectonicError::location(
                "Vector hash already exists in location slab"
            ));
        };

        let entry = LocationEntry::new_pending(
            id, 
            hash, 
            arena_index
        );

        self.storage.insert(id, entry);
        self.hashes.insert(hash, id);

        #[cfg(debug_assertions)]
        self.validate_integrity();

        Ok(())
    }

    #[inline]
    pub fn complete_pending(
        &mut self, 
        id: UniqueID, 
        partition_index: usize, 
        shard_index: usize, 
        slot_index: usize
    ) -> TectonicResult<()> {
        let entry = self
            .storage
            .get_mut(&id)
            .ok_or_else(|| TectonicError::location("Pending location ID not located"))?;

        if !entry.is_pending() {
            return Err(TectonicError::location(
                "Requested location entry is non-pending"
            ));
        };

        entry.update_state(partition_index, shard_index, slot_index);

        #[cfg(debug_assertions)]
        self.validate_integrity();

        Ok(()) 
    }

    #[inline]
    pub fn remove(&mut self, id: &UniqueID) -> TectonicResult<LocationEntry> {
        let entry = self
            .storage
            .get(id)
            .ok_or_else(|| TectonicError::location("Location ID not found"))?;

        let removed_hash = self.hashes.remove(&entry.hash);

        if removed_hash != Some(*id) {
            return Err(TectonicError::inconsistent_state(
                "Location Hash index did not point to correct ID placement"
            ));
        };

        #[cfg(debug_assertions)]
        self.validate_integrity();

        Ok(entry.clone())
    }

    // ============================================================
    // LOCATION SLAB: HELPER METHODS
    // ============================================================

    #[inline]
    pub fn add_location(&mut self, id: &UniqueID, hash: u64, entry: LocationEntry) {
        self.storage.insert(*id, entry);
        self.hashes.insert(hash, *id);
    }

    #[inline]
    pub fn get_location(&self, id: &UniqueID) -> Option<&LocationEntry> {
        self.storage.get(id)
    }

    #[inline]
    pub fn get_location_mut(&mut self, id: &UniqueID) -> Option<&mut LocationEntry> {
        self.storage.get_mut(id)
    }

    #[inline]
    pub fn contains_location(&self, id: &UniqueID) -> bool {
        self.storage.contains_key(id)
    }

    #[inline]
    pub fn get_by_hash(&self, hash: u64) -> Option<&LocationEntry> {
        if let Some(found_id) = self.hashes.get(&hash) {
            self.storage.get(found_id)
        } else {
            None
        }
    }

    #[inline]
    pub fn pending_count(&self) -> usize {
        self.storage
            .values()
            .filter(|entry| entry.is_pending())
            .count()
    }

    #[inline]
    pub fn routed_count(&self) -> usize {
        self.storage
            .values()
            .filter(|entry| entry.is_routed())
            .count()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.storage.clear();
        self.hashes.clear();
    }

    // ============================================================
    // LOCATION SLAB: DEBUGGING
    // ============================================================

    #[inline]
    pub fn validate_integrity(&self) -> TectonicResult<()> {
        if self.storage.len() != self.hashes.len() {
            return Err(TectonicError::inconsistent_state(
                "Location storage and HashMap index size mismatch"
            ));
        };

        for (id, entry) in self.storage.iter() {
            if *id != entry.id {
                return Err(TectonicError::inconsistent_state(
                    "Location ID key doesn't match requested Entry ID"
                ));
            }

            let hash_value = self.hashes.get(&entry.hash).ok_or_else(|| {
                TectonicError::inconsistent_state("Location entry has misisng HashMap index")
            })?;

            if hash_value != id {
                return Err(TectonicError::location(
                    "Location HashMap index points to wrond ID value"
                ));
            };
        };

        Ok(())

    }

}


// ============================================================
// INTERNAL LOCATION ENTRY
// ============================================================
// Internal entry class used for arena/partition/shard/slot pointer storage.
// Also contains relevant information => ID & Hash.
// ---
// Location Entries store relevant information in an easily accessible mapping-based format for
// quick, high-performance retrieval. Stored values (arena/partition/shard/slot) act as individual 
// pointer-values, directing cache features to concrete final placement of related values.

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct LocationEntry {
    id: UniqueID,
    hash: Hash64,
    arena_index: usize,
    state: LocationState,
}

// ============================================================
// LOCATION ENTRY: CONSTRUCTORS
// ============================================================

#[allow(dead_code)]
impl LocationEntry {
    #[inline]
    pub fn new_pending(id: UniqueID, hash: Hash64, arena_index: usize) -> Self {
        Self { 
            id, 
            hash, 
            arena_index, 
            state: LocationState::Pending, 
        }
    }

    #[inline]
    pub fn new_routed(
        id: UniqueID, 
        hash: Hash64, 
        arena_index: usize,
        partition_index: usize,
        shard_index: usize,
        slot_index: usize,
    ) -> Self {
        Self { 
            id, 
            hash, 
            arena_index, 
            state: LocationState::Routed { 
                partition_index, 
                shard_index, 
                slot_index 
            }, 
        }
    }

    // ============================================================
    // LOCATION ENTRY: HELPER METHODS
    // ============================================================

    #[inline]
    pub fn get_id(&self) -> &UniqueID {
        &self.id
    }

    #[inline]
    pub fn get_hash(&self) -> &Hash64 {
        &self.hash
    }

    #[inline]
    pub fn get_state(&self) -> &LocationState {
        &self.state
    }

    #[inline]
    pub fn is_pending(&self) -> bool {
        matches!(self.state, LocationState::Pending)
    }

    #[inline]
    pub fn is_routed(&self) -> bool {
        matches!(self.state, LocationState::Routed { .. })
    }

    #[inline]
    pub fn get_arena(&self) -> &usize {
        &self.arena_index
    }

    #[inline]
    pub fn update_state(
        &mut self, 
        partition_index: usize, 
        shard_index: usize, 
        slot_index: usize
    ) -> TectonicResult<()> {
        if !self.is_pending() {
            return Err(TectonicError::location(
                "Cannot update non-pending LocationEntry"
            ))
        };

        self.state = LocationState::Routed { 
            partition_index, 
            shard_index, 
            slot_index 
        };

        Ok(())
    }

    #[inline]
    pub fn get_partition(&self) -> TectonicResult<usize> {
        match self.state {
            LocationState::Routed { partition_index,.. } => Ok(partition_index),
            LocationState::Pending => Err(TectonicError::location(
                "Pending location holds no concrete partition index"
            )),
        }
    }

    #[inline]
    pub fn get_shard(&self) -> TectonicResult<usize> {
        match self.state {
            LocationState::Routed { shard_index,.. } => Ok(shard_index),
            LocationState::Pending => Err(TectonicError::location(
                "Pending location holds no concrete shard index"
            )),
        }
    }

    #[inline]
    pub fn get_slot(&self) -> TectonicResult<usize> {
        match self.state {
            LocationState::Routed { slot_index,.. } => Ok(slot_index),
            LocationState::Pending => Err(TectonicError::location(
                "Pending location holds no concrete slot index"
            )),
        }
    }
}

// ============================================================
// INTERNAL LOCATION STATUS
// ============================================================
// Routing state for cached vectors.
// ---
// During bootstrapping, vectors exist in the location arena before repository routing is
// finalized. Those entries are represented as `Pending`.
// --- 
// Once centroids are initialized and the repository routes the entry into a
// partition/shard/slot, the location becomes `Routed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationState {
        Pending,
        Routed {
            partition_index: usize,
            shard_index: usize,
            slot_index: usize,
        }
    }