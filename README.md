# Fly Demo
A simple web service in Rust with CI/CD using GitHub actions, and automated deployments to [fly.io](https://fly.io)

## Build and Test

### Prerequisites
Ensure you have Rust and Cargo installed. See [rustup.rs](https://rustup.rs) for installation instructions.

### Build
To build the project, run:
```bash
cargo build
```

### Test
To run the tests, use:
```bash
cargo test
```

### Run locally
To run the tests, use:
```bash
cargo run
```

## GitHub Workflows
This project includes two GitHub workflows:

- **[Rust Workflow](.github/workflows/rust.yml)**
  Runs on every push or pull request to `main`. Builds the project, runs Clippy, and executes tests.

- **[Fly Deploy Workflow](.github/workflows/fly-deploy.yml)**
  Triggers after a successful `Rust` workflow run. Deploys the app to Fly.io using `flyctl`.
