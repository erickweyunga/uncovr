# Contributing to Uncovr

Thank you for your interest in contributing to Uncovr! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/erickweyunga/uncovr.git
   cd uncover
   ```
3. Create a new branch for your feature or fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

1. Install Rust (1.70 or later):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Run benchmarks:
   ```bash
   cargo bench
   ```
   See [benches/README.md](benches/README.md) for more details.

5. Run the api example:
   ```bash
   cd examples/api
   cargo run
   ```

## Making Changes

### Code Style

- Follow Rust standard formatting with `rustfmt`:
  ```bash
  cargo fmt
  ```

- Run clippy to catch common mistakes:
  ```bash
  cargo clippy -- -D warnings
  ```

- Write clear, concise code with meaningful variable names
- Add documentation comments for public APIs using `///`
- Avoid using emojis in documentation and code comments

### Documentation

- Document all public types, functions, and modules
- Include examples in doc comments where appropriate
- Update README.md if adding new features
- Update CHANGELOG.md following the [Keep a Changelog](https://keepachangelog.com/) format

### Testing

- Write tests for new functionality
- Ensure all tests pass before submitting:
  ```bash
  cargo test
  ```

- Test examples to ensure they work:
  ```bash
  cd examples/api && cargo run
  ```

### Commit Messages

Write clear, descriptive commit messages:

```
Add feature: Brief description of the change

More detailed explanation of what changed and why,
if necessary. Keep lines under 72 characters.
```

## Submitting Changes

1. Commit your changes:
   ```bash
   git add .
   git commit -m "Your descriptive commit message"
   ```

2. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

3. Create a Pull Request from your fork to the main repository

4. In your PR description:
   - Describe what changes you made
   - Explain why you made them
   - Reference any related issues
   - Include screenshots for UI changes (if applicable)

## Pull Request Guidelines

- Keep PRs focused on a single feature or fix
- Update documentation and tests as needed
- Ensure CI passes (all tests, fmt, clippy)
- Respond to review feedback promptly
- Squash commits if requested before merging

## Project Structure

```
uncover/
├── src/
│   ├── api/            # Core API traits
│   ├── config/         # Configuration system
│   ├── context/        # Request context
│   ├── logging/        # Logging initialization
│   ├── middleware/     # Middleware utilities
│   ├── openapi/        # OpenAPI documentation
│   ├── server/         # Server builder and traits
│   └── lib.rs          # Main library entry point
├── benches/
│   ├── framework.rs    # Performance benchmarks
│   └── README.md       # Benchmark documentation
├── examples/
│   └── api/            # Example API application
├── scripts/
│   └── run_benchmarks.sh  # Benchmark helper script
├── CHANGELOG.md        # Version history
├── CONTRIBUTING.md     # This file
└── README.md           # Project documentation
```

## Adding New Features

When adding significant new features:

1. Open an issue first to discuss the feature
2. Wait for maintainer feedback before implementing
3. Keep the API simple and intuitive
4. Maintain backward compatibility when possible
5. Add comprehensive documentation and examples

## Reporting Bugs

When reporting bugs, include:

- Rust version (`rustc --version`)
- Uncovr version
- Minimal code to reproduce the issue
- Expected vs actual behavior
- Error messages or stack traces

## Questions?

If you have questions about contributing, feel free to open an issue with the "question" label.

## License

By contributing to Uncovr, you agree that your contributions will be licensed under the MIT License.
