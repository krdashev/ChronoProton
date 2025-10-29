# Contributing to ChronoPhoton

Thank you for your interest in contributing to ChronoPhoton! This document provides guidelines for contributions.

## Code of Conduct

Be respectful, inclusive, and constructive. This is a scientific computing project aimed at advancing research.

## How to Contribute

### Reporting Bugs

- Use GitHub Issues
- Include minimal reproducible example
- Specify OS, Rust version, GPU hardware
- Attach relevant logs and configuration files

### Suggesting Features

- Open an issue with `[Feature Request]` prefix
- Describe the physics use case
- Outline proposed API if applicable

### Submitting Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make changes following the style guide
4. Add tests for new functionality
5. Run full test suite: `cargo test --all`
6. Run benchmarks if performance-critical: `cargo bench`
7. Format code: `cargo fmt`
8. Lint code: `cargo clippy -- -D warnings`
9. Commit with descriptive messages
10. Push and open a PR

## Code Style

- Follow Rust standard style (`rustfmt`)
- Pass all `clippy` lints with `-D warnings`
- Document all public APIs with rustdoc comments
- Include examples in doc comments where helpful
- Keep line length ≤ 100 characters

## Testing Requirements

- Unit tests for all new functions
- Integration tests for new features
- Validation tests against analytical solutions when possible
- Maintain > 85% code coverage

### Running Tests

```bash
# All tests
cargo test --all

# Specific module
cargo test core::hamiltonian

# With output
cargo test -- --nocapture

# Benchmarks
cargo bench
```

## Adding New Hamiltonians

1. Implement the `Hamiltonian` trait in `src/core/systems/`
2. Add unit tests verifying hermiticity and dimensions
3. Create an example configuration in `examples/configs/`
4. Add validation test against known analytical result if available
5. Document physics background in module docstring

Example structure:
```rust
/// Hamiltonian for [physics system]
///
/// Describes [equation and physics]
///
/// # Parameters
/// - `param1`: Description
///
/// # Example
/// ```rust
/// let h = MyHamiltonian::new(1.0, 2.0);
/// ```
pub struct MyHamiltonian {
    // fields
}

impl Hamiltonian for MyHamiltonian {
    // implementation
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hermiticity() { /* ... */ }
}
```

## Adding GPU Kernels

1. Write shader in WGSL in `src/gpu/shaders/`
2. Add CPU reference implementation
3. Add test comparing GPU and CPU results within tolerance
4. Profile and optimize for memory bandwidth

## Documentation

- Update user guide in `docs/` for user-facing features
- Update architecture docs for internal changes
- Add examples to `examples/` for new capabilities
- Keep `claude.md` in sync with major architectural changes

## Commit Messages

Use conventional commits format:

```
type(scope): subject

body

footer
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, no logic change)
- `refactor`: Code restructuring
- `perf`: Performance improvement
- `test`: Adding/updating tests
- `chore`: Maintenance tasks

Example:
```
feat(floquet): add Magnus integrator

Implement 2nd and 4th order Magnus expansion for
time-periodic Hamiltonians. Improves accuracy for
rapidly oscillating systems.

Closes #42
```

## Performance Considerations

- Profile before optimizing
- Use `cargo bench` to track regressions
- GPU kernels should achieve > 70% memory bandwidth utilization
- Document Big-O complexity for algorithms

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).
