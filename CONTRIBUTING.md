# Contributing to gridcore

Thank you for your interest in contributing to gridcore! You can fork this
repository, create an independent branch alongside the main branch, apply your
changes, and submit a Pull Request!

## Build from Source

So far, gridcore only requires Rust, except when you're using Linux, as some
dependencies depends on `openssl` and `pkg-config`, which has probably not
bundled in your distro.

If you've not installed these two yet, feel free to install them on your
computer:

```bash
sudo apt install openssl pkg-config
```

You must also install Rust in advance. The installer can be found on Rust
official website.

Once installed, you can start developing gridcore with your favourite editor or
IDE.

> [!NOTE]
>
> If you're using NixOS, you're no need to install Rust, yet must enable flake
> in advance.
>
> Once enabled, you can clone this repo, enter in the directory, and run this
> command to enable an isolated development environmemt provided by NixOS:
>
> ```bash
> nix develop
> ```
>
> and then you can develop gridcore as if you're using any other systems!

## Code Style Specifications

When you make your changes, please follow these code formats:

1. Order of Importing

We first import items from local modules, then the standard library, and finally
third-party crates:

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

Nested braces are not allowed. Therefore, you must split nested items into
separate lines:

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
