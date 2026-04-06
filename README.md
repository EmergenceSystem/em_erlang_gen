# em_erlang_gen

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Emergence agent — generates Erlang source code from an [AlephTree](https://github.com/aleph-lang/aleph-syntax-tree) intermediate representation.

This agent is the Emergence network integration of the [`erlanggen`](https://crates.io/crates/erlanggen) aleph-lang crate.

---

## Role in the Aleph pipeline

```
Source code (any language)
      ↓  em_*_parser
  AlephTree (JSON)
      ↓  em_betareduction / em_constant_folding  (optional)
  AlephTree (JSON)
      ↓  em_erlang_gen  (this agent)
   Erlang source code
```

---

## Query protocol

**Input** (`body`): JSON-serialized `aleph_tree` embryo

**Output**: JSON array containing one `code` embryo:
```json
[{
  "type": "code",
  "properties": {
    "language": "erlang",
    "source_language": "python",
    "content": "% generated erlang..."
  }
}]
```

---

## Capabilities

```rust
vec!["generator", "erlang"]
```

---

## Deployment

```bash
git clone https://github.com/EmergenceSystem/em_erlang_gen
cd em_erlang_gen
cargo build --release
./target/release/em_erlang_gen
```

Configure via environment variables:
- `EM_DISCO_NODES` — comma-separated `host:port` list (default: `localhost:8080`)
- `EM_FILTER_JWT` — optional JWT token
- `EM_FILTER_RECONNECT_MS` — reconnect delay in ms (default: 5000)
