# Security Policy

## Supported Versions

Security updates are provided only for the **latest released version** of `rust-jpl`.

Users are encouraged to:

- Always use the most recent release
- Subscribe to release notifications

Older versions may not receive security fixes.

---

## Reporting a Vulnerability

If you discover a security vulnerability, **please do not open a public GitHub issue**.

Instead, report it **privately** to the project maintainer.

### How to Report

Please include as much detail as possible:

- A clear description of the vulnerability
- Steps to reproduce the issue
- Affected versions (if known)
- Potential impact
- Any relevant logs or proof-of-concept code

Security reports will be acknowledged as soon as possible.

---

## Response Process

Upon receiving a security report:

1. The maintainer will acknowledge receipt
2. The issue will be investigated
3. A fix will be developed and tested
4. A patched release will be published
5. A security advisory may be issued

Response times may vary, as this is a **solo-maintained project**, but all reports are taken seriously.

---

## Disclosure Policy

`rust-jpl` follows **responsible disclosure practices**.

Please allow time for:

- Investigation
- Fix development
- Coordinated disclosure

Public disclosure should occur **only after** a fix has been released or explicitly approved by the maintainer.

---

## Scope

This security policy applies to:

- The `rust-jpl` library source code
- Configuration handling
- Ephemeris file parsing and loading

It does **not** cover:

- Vulnerabilities in third-party ephemeris data provided by NASA JPL
- Issues caused by incorrect configuration or misuse

---

## Dependencies

- Dependencies are managed via Cargo
- Regular dependency updates are encouraged
- Security advisories are monitored via the RustSec Advisory Database

Users are encouraged to run:

```bash
cargo audit
```

---

## Contact

If you are unsure whether an issue is security-related, please err on the side of caution and report it privately.

Thank you for helping keep **rust-jpl** secure.
