Derives for high performance JSON serialisation (and eventually deserialisation).

Attributres are supported for both `#[simd_json(...)]` and for compatibilty also for `#[serde(...)]` and follow the same nameing conventions as serde.

For fields:

* `rename = "new_name"` - renames a field