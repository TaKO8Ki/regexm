#[macro_export]
macro_rules! regexm {
    ($($tokens:tt)*) => {
        $crate::__regexm! {
            tokens = [$($tokens)*],
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __regexm {
    (
        tokens = [match $str:tt {$pattern:expr => $tokens:expr, $($rest:tt)*}],
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $pattern,
            first_token = $tokens,
        }
    };

    (
        tokens = [$($pattern:expr => $tokens:expr$(,)?)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
    ) => {
        if regex::Regex::new($first_pattern).unwrap().is_match($str) {
            $first_tokens
        } $(else if regex::Regex::new($pattern).unwrap().is_match($str) {
            $tokens
        })*
    };
}

// #[cfg(test)]
// mod test {
//     super::regex;
//     #[test]
//     fn test_regexm() {
//         let text = "foo";
//         let result: &str;

//         regexm!(match text {
//             r#"foo?"# => result = "foo",
//         })
//     }
// }
