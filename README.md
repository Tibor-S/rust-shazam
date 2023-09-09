# shazamrs

Credit: https://github.com/marin-m/SongRec

```rust
from_file(path: &str) -> Result<serde_json::Value>
```

```rust
from_buffer(buffer: Vec<f32>, sample_rate: u32) -> Result<serde_json::Value>`
```

buffer should be mono and `-1 =< samples =< 1`
