<h2># Async Await Example with Reqwest and Error Handling in Rust</h2>

<p>This Rust project demonstrates how to perform asynchronous HTTP requests using the `reqwest` crate with proper error handling using the `error-chain` crate.</p>

<p>## Requirements</p>

<li> Rust (latest stable version)</li>
<li> Cargo (comes with Rust)</li>
<br>
<p>## Dependencies</p>

This project relies on the following crates:
- `reqwest` for making HTTP requests.
- `tokio` for asynchronous runtime.
- `error-chain` for simplified error handling.
<br>
Ensure your `Cargo.toml` includes:
<br>
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
error-chain = "0.12"
```
