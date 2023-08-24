# uuidv4

A Rust library for generating UUID v4.

## Installation

Run:

```bash
cargo add uuidv4
```

Or manually add to your Cargo.toml:

```toml
[dependencies]
uuidv4 = "0.1.0"
```

## Usage

```rust
use uuidv4::uuid;

fn main() {
    let my_uuid = uuid::v4();
    println!("Generated UUID: {}", my_uuid);
}
```

> You can play around with UUIDs or generate them online [here](https://lotsoftools.com/tools/uuid)
