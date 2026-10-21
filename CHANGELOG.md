# Changelog

All three crates keep hampi's version number (0.7.2) so consumers can route
`asn1-codecs = "=0.7.2"` requirements here with `[patch.crates-io]`. Releases of
this repository are tagged `vX.Y.Z` independently of that crate version.

## v0.1.0 -- extraction from nextgsim

First standalone release. Content is byte-for-byte the compiler nextgsim
vendored at `2f27979` (= nextgsim main `7d18b7d` for these crates), which
nextgcore pins today. Every fix below was made in nextgsim and keeps its
history here.

### Fixes over hampi 0.7.2

See README "What is fixed" for the standards reference behind each.

- CHOICE: encode the extension arm, not only decode it (nextgsim #117 / PR #181).
- Information object classes: keep fields written inline as a base type
  (`OCTET STRING (CONTAINING ...)`) in the generated open-type enum
  (nextgsim #185 / PR #187).
- Parser: an object-set pass that consumes no token is a parse error, not an
  infinite loop (nextgsim #45 / PR #195).
- Opt-in skipping of unknown IEs, reported with their criticality
  (nextgsim #214 / PR #217).
- SEQUENCE OF: enforce SIZE on encode; extensible SIZE (nextgsim #212 / PR #215).
- ENUMERATED: encode extension additions and decode them to the right value
  (nextgsim #213 / PR #216).
- An entry of an empty extension object set is a real open-type entry
  (nextgsim #218 / PR #219).
- Length determinants of 16K and over are fragmented (nextgsim #222 / PR #223).
- Variable-size BIT/OCTET STRING content is octet-aligned at any length
  (nextgsim #226 / PR #227).
- Extensible INTEGER values past the root encode; types are sized from the
  extension range (nextgsim #231 / PR #235).
- Mixed single-value + range constraints take the largest upper bound
  (nextgsim #241 / PR #242).
- Character strings are aligned by the type's upper bound; UTF8String's size is
  not PER-visible (nextgsim #246 / PR #247).
- SEQUENCE extension additions and addition groups are generated and coded;
  normally-small length switches form above 64, not 32 (nextgsim #253 / PR #256).

### Added in this repository

- Cargo workspace over the three crates.
- `codegen-golden`: SHA-256 pins of the generated code for every schema
  nextgcore and nextgsim compile, enforced in CI.
