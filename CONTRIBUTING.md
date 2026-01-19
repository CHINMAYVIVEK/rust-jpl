# Contributing to rust-jpl

Thank you for your interest in contributing to **rust-jpl**! 🎉  
Contributions of all kinds are welcome — bug reports, feature requests, documentation improvements, and code contributions.

This project focuses on **scientific correctness, API clarity, and long-term stability**, especially due to its integration with **NASA JPL ephemeris data**.

---

## Code of Conduct

This project follows the guidelines outlined in [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).  
By participating, you agree to uphold respectful and inclusive behavior.

---

## Ways to Contribute

You can contribute by:

- Reporting bugs
- Suggesting features or improvements
- Improving documentation
- Writing tests
- Submitting pull requests

If you are unsure where to start, feel free to open an issue for discussion.

---

## Reporting Bugs

When reporting a bug, please include:

- The `rust-jpl` version
- Rust compiler version (`rustc --version`)
- Operating system and architecture
- The relevant NASA JPL ephemeris (e.g., DE441)
- Steps to reproduce the issue
- Expected vs actual behavior
- A minimal reproducible example, if possible

Please use the **Bug Report** issue template.

---

## Feature Requests

Feature requests are welcome, especially those that:

- Improve scientific accuracy
- Extend support for additional JPL ephemerides
- Improve ergonomics without sacrificing clarity
- Maintain backward compatibility

Before starting implementation:

1. Open a feature request issue
2. Describe the use case and scope
3. Discuss API design if public APIs are involved

This helps avoid wasted effort and ensures design alignment.

---

## Development Setup

### Requirements

- Rust **1.70.0** or newer (MSRV)
- Cargo
- Git

Verify your setup:

```bash
rustc --version
cargo --version
```

---

### Clone and Build

```bash
git clone https://github.com/chinmayvivek/rust-jpl.git
cd rust-jpl
cargo build
```

---

## Coding Guidelines

### Rust Style

- Follow `rustfmt`
- Code must pass `cargo clippy` without warnings
- Prefer clear, idiomatic Rust over clever code

```bash
cargo fmt
cargo clippy --all-targets --all-features
```

---

### API Design Principles

- Public APIs should be **explicit and predictable**
- Avoid breaking changes whenever possible
- All public items must be documented
- Types should encode invariants where possible
- Favor `Result` over panics

---

### Safety

- Avoid `unsafe` unless absolutely necessary
- Any use of `unsafe` must be:
  - Minimal
  - Justified
  - Documented

---

## Testing

- New features must include tests
- Bug fixes should include regression tests
- Prefer unit tests where possible
- Integration tests should be used for ephemeris validation

Run tests with:

```bash
cargo test
```

---

## Documentation

- Public APIs must have `///` doc comments
- Examples in docs should compile
- Update `README.md` when behavior changes
- Keep documentation accurate and concise

Check documentation locally:

```bash
cargo doc --open
```

---

## Commit Messages

Please use clear and descriptive commit messages:

```text
feat: add DE440 ephemeris support
fix: correct Julian date conversion edge case
docs: clarify configuration example
refactor: simplify ephemeris loader
```

---

## Pull Requests

### Before Submitting

Ensure that:

- Code builds successfully
- All tests pass
- `cargo fmt` and `cargo clippy` succeed
- Public APIs are documented
- Relevant issues are referenced

### Review Process

- All pull requests are reviewed by the maintainer
- Feedback may be requested
- Be open to discussion and revisions

As a solo-maintained project, response times may vary — thank you for your patience.

---

## Release Policy

The project follows **Semantic Versioning**.

- PATCH: bug fixes
- MINOR: backward-compatible features
- MAJOR: breaking changes

See [`RELEASE.md`](RELEASE.md) for details.

---

## License

By contributing, you agree that your contributions will be licensed under the project’s license:

- MIT OR Apache-2.0

---

## Questions or Discussions?

If you have questions or are unsure about anything:

- Open an issue
- Start a discussion

Thank you for contributing to **rust-jpl** 🚀

```

```
