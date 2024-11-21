# Contributing to gridcore

Thank you for your interest in contributing to gridcore! You can fork this repository, create an independent branch
alongside of the main branch, apply your changes and create a Pull Request!

When you make your changes, please follow these code formats:

1. The Order of Importing

We first import items from local create, then the standard library, and last the third-party crates:

```rust
use crate::mod_name::StructureName;
use super::EnumerationName;

use std::sync::LazyLock;

use tokio::fs::File;
```

2. The Selection between Merging Importations and Flatting Importations

We merge these items into a pair of brackets: structures, enumerations and constants:

```rust
use crate::mod_name::{StructureName, EnumerationName, CONSTANT_NAME};
```

Nested brackets are not accepted. This means you must split the nested items into another line:

```rust
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, sync_channel};

// This is not accepted:
// use std::sync::{Arc, RwLock, mpsc::{channel, sync_channel}};
```

3. In-code Layout

We follow this layout in a single code file:

|             layout             |
|:------------------------------:|
|          importations          |
| constants and static variables |
|  structures and enumerations   |
|        implementations         |
|         pure functions         |

Please don't forget to write test code after adding new features.