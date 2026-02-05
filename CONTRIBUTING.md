# Contributing to `StaticDataGen`

Welcome! We're thrilled that you're interested in contributing to the `StaticDataGen` library. Whether you're looking to evangelize, submit feedback, or contribute code, we appreciate your involvement in making `StaticDataGen` a better tool for everyone. Here's how you can get started.

## Evangelize

One of the simplest ways to help us out is by spreading the word about `StaticDataGen`. We believe that a bigger, more involved community makes for a better framework, and that better frameworks make the world a better place. If you know people who might benefit from using `StaticDataGen`, please let them know!

## How to Contribute

If you're interested in making a more direct contribution, there are several ways you can help us improve `StaticDataGen`. Here are some guidelines for submitting feedback, bug reports, and code contributions.

### Feedback

Your feedback is incredibly valuable to us, and we're always looking for ways to make `StaticDataGen` better. If you have ideas, suggestions, or questions about `StaticDataGen`, we'd love to hear them. Here's how you can provide feedback:

- Click [here][02] to submit a new feedback.
- Use a descriptive title that clearly summarizes your feedback.
- Provide a detailed description of the issue or suggestion.
- Be patient while we review and respond to your feedback.

### Bug Reports

If you encounter a bug while using `StaticDataGen`, please let us know so we can fix it. Here's how you can submit a bug report:

- Click [here][02] to submit a new issue.
- Use a descriptive title that clearly summarizes the bug.
- Provide a detailed description of the issue, including steps to reproduce it.
- Be patient while we review and respond to your bug report.

### Code Contributions

If you're interested in contributing code to `StaticDataGen`, we're excited to have your help! Here's what you need to know:

#### Development Setup

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/staticdatagen.git
   cd staticdatagen
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/sebastienrousseau/staticdatagen.git
   ```
4. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

#### Code Style Guidelines

We follow Rust's standard coding conventions. Please ensure your code:

- Follows the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Is formatted with `rustfmt`
- Has no warnings from `clippy`
- Includes documentation for public APIs

#### Testing Requirements

Before submitting a pull request, ensure all tests pass:

```bash
# Run the test suite
cargo test --lib

# Check for linting issues
cargo clippy -- -D warnings

# Format your code
cargo fmt

# Build the library
cargo build --lib
```

**Important:** Code coverage must remain at or above 90%. Run tests with coverage to verify:

```bash
cargo tarpaulin --lib --out Html
```

#### Submitting a Pull Request

1. Ensure your changes pass all tests and linting checks
2. Write clear, concise commit messages
3. Update documentation if you've changed public APIs
4. Submit your pull request with:
   - A descriptive title
   - A summary of changes
   - Reference to any related issues (e.g., "Fixes #123")

#### Feature Requests

If you have an idea for a new feature or improvement, we'd love to hear it:

1. Check existing issues to avoid duplicates
2. Open a new issue describing the feature
3. Wait for feedback before implementing
4. Once approved, follow the contribution workflow above

We hope that this guide has been helpful in explaining how you can contribute to `StaticDataGen`. Thank you for your interest and involvement in our project!

[01]: https://github.com/sebastienrousseau/staticdatagen
[02]: https://github.com/sebastienrousseau/staticdatagen/issues/new
