# Vendored crates

## `vello_encoding` 0.6.0 and 0.9.0

Crisp text for every vello renderer in CrownOS: masonry/xilem (0.6) and
crownshell (0.9). Upstream only hints glyphs vertically and places them at
fractional x, so unhinted fonts such as Inter render soft stems. Two changes:

- `resolve.rs`: hinted glyph origins snap to whole pixels on both axes, not only y.
- stem darkening: outlines grow by 0.07px per side (`STEM_DARKENING`), in
  `stem_darkening.rs` (0.6) and `FontEmbolden::with_stem_darkening` (0.9).

Each consuming workspace opts in with:

```toml
[patch.crates-io]
vello_encoding = { path = "../vendor/vello_encoding-0.9.0" }
# a workspace that also has xilem/masonry adds the 0.6 copy under a second key:
vello_encoding_06 = { package = "vello_encoding", path = "../vendor/vello_encoding-0.6.0" }
```

Drop a copy when its vello version is no longer in use.
