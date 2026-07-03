# Changelog

All notable changes to the Bluefin Pro SDK are documented here.

## [TypeScript 3.0.0 / Python 2.0.0 / Rust 2.0.0] - 2026-07-03

### Changed — BREAKING

- **`PUT /api/v1/account/preferences` write model replaced with a strict allow-list** (BFP-4644).
  The `UpdateAccountPreferenceRequest` body was rewritten to match the server's
  new allow-list schema (a mass-assignment security fix). The request now accepts
  only these optional fields, and any unknown field is rejected by the server with
  a `400` naming the offending field:
  - `favorites: string[]` — favorite market symbols (e.g. `["BTC-PERP","SUI-PERP"]`).
    Full array each time; to remove a favorite, send the array without it.
  - `functionBarMode: "all" | "Popular" | "Favorites"` — note the mixed casing
    (`all` is lowercase; `Popular` and `Favorites` are capitalized).
  - `onboardingCompleted: boolean`
  - `termsAccepted: boolean`

  The write is a **full replace** — always send the complete object; the server
  overwrites the stored preferences. A successful call returns `204 No Content`
  (no response body).

  **Removed from the write model:** `language`, `theme`, and `market`. Any code
  still sending these (or any other extra key) through this endpoint will now
  receive a `400`. The serializer no longer emits unknown/extra keys.

### Unchanged

- `GET /api/v1/account/preferences` (`AccountPreference`) is intentionally left
  permissive (`additionalProperties: true`) and still exposes the legacy typed
  `language` / `theme` / `market` fields. GET and PUT are deliberately asymmetric:
  the new write fields round-trip through GET as untyped extra keys.
