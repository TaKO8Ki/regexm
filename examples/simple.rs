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

    // Output: yyyy-mm-dd

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

    // Output: default
}
