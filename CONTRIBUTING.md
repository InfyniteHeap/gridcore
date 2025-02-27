# Contributing to gridcore

Thank you for your interest in contributing to gridcore! You can fork this repository, create an independent branch alongside the main branch, apply your changes, and submit a Pull Request!

When you make your changes, please follow these code formats:

1. Order of Importing

We first import items from local modules, then the standard library, and finally third-party crates:

```rust
use crate::mod_name::StructureName;
use super::EnumerationName;

use std::sync::LazyLock;

use tokio::fs::File;
```

2. Merging vs. Flattening Import Statements

We merge these items using braces for structures, enumerations, and constants:

```rust
use crate::mod_name::{StructureName, EnumerationName, CONSTANT_NAME};
```

Nested braces are not allowed. Therefore, you must split nested items into separate lines:

```rust
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, sync_channel};

// This is not accepted:
// use std::sync::{Arc, RwLock, mpsc::{channel, sync_channel}};
```

3. In-code Layout

We follow this layout in a single code file:

|             layout             |
| :----------------------------: |
|          importations          |
| constants and static variables |
|  structures and enumerations   |
|        implementations         |
|         pure functions         |

Please don't forget to write test code after adding new features.
