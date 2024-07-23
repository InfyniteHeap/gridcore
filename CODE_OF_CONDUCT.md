> This Code of conduct is a draft so far.

# Code of conduct

To contribute to gridcore, you should follow these rules:

## The `use`-relevant

### Importing Order

We first import functions, structures, etc. from local modules, then from standard library, last from external crates. Among them must be seperated by an empty line.

Here is an example:

```rust
use crate::foo;
// or
use super::foo;
// or 
use self::foo;
// ----other imports----

use std::io;
use std::sync;
// ----other imports----

use tokio::runtime::Runtime;
// ----other imports----
```

### Importing Path

We import paths until reaching out types, traits, constants and macros, while importing paths until reaching out the upper module name of functions and methods:

```rust
// `Bar` is a type.
use foo::Bar;
// `baz()` is a function, so we declare its upper module name here.
use foo;

let var = Bar::new();
foo::baz(var);
```

The only exception is that when there are several types between/among which have the same name but distinguished behaviors (e.g. `reqwest::Result<T>` and `serde_json::Result<T>`), we also must declare the names of their individual upper module:

```rust
use serde_json::Value;

fn foo() -> reqwest::Result<String> { ... }
fn baz() -> serde_json::Result<Value> { ... }
```

When their individual upper module names are also conflicts, use `as` to rename the types:

```rust
use std::sync::Mutex as StdMutex;
use tokio::sync::Mutex as TokioMutex;
```

### Items Merging

We merge types, traits, constants, macros together when they are affiliated to the same module:

```rust
use foo::{Bar, NewType};
```

If you need to use functions/methods and types/traits/constants/macros, declare `self` when importing items:

```rust
use foo::{self, Bar, NewType};

foo::baz();
```

## `&str`, `String` and Other Relevant String Types

if possible, pass `&str` and return `String` when designing a function to diminish overheads and guarantee that there are no confusing lifetime issues:

```rust
fn foo(param: &str) -> String { ... }

fn bar() {
    let str1 = "text";
    let str2 = String::from("text");

    let str1_ret = foo(str1);
    let str2_ret = foo(&str2);
}
```

Also, use `String` when designing structures:

```rust
struct Foo {
    bar: String,
}
```

The exception is that, when a function returns a string literal, use `&'static str` instead of `String`:

```rust
fn foo() -> &'static str {
    "bar"
}
```