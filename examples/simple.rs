fn main() {
    let text = "2020-01-01";
    regexm::regexm!(match text {
        r"^\d{4}-\d{2}$" => println!("ym"),
        r"^\d{4}-\d{2}-\d{2}$" => {
            println!("ymd");
            println!("ymd");
        }
        r"^\d{4}-\d{2}a$" => println!("ym"),
        _ => println!("default"),
    });

    let text2 = "foo";
    regexm::regexm!(let foo = match text2 {
        r"^\d{4}-\d{2}-\d{2}$" => "ymd",
        r"^\d{4}-\d{2}$" => "ym",
        r"^\d{4}-\d{2}-\d{2}$" => {
            "ymd";
            "ymd"
        }
        _ => "default"
    });
    println!("{}", foo);
}
