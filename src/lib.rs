#[macro_export]
macro_rules! regexm {
    ($($tokens:tt)*) => {
        $crate::regexm_parse! {
            tokens = [$($tokens)*],
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! regexm_parse {
    (
        tokens = [let $var:tt = match $str:tt {$pattern:expr => $token:expr, $($rest:tt)*}],
    ) => {
        $crate::__regexm_parse! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $pattern,
            first_token = $token,
            var = $var,
            result = unknown,
        }
    };

    (
        tokens = [let $var:tt = match $str:tt {$pattern:expr => {$token:expr} $($rest:tt)*}],
    ) => {
        $crate::__regexm_parse! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $pattern,
            first_token = $token,
            var = $var,
            result = unknown,
        }
    };

    (
        tokens = [match $str:tt {$pattern:expr => $token:expr, $($rest:tt)*}],
    ) => {
        $crate::__regexm_parse! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $pattern,
            first_token = $token,
            result = unknown,
        }
    };

    (
        tokens = [match $str:tt {$pattern:expr => $token:block $($rest:tt)*}],
    ) => {
        $crate::__regexm_parse! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $pattern,
            first_token = $token,
            result = unknown,
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __regexm_parse {
    // (
    //     tokens = [_ => $default_token:expr$(,)?],
    //     str = $str:expr,
    //     first_pattern = $first_pattern:expr,
    //     first_token = $first_tokens:expr,
    //     var = $var:tt,
    //     result = [$result:tt],
    // ) => {
    //     let $var = $result else {$default_token};
    // };
    (
        tokens = [_ => $default_token:expr$(,)?],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        result = [$($result:tt)*],
    ) => {
        $($result)*
    };

    (
        tokens = [_ => $default_token:block],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        result = [$($result:tt)*],
    ) => {
        $($result)*
    };

    // (
    //     tokens = [$pattern:expr => $token:expr, $($rest:tt)*],
    //     str = $str:expr,
    //     first_pattern = $first_pattern:expr,
    //     first_token = $first_tokens:expr,
    //     var = $var:tt,
    //     result = unknown,
    // ) => {
    //     $crate::__regexm_parse! {
    //         tokens = [$($rest)*],
    //         str = $str,
    //         first_pattern = $first_pattern,
    //         first_token = $first_tokens,
    //         var = $var,
    //         result = [if regex::Regex::new($first_pattern).unwrap().is_match($str) {
    //             $first_tokens
    //         } else if regex::Regex::new($pattern).unwrap().is_match($str) {
    //             $token
    //         }],
    //     }
    // };

    (
        tokens = [$pattern:expr => $token:expr, $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        result = unknown,
    ) => {
        $crate::__regexm_parse! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_token = $first_tokens,
            result = [if regex::Regex::new($first_pattern).unwrap().is_match($str) {
                $first_tokens
            } else if regex::Regex::new($pattern).unwrap().is_match($str) {
                $token
            }],
        }
    };

    (
        tokens = [$pattern:expr => $token:block $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        result = unknown,
    ) => {
        $crate::__regexm_parse! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_token = $first_tokens,
            result = [if regex::Regex::new($first_pattern).unwrap().is_match($str) {
                $first_tokens
            } else if regex::Regex::new($pattern).unwrap().is_match($str) {
                $token
            }],
        }
    };

    (
        tokens = [$pattern:expr => $token:expr, $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        result = [$($result:tt)*],
    ) => {
        $crate::__regexm_parse! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_token = $first_tokens,
            result = [$($result)* else if regex::Regex::new($pattern).unwrap().is_match($str) {
                $token
            }],
        }
    };

    (
        tokens = [$pattern:expr => $token:block $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        result = [$($result:tt)*],
    ) => {
        $crate::__regexm_parse! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_token = $first_tokens,
            result = [$($result)* else if regex::Regex::new($pattern).unwrap().is_match($str) {
                $token
            }],
        }
    };

    // (
    //     tokens = [$pattern:expr => $token:expr, $($rest:tt)*],
    //     str = $str:expr,
    //     first_pattern = $first_pattern:expr,
    //     first_token = $first_tokens:expr,
    //     var = $var:tt,
    //     result = [$result:tt],
    // ) => {
    //     $crate::__regexm_parse! {
    //         tokens = [$($rest)*],
    //         str = $str,
    //         first_pattern = $first_pattern,
    //         first_token = $first_tokens,
    //         var = $var,
    //         result = [$result else if regex::Regex::new($pattern).unwrap().is_match($str) {
    //             $tokens
    //         }],
    //     }
    // };

    // (
    //     tokens = [$($pattern:expr => $tokens:expr,)* _ => $default_token:expr$(,)?],
    //     str = $str:expr,
    //     first_pattern = $first_pattern:expr,
    //     first_token = $first_tokens:expr,
    //     var = $var:tt,
    //     result = [$result:expr],
    // ) => {
    //     let $var = $result;
    // };

    // (
    //     tokens = [$($pattern:expr => $tokens:expr,)* _ => $default_token:expr$(,)?],
    //     str = $str:expr,
    //     first_pattern = $first_pattern:expr,
    //     first_token = $first_tokens:expr,
    //     result = $result:tt,
    // ) => {
    //     if regex::Regex::new($first_pattern).unwrap().is_match($str) {
    //         $first_tokens
    //     } $(else if regex::Regex::new($pattern).unwrap().is_match($str) {
    //         $tokens
    //     })* else {
    //         $default_token
    //     }
    // };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_regexm() {
        let text1 = "2021-01-01";
        regexm!(match text1 {
            r"^\d{4}-\d{2}-\d{2}$" => assert!(true),
            r"^\d{4}-\d{2}$" => assert!(false),
            _ => assert!(false),
        });

        // regexm!(let foo = match text1 {
        //     r"^\d{4}-\d{2}-\d{2}$" => "ymd",
        //     r"^\d{4}-\d{2}$" => "ym",
        //     _ => "default"
        // });
        // assert_eq!(foo, "ymd");

        // let text2 = "foo";
        // regexm!(let foo = match text2 {
        //     r"^\d{4}-\d{2}-\d{2}$" => "ymd",
        //     r"^\d{4}-\d{2}$" => "ym",
        //     _ => "default"
        // });
        // assert_eq!(foo, "default")
    }
}
