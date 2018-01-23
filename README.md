# Helpscout API Bindings in Rust

Currently in active development.

Example:

```rust
let api_key: String = env::var("API_KEY").expect("to have API_KEY set");
let mut c = Client::new(&api_key);
let mailboxes = mailboxes::list(&c).expect("Mailboxes to be listed");
```

See the tests directory for more examples.
