>This crate is under development.

# gridcore
The core of GCL that implements almost all of launcher functions.

# Usage
Please add this crate in your `Cargo.toml` file under `[dependencies]` item:

```toml
[dependencies]
gridcore = "0.1.0"
```

Since this crate is specifically produced for Tauri, the usage of this crate might have a little difficulties.

For example, to implement OAuth2 verification, we need to import these into your scope:

```rust
use std::future::Future;

use serde_json::Value;
use tokio::runtime::Runtime;

use gridcore::{auth::*, json::*};
```

Both the `serde_json` and `tokio` are required. `tokio` must have the feature `rt-multi-thread` enabled.

The rest of implementations can be referred from `test` folder.

# License
This crate is distributed under GPL-3.0 license.