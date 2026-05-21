# ᨒ Tectonic — Architecture Overview

## Overview

Tectonic is a high-performance **vector caching framework** designed for:

- low-latency similarity search
- memory-efficient storage
- adaptive cache behavior
- User customisation and optimisation
- scalable and parallel execution

The system is built around a clear separation of concerns between **static vector storage** and **dynamic indexing/search structures**, enabling both performance and flexibility. The main design choice for the primary cache implementation is based on **IVF-PQ (Inverted File Structure)** architecture used in high-performance Vector DBs like FAISS and Milvus. [Documentation](https://milvus.io/docs/ivf-pq.md)
This design idea intends to limit approximate search space and efficiently direct queries by partitioning the main cache into distinct subsections centered around a singular centroid vector (K-Means Clustering). This along with **Quantized Vectors (Scalar Quantization)** leading the search functionality allows for swift, ressource-efficient similarity search across large portions of the embeddings space.

---

## Core Design Principles

### 1. Separation of Storage and Indexing

Tectonic separates:

- **Arena (Slab)**      ->  Authoritative storage of Vectors & related data.
- **Repository**        ->  Dynamic indexing, search and lookup.
- **Location Slab**     ->  Middle-layer between Repository & Arena.

In order to support further self-healing functionality in the form of **Hysteresis**, while also ensuring contiguous memory layout of high-vlue vectors concrete Vectors are stored seperately in the stable **Arena** and dynamic pointers in the dynamic **Repository**. 
For swift lookup Tectonic employs a third distinct data structure - **Location Slab** - that holds dynamic records of Vector location in arena/partitions/shards/slots. 

All this allows for:

- Fast, stable mutation of search structures without moving data (**Hysteresis**).
- Stable, contiguous memory layout for vector entries.
- Flexible indexing strategies.

This seperation of vector storage was primarily intended to settle 2 main issues; **Stable & Congiguous memory layout** and **Dynamic Clustering**. 

#### 1.1 Arena/Slab

The *Arena* acts as the primary storage structure for incoming semantic vectors, and also as the main source of truth throughout the application - fx. **Vector comparison**, **Similarity search**.
Each individual entry are stored utilising a UniqueID identifier based on generational ID technique, ensuring that all vector entries possess a unique identifying value. This further more helps in quickly and reliably identifying the correct vector when compared to entries located in the *Repository*. 
To ensure dynamic indexing the *Arena* utilizes a *Free-list* structure to assign array location to incoming data. 

#### 1.2 Repository

The *Repository* acts as the dynamic storage portion of the application, storing copies of the UniqueID identifier along with **Quantized Vector Entries (Scalar Quantization)** for compacted storage and speedier similarity searches. The main idea behind the *Repository* structure is its dynamic behaviour - Allowing for mutation without moving actual vector entries and compromising contiguous mmemory layout.

The *Repository* is divided into 3 main distinct parts to lessen the overall search space:
- Partitions    -> Larger internal sections centered around a **Centroid** vector.
- Shard         -> Smaller internal sections located inside Partitions to allow for multithreading.
- Slots         -> Individual storage spaces for repository-entries.

The **Centroid** vectors allow the application to swiftly divide the embedding search space into smaller more manageable sections, and then allow for multiple search-threads to begin exploring the inner Shards for candidate entries (This is accomplished by using **Min Heap** structures).

#### 1.3 Location Slab

The *Location Slab* acts as the intermidiary layer between *Arena* and *Repository*, answering the question: "If I retreive a Vector entry from Arena, how would I know exactly where it is located in the Repository and vice versa?" This complimentary structure allows for optimal O(1) lookup functionality across both structures withouth unnecessary overhead. 

Example of LocationEntry struct:
```rust
pub struct Locationentry {
    id: UniqueId,
    arena_index: usize,
    partition_index: usize,
    shard_index: usize,
    slot_index: usize,
}
```
---

### 2. Predictable Performance

- O(1) addressing via structured location mapping and Arena indexing
- SIMD-accelerated vector quantization 
- minimized pointer chasing  
- cache-friendly, contiguous memory layout  

---

### 3. Adaptive Cache Behavior

- eviction policies tailored to vector workloads  
- dynamic protection tiers based on usage metrics  
- hysteresis-based health monitoring  
- future support for structural reorganization  

---

### 4. Extensibility

- pluggable eviction policies  
- configurable validation and duplicate handling  
- modular search strategies  
- scalable partition/shard architecture  

---

## Core Components

### Arena (Slab)

The **Arena** is the static storage layer responsible for holding all vector entries.

#### Responsibilities

- stores full, high-precision vectors (`DimVector<D>`)  
- maintains stable memory locations  
- owns entry lifecycle and metadata  
- ensures memory locality and efficient access  

#### Characteristics

- append-friendly with slot reuse  
- generational slot support (optional)  
- no search logic  
- optimized for read-heavy workloads  

---

### Repository

The **Repository** is the dynamic indexing layer that enables fast lookup and search.

#### Responsibilities

- maps vector hashes → storage locations  
- organizes vectors into searchable structures  
- reduces search space using partitioning  
- supports concurrent access via sharding  

---

## Repository Structure

### Partitions

Partitions divide the vector space into coarse regions.

#### Purpose

- reduce search scope  
- group semantically similar vectors  
- enable scalable indexing  

#### Mechanism

- centroid-based assignment  
- incoming vectors mapped to closest partition  
- query routing via centroid proximity  

---

### Shards

Each partition contains multiple **shards**.

#### Purpose

- enable multi-threaded access  
- reduce lock contention  
- improve parallel search throughput  

#### Characteristics

- independent substructures  
- local indexing within partition  
- optimized for concurrent reads/writes  

---

### RepoLocation

The bridge between Repository and Arena:

```rust
pub struct RepoLocation {
    pub partition_idx: usize,
    pub shard_idx: usize,
    pub slot_idx: usize,
}
```