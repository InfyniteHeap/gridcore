> [!WARNING]
> This project is waiting for being refactored, as its structure and bygone Git
> commit history is too unreasonable and chaoic to be maintained.

# gridcore

The core implementation of Grid Craft Launcher, yet not limited to that!

You can choose either directly using it as a command line program, or
integrating it into to your project as a dependency!

## Features

**A completely asynchronous core**!

- Powered by [Tokio](https://tokio.rs/), gridcore runs fast and effective!

**An effective core!**

## Usage

### As A Command Line Tool

run `grid-cli --help` to get usages.

### As A Dependency

Add this crate to your `Cargo.toml` file under `[dependencies]` table:

```toml
gridcore = "0.1.0"
```

Or let Cargo helps you:

```bash
cargo add gridcore
```

## License

This crate is distributed under GPL-3.0 license.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details!
