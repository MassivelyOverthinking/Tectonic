# 🚀 Future Design Implementations

A general overview of concrete design considerations, systems architecture extensions and additional features to be included in future Tectonic updates. 

---

## 🚩 Milestones

### 1. First Deployment => Optimization (SIMD, Quantization, Multithreading)

* Expand **SIMD** operations to additional features for better perfromance.
    * **Centroid handling**
    * **Vector comparison**
* Expand **SIMD** functionality to include further hardware architecture standards.
* Include additional **Vector Quantization** options for storage.
    * **Product Quantization**
    * **Binary Quantization**
    * (**Possible**) **Residual Quantization**
* Extensive benchmark testing on Atomic heap integration for similarity search.
    * What performs better? Lock-free Binary Heap or current multi-Heap structure.

### 2. Second Deployment => Integration (Docker, Cloud, LLMs)

* Create and upload stable **Docker Image** to expand usability.
* Integrate Cache functionality with major **Cloud** frameworks.
    * **AWS**
    * **Azure**
* Integrate Cache functionality with widely-used **LLM** MCP or Skills.

### 3. Third Deployment => Extension (Streaming, Hysteresis, Eviction, Admission)

* Expand to include streaming of Cache results to user. 
* Include additional async functionality where performance optimal.
* Extensive Hysteresis implementation for self-healing quality (K-means Reclustering).
* Construct and test new SemAware **Admission** & **Eviction** policies.

### 4. Fourth Deployment => Optimization (Nightly, Dynamic Behaviour, Health Check)

* Expand functionality to include **Portable SIMD** (Nightly) and default fallback options.
* Add new Health-check feature to assist with optimal Cache configuration and maintenance.

### 5. Fifth Deployment => Extension (Replication, Query Pre-loader, Cold Storage)

* New **Replication** for long-term persistence to disk (**Snapshot**).
* New preloaded popular queries to extent Cache performance.
* New **Cold Storage** for long-term Vector management.

### 6. Sixth Deploymnet => Observability (Audit Trails, Visualisation)

* Expand internal auditing and debugging features to allow for better maintenance.
* (**Possible**) Small-scale web application for user visibility of Cache metrics.


