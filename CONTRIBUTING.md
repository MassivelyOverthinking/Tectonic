# 🤝 Contributing Guide

First off — **thank you for considering contributing to Tectonic**!
All possible improvements, design suggestions and additional features are much appreciated.
Whether you're fixing a bug, improving performance, adding features, or refining documentation — your effort matters and are incredibly important to future development and extensibility. 

This project primmarily aims to construct a **high-performance vector search & caching framework** configured in Rust and exported to Python, and contributions of all sizes and levels of expertise are welcome and deeply appreciated. I'm currently somewhat "new" to high-performance Rust code implementation so any assistance here from venerated Rust developers, be it merely alternate code suggestions or optimization ideas, would be hugely appreciated 🙏

For further information regarding overall project design considerations and systems architecture, please refer to the ARCHITECTURE.md, EXAMPLES.md files or feel free to reach out to me with any additional questions or inquiries!

---

## 🌟 Ways to Contribute

You don’t need to write complex, high-level code to meaningfully contribute. Here are a few great ways anybody can help build and improve the project:

* 🐛 Reporting bugs
* 💡 Suggesting new features or possible improvements
* 🧪 Adding tests or benchmarks
* 📝 Improving project documentation
* ⚡  Perfromance Optimization
* 🔍 Reviewing Git requests

---

## 🚀 Getting Started

### 1. Fork & Clone
```bash
git git@github.com:MassivelyOverthinking/Tectonic.git
cd crates/tectonic-core
```

### 2. Create a Branch
```bash
git checkout -b feature/your-feature-name
```

Please utilise descriptive names for features:
* `feature/vector-slicing`
* `fix/arena-borrow-bug`
* `docs/cache-result`

---

## 🧰 Development Setup

### Rust
Make sure you have:
* Rust (latest stable - currently not working with Nightly)
* Cargo

```bash
rustup update
cargo build
```

### Python (if applicable)
```bash
pip install -r requirements.txt
```

---

## 🧪 Running Tests

```bash
cargo test
```

If applicable:
```bash
pytest
```

---

## 🧹 Code Style & Quality

I generally aim for **clean, readable, and maintainable code**. So please ensure that you correctly format code

### Rust
* Format your code:

  ```bash
  cargo fmt
  ```
* Run lints:
  ```bash
  cargo clippy
  ```

### General Guidelines
* I prefer clarity over cleverness
* Keep functions small, independent and focused
* Please use meaningful and easily-understandable names
* Avoid unnecessary allocations
* Please document any non-obvious decisions

---

## 🧠 Design Principles

The mian project design principles are the following:
* ⚡ Performance-aware design
* 🧩 Modular architecture
* 🔒 Safe and idiomatic Rust (Still relatively new to this)
* 📦 Clear data ownership (important for vectors & memory)
* 🔍 Deterministic and debuggable behavior

When contributing, consider:
* Does this introduce unnecessary cloning?
* Is memory usage predictable?
* Does this align with existing abstractions?

---

## 📦 Commit Guidelines

Use clear and structured commit messages:

```text
feat: add CacheResult slicing support
fix: correct arena borrowing logic
docs: improve contributing guide
refactor: simplify distance computation
```

This practice will greatly improve overall project visibility and understanding.

---

## 🔀 Pull Request Process

### Before submitting:

* ✅ Code compiles (`cargo build`)
* ✅ Tests pass (`cargo test`)
* ✅ Code is formatted (`cargo fmt`)
* ✅ Lints pass (`cargo clippy`)

### PR Checklist

* [ ] Clear description of changes
* [ ] Linked issue (if applicable)
* [ ] Tests added/updated (if needed)
* [ ] No unnecessary changes

---

## 🧾 Writing Good PRs

A good PR should answer:

* **What** does this change do?
* **Why** is it needed?
* **How** does it work?

Example:

```md
### Summary
Adds slicing support to CacheResult.

### Motivation
Allows easier inspection of top-k results.

### Changes
- Implemented Index and Deref
- Added `top(n)` helper
```

---

## 🐛 Reporting Issues

When reporting bugs, please be as informative as possible, and include necessary information:

* 📌 Description of the issue/bug
* 🔁 Steps to reproduce the issue
* 📦 Relevant code snippets (Where the bug exists)
* ⚠️ Expected vs. actual behavior 
* 🖥 Environment (OS, Rust version, etc.)

---

## 💡 Feature Requests

I encourage any ideas or improvement suggestions, so if you view an opportunity for improvement I'm all ears! When suggesting a feature please include the necessary information:

* Explain the general use case
* Describe any suggested design considerations
* Describe expected behavior
* Consider any performance implications

---

## 🧪 Performance Contributions

This project cares deeply about performance as the main goal is a high-performance framework.
If you submit optimizations please inlcude the necessary information:

* Any benchmarks if possible
* Explain trade-offs
* Avoid premature optimization

---

## 📚 Documentation

Good documentation is just as valuable as any high-performance code.
* Keep it clear and concise - No vagueness
* Add examples where helpful
* Please ensure to update relevant docs alongside code changes

---

## 🏁 Final Notes

Every contribution counts — no matter how small.

> “The best way to improve a project is to start improving it.”

Thank you for being part of this journey 🚀
