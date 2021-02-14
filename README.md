<div align="center">

 # regexm
 
 A Rust macro for writing regex pattern matching.

 [![github workflow status](https://img.shields.io/github/workflow/status/TaKO8Ki/regexm/CI/main)](https://github.com/TaKO8Ki/regexm/actions) [![crates](https://img.shields.io/crates/v/regexm.svg?logo=rust)](https://crates.io/crates/regexm) [![docs](https://img.shields.io/badge/docs-regexm-8da0cb?labelColor=555555&logo=rust)](https://docs.rs/regexm)

 [Usage](##Usage) | [Examples](examples) | [Docs](https://docs.rs/regexm)

</div>

## Features

- Capture groups

## Dependencies

```toml
[dependencies]
regex = "1"
regexm = "0.1"
```

If you want to use `Capture groups` feature, please use `regexm = "0.2-beta"`.

## Usage

```rust
fn main() {
    let text1 = "2020-01-01";
    regexm::regexm!(match text1 {
        r"^\d{4}$" => println!("yyyy"),
        r"^\d{4}-\d{2}$" => println!("yyyy-mm"),
        // block
        r"^\d{4}-\d{2}-\d{2}$" => {
            let yyyy_mm_dd = "yyyy-mm-dd";
            println!("{}", yyyy_mm_dd);
        }
        _ => println!("default"),
    });
}
```

Output:

```sh
yyyy-mm-dd
```


```rust
fn main() {
    let text2 = "foo";
    let foo = regexm::regexm!(match text2 {
        r"^\d{4}-\d{2}-\d{2}$" => "yyyy-mm-dd",
        r"^\d{4}-\d{2}$" => "yyyy-mm",
        // block
        r"^\d{4}-\d{2}-\d{2}$" => {
            let yyyy_mm_dd = "yyyy-mm-dd";
            yyyy_mm_dd
        }
        _ => "default",
    });
    println!("{}", foo);
}
```

Output:

```sh
default
```
