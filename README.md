<div align="center">

 # regexm

 [![github workflow status](https://img.shields.io/github/workflow/status/TaKO8Ki/regexm/CI/main)](https://github.com/TaKO8Ki/regexm/actions) [![crates](https://img.shields.io/crates/v/regexm.svg?logo=rust)](https://crates.io/crates/regexm) [![docs](https://img.shields.io/badge/docs-regexm-8da0cb?labelColor=555555&logo=rust)](https://docs.rs/regexm)

 [Usage](##Usage) | [Examples](examples) | [Docs](https://docs.rs/regexm)

</div>

## Dependencies

```toml
[dependencies]
regex = "1"
regexm = "0.1-beta"
```

## Usage

```rust
fn main() {
    let text = "2020-01-01";
    regexm::regexm!(match text {
        r"^\d{4}-\d{2}-\d{2}$" => println!("ymd"),
        r"^\d{4}-\d{2}$" => println!("ym"),
        _ => println!("default"),
    });

    let text2 = "foo";
    regexm::regexm!(let foo = match text2 {
        r"^\d{4}-\d{2}-\d{2}$" => "ymd",
        r"^\d{4}-\d{2}$" => "ym",
        _ => "default"
    });
    println!("{}", foo)
}
```
