# ᨒ Tectonic Roadmap - Complete Application Architecture

#### Complete project roadmap for Tectonic Vector caching framework designed with incremental milestones and class-based implementation strategy in mind to ensure continued overview of finished/unfinished API features. 

---

## Class Implementations

- **Cache**     (Main caching Interface)
**[]**  Query Method
**[]**  Insert Method
**[]**  Rebuild Method
**[]**  Metrics Method
**[]**  Dynamic Partitioning
**[]**  Dynamic Sharding
**[]**  Dynamic Centroid Recalculation
- **Eviction**  (Custom Eviction Policies)
- **Filters**   (Custom Membership Filters)
- **Metadata**  (Caching Metrics Interface)
- **Search**    (Custom Semantic Search Options)
- **Utility**   (Collective Utility Methods)
- **Vector**    (Main Vector Interface)
- **FFI Layer** (Safety layer between Rust & Python)

## Core Design Choices

- Partitioning & Sharding
- Max Heap for Semantic Search
- K-means clustering for Partition Balancing
- Multiple Eviction Strategies
- Multiple Semantic Seach strategies
- Multithreaded Shard Operations
- Quickk Membership Checks with Probabilistic Filters
