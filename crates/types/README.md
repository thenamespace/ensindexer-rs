# types

Shared ENS primitive and helper crate.

## Responsibility

`types` contains cross-crate primitives for IDs, log context, ENS name handling, scalar formatting, and shared chain constants.

## Modules

- `core`: constants, `LogContext`, namehash/labelhash helpers, event ID helpers, resolver ID formatting, DNS name decoding, and label validation.

## Architecture Notes

This crate stays dependency-light and is safe for all other crates to depend on. It centralizes compatibility-sensitive formatting such as lowercase `0x` hex IDs, official event ID shapes, wrapped batch event IDs, and bracketed unknown labels.

## Boundary Rules

- This crate should contain pure helpers and small value types only.
- This crate should not depend on storage, API, server, ingest, projection, or config.
- Public helpers should be deterministic and easy to test without external services.
- Formatting helpers should document whether they match Ethereum, ENS, or Graph Node conventions.

## Testing Approach

Unit-test every compatibility-sensitive helper: namehash, labelhash, DNS wire-name decoding, unknown label formatting, hex casing, event IDs, and resolver IDs. These tests protect downstream crates from subtle schema drift.
