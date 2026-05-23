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

### 2. Adaptive Cache Behavior

For the vector cache to be stable, efficient and optimal for high through-put workloads the cache must evolve and mutate along with the desired user input. Tectonic's main priority here is therefore to enable reliable, dynamic cache behaviour:

- Eviction policies tailored to vector workloads.
- Admission policies tailored to vector workloads.
- Dynamic protection tiers based on usage metrics. 
- Hysteresis-based health monitoring and reorginization.  

#### 2.1 Eviction Strategy

Tectonic enables a wide array of pre-configured, user-customizable *Eviction Policies* designed and tailored psecifically with Vector-realted workloads in mind. The goal of these strategy implmentations is to ensure the underlying cache stores only the best and most optimal Vector entries, thereby optimising space and search efficiency. 

**Examples:**
- LIFO (Last in, First out)     -> Standard predictable eviction policy.
- FIFO (First in, First out)    -> Standard predictable eviction policy.
- LRU (Last Recently Used)      -> Standard eviction policy for storing most used entries.
- Segmented LRU                 -> Advanced policy for more in-depth handling of most used entries.
- Vector ARC                    -> Advanced Policy for data-driven handling of most used entries.

#### 2.2 Admission Strategy

As with *Eviction Policies* designed to discard least impactful entries in favour of stronger candidates, Tectonic's *Admission Strategies* attempt to better the cache contents by spotting low-immpact vector *before* entry into the application. Any candidates found to be lacking in concrete value will be discarded before even entering the cache.
This feature also provides concrete pre-configured strategy implementations that can be tailored by users for specific purposes.

**Examples:**
- Always        ->  Standard becnhmarking strategy that will ALWAYS allow entry admission.
- TwoHit        ->  Standard strategy that will only allow repeat values to enter.
- TinyLFU       ->  Advanced strategy employing probability to determine possible entry value.
- WindowLFU     ->  Advanced strategy employing probability & segmentation to determine entry value.

#### Hysteresis (FUTURE)

Over time as large amounts of distinct vectors are inserted and removed from the cache, the overall internal "health" of the cache slowly deteriorates and performance is compromised. An example of this would be that *Centroid* values no longer accurately represents the underlying *Partition* thay are cnetered arounnd, thereby leading to poor vector returns. 
To combat exactly this **Tectonic** intends to implement a commplete system *Hysteresis* mechanic to dynamically monitor internal "health" and restructure the cache when operations become weakened. This *Hysteresis* feature would utilise a performmance-optimised *K-mean Clustering* algorithm to recompute ALL internal vectors into new, highly-accurate partitions when "health" runs dangerously low - Ensuring continous "health" monitoring and cache self-healing when application becomes unstable. 

**NOTE**: The implementation feature is still experimental and will be launched later. 

---

### 3. Predictable Performance

- O(1) addressing via structured location mapping and Arena indexing
- SIMD-accelerated vector quantization 
- minimized pointer chasing  
- cache-friendly, contiguous memory layout 

---

### 4. Extensibility

Another key feature to consider was that of general user-customisation. Basically, allowing individual users to tailor critical parts of the overall vector cache to support niche, personlized scenarios without skimping out on overall performance. Each critical element of the underlying vector cache are therefore completely customizable and/or optional to include, supporting greater user freedom. 

While the primary intention behind this user-defined design choice is freedom, **Tectonic** does provide strong and reliable out-of-the-box model configurations for varying workflows focused on core aspects like *High-performance*, *Vector Simmilarity*, *Latency* etc.

Examples of Cache Extensions:

- **Pluggable eviction policies**
- **Pluggable admission policies**
- **Multiple Quantization methods**
- **Vector validation & Duplicate handling** 
- **Similarity search strategies**  
- **Scalable partition/shard architecture** 
- **Internal observability metrics**

---

## Core Components

This sections displays all major core components, their general design purpose and responsibilities to allow users an in-depth view of internal functionality and features.

### Arena (Slab)

The **Arena** is the static storage layer responsible for holding all vector entries, and acting as the primary source of *truth* in the cache layer.
- Stores full, high-precision vectors [`DimVector<D>`]
- Maintains stable memory locations
- Contiguous memory layout
- Generational ID identifiers 

---

### Repository

The **Repository** is the dynamic indexing layer that enables fast lookup and search. Utilises internal splits (*Partitions*, *Shards*, *Slots*) to minimize search space and enable efficient multithreading support.
- Organizes vectors into searchable structures.
- Limits embedding space using internal partitioning.
- Supports conncurrent search efforts via internal sharding. 

---

### Location Slab

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