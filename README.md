# env-rs
Simple lib to parse envs to struct in rust.

# usage
```rust
use env_rs::{Parser, self};

#[derive(Parser, Debug)]
struct Env {
    // the env variable `name`
    name: String,
    // the env variable `age`
    age: String
}

let env: Env = env_rs::parse().unwrap();
println!("{:?}", env);
```
