# Lab 3 - Rust Migration, Containers, and CI/CD (Midterm)

PerfReport reports processor count, memory usage, uptime, disk usage, and an efficiency score. This project migrates the Windows C++ application to Rust using `sysinfo` and packages it in a Linux Docker container.

## Build and run

```sh
cargo run
cargo fmt --all --check
cargo build --release --locked
cargo test --locked
docker build -t perfreport .
docker run --rm perfreport
```

The original C++ source and Windows build script are in `cpp/`. The Rust application runs on Windows and Linux. Tests verify scoring thresholds and ratings.

On Windows, run `cpp/build_cpp.bat` to compile the original application, then run `cpp/perfreport.exe` to display its report.

GitHub Actions checks formatting, builds, and tests on pushes to main and pull requests. The Docker image builds and runs after the Rust checks pass.
