fn main() {
    let text1 = "2020-01-02";
    regexm::regexm!(match text1 {
        r"^(\d{4})-(\d{2})-(\d{2})$" => |caps| println!(
            "year: {}, month: {}, day: {}",
            caps.get(1).map_or("", |m| m.as_str()),
            caps.get(2).map_or("", |m| m.as_str()),
            caps.get(3).map_or("", |m| m.as_str())
        ),
        _ => println!("default"),
    });

    // Output:
    // 2020
    // 01
    // 02
}
