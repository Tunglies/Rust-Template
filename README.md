# Rust-Template

A Rust crate template with CI for quality and format checks.

## Features

- 🦀 **Rust 2021 Edition** - Modern Rust project structure
- 🔍 **Quality Checks** - Automated CI with GitHub Actions
- 📝 **Format Checking** - Ensures consistent code style with `rustfmt`
- 🔧 **Linting** - Code quality checks with `clippy`
- ✅ **Testing** - Example tests included
- 📦 **Library & Binary** - Template for both lib and executable

## CI Pipeline

The template includes a comprehensive CI pipeline that runs on every push and pull request:

- **Test** - Runs all unit tests
- **Format Check** - Ensures code is properly formatted
- **Clippy** - Lints code for common mistakes and improvements
- **Build** - Verifies the project builds successfully

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Installation

Clone this template:

```bash
git clone https://github.com/Tunglies/Rust-Template.git
cd Rust-Template
```

### Building

Build the project:

```bash
cargo build
```

Build with optimizations:

```bash
cargo build --release
```

### Running

Run the example binary:

```bash
cargo run
```

### Testing

Run all tests:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

### Code Quality

Check code formatting:

```bash
cargo fmt -- --check
```

Auto-format code:

```bash
cargo fmt
```

Run clippy linter:

```bash
cargo clippy
```

Fix clippy warnings:

```bash
cargo clippy --fix
```

## Project Structure

```
.
├── .github/
│   └── workflows/
│       └── ci.yml          # GitHub Actions CI configuration
├── src/
│   ├── lib.rs              # Library code with example functions
│   └── main.rs             # Binary entry point
├── Cargo.toml              # Project manifest
├── .gitignore              # Git ignore rules
└── README.md               # This file
```

## Usage

This template includes example functions. Replace them with your own code:

```rust
use rust_template::{add, subtract};

fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
}
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Make sure your code passes all CI checks:
```bash
cargo test && cargo fmt -- --check && cargo clippy -- -D warnings
```

## License

MIT