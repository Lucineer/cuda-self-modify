# cuda-self-modify

Self-modifying programs — runtime code adaptation through deliberation with checkpoint/rollback

Part of the Cocapn fleet — a Lucineer vessel component.

## What It Does

### Key Types

- `Mutation` — core data structure
- `Observation` — core data structure
- `SelfModifyingProgram` — core data structure
- `Checkpoint` — core data structure
- `AdaptationStats` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-self-modify.git
cd cuda-self-modify

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_self_modify::*;

// See src/lib.rs for full API
// 6 unit tests included
```

### Available Implementations

- `SelfModifyingProgram` — see source for methods

## Testing

```bash
cargo test
```

6 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: other
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates


## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
