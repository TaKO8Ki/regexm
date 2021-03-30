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
regexm = "0.2"
```

## Usage

### Simple pattern matching

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

the generated code will be the following:

```rust
fn main() {
    let text1 = "2020-01-01";
    if regex::Regex::new(r"^\d{4}$").unwrap().is_match(text1) {
        println!("yyyy")
    } else if regex::Regex::new(r"^\d{4}-\d{2}$").unwrap().is_match(text1) {
        println!("yyyy-mm")
    } else if regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$")
        .unwrap()
        .is_match(text1)
    {
        let yyyy_mm_dd = "yyyy-mm-dd";
        println!("{}", yyyy_mm_dd);
    } else {
        println!("default")
    };
}
```


### Let match

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

the generated code will be the following:

```rust
fn main() {
    let text2 = "foo";
    let foo = if regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$")
        .unwrap()
        .is_match(text2)
    {
        "yyyy-mm-dd"
    } else if regex::Regex::new(r"^\d{4}-\d{2}$").unwrap().is_match(text2) {
        "yyyy-mm"
    } else if regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$")
        .unwrap()
        .is_match(text2)
    {
        let yyyy_mm_dd = "yyyy-mm-dd";
        yyyy_mm_dd
    } else {
        "default"
    };
    println!("{}", foo);
}
```

### Capture Groups

```rust
fn main() {
    let text1 = "2020-01-02";
    regexm::regexm!(match text1 {
        // capture groups
        captures(r"^(\d{4})-(\d{2})-(\d{2})$") => |caps| println!(
            "year: {}, month: {}, day: {}",
            caps.get(1).map_or("", |m| m.as_str()),
            caps.get(2).map_or("", |m| m.as_str()),
            caps.get(3).map_or("", |m| m.as_str())
        ),
        _ => println!("default"),
    });
}
```

Output:

```sh
2020
01
02
```

the generated code will be the following:

```rust
fn main() {
    let text1 = "2020-01-02";
    if regex::Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$")
        .unwrap()
        .is_match(text1)
    {
        let closure = |caps: regex::Captures| {
            println!(
                "year: {}, month: {}, day: {}",
                caps.get(1).map_or("", |m| m.as_str()),
                caps.get(2).map_or("", |m| m.as_str()),
                caps.get(3).map_or("", |m| m.as_str())
            )
        };
        closure(
            regex::Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$")
                .unwrap()
                .captures(text1)
                .unwrap(),
        )
    } else {
        println!("default")
    };
}

```
