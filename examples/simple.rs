fn main() {
    let text1 = "2020-01-01";
    regexm::regexm!(match text1 {
        r"^\d{4}$" => println!("y"),
        r"^\d{4}-\d{2}$" => println!("y-m"),
        // block
        r"^\d{4}-\d{2}-\d{2}$" => {
            let y_m_d = "y-m-d";
            println!("{}", y_m_d);
        }
        _ => println!("default"),
    });

    let text2 = "foo";
    regexm::regexm!(let foo = match text2 {
        r"^\d{4}-\d{2}-\d{2}$" => "ymd",
        r"^\d{4}-\d{2}$" => "ym",
        // block
        r"^\d{4}-\d{2}-\d{2}$" => {
            let ymd = "ymd";
            ymd
        }
        _ => "default"
    });
    println!("{}", foo);
}
