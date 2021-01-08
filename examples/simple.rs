fn main() {
    let text = "2014-01-01";
    regexm::regexm!(match text {
        r"^\d{4}-\d{2}-\d{2}$" => println!("ymd"),
        r"^\d{4}-\d{2}$" => println!("ym"),
    })
}
