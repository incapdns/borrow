<img width="680" alt="banner" src="https://github.com/user-attachments/assets/1740befa-c25d-4428-bda8-c34d437f333e">

# [Click here to read the docs!](https://docs.rs/borrow)

## `no_std`

The runtime supports `no_std` when default features are disabled. The derive
macro still runs on the build host and may use `std`; its generated code uses
`core` paths.

```toml
[dependencies]
borrow = {
    git = "https://github.com/incapdns/borrow",
    default-features = false,
    features = ["no_usage_tracking"],
}
```

The default `std` feature preserves upstream behavior. Usage tracking is
available only with `std`; `no_std` builds use the zero-sized mock tracker.
