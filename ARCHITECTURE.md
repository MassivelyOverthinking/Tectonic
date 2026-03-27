# ᨒ Tectonic — Architecture Overview

## Overview

Tectonic is a high-performance **vector caching framework** designed for:

- low-latency similarity search
- memory-efficient storage
- adaptive cache behavior
- scalable and parallel execution

The system is built around a clear separation of concerns between **static vector storage** and **dynamic indexing/search structures**, enabling both performance and flexibility. The main design choice for the primary cache implementation is based on **IVF-PQ (Inverted File Structure)** architecture used in high-performance Vector DBs like FAISS and Milvus. [Documentation](https://milvus.io/docs/ivf-pq.md)
This design idea intends to limit approximate search space and efficiently direct queries by partitioning the main cache into distinct subsections centered around a singular centroid vector (K-Means Clustering). This along with **Quantized Vectors (Scalar Quantization)** leading the search functionality allows for swift, ressource-efficient similarity search across large portions of the embeddings space.

---

## Core Design Principles

### 1. Separation of Storage and Indexing

Tectonic separates:

- **Arena (Slab)** → authoritative storage of concrete vectors  
- **Repository** → dynamic indexing, search and lookup

This allows:

- fast mutation of search structures without moving data
- stable, contiguous memory layout for vector entries 
- flexible indexing strategies  

This seperation of vector storage was primarily intended to settle 2 main issues; **Stable & Congiguous memory layout** and **Dynamic Clustering**. 

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