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
    // let match
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

    // match
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
    (
        tokens = [_ => $default_token:expr$(,)?],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        var = $var:tt,
        result = [$($result:tt)*],
    ) => {
        let $var = $($result)* else {$default_token};
    };

    (
        tokens = [_ => $default_token:block],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        var = $var:tt,
        result = [$($result:tt)*],
    ) => {
        let $var = $($result)* else {$default_token};
    };

    (
        tokens = [_ => $default_token:expr$(,)?],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        var = $var:tt,
        result = unknown,
    ) => {
        let $var = if regex::Regex::new($first_pattern).unwrap().is_match($str) {
            $first_tokens
        } else {$default_token};
    };

    (
        tokens = [_ => $default_token:block],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        var = $var:tt,
        result = unknown,
    ) => {
        let $var = if regex::Regex::new($first_pattern).unwrap().is_match($str) {
            $first_tokens
        } else {$default_token};
    };

    (
        tokens = [$pattern:expr => $token:block $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        var = $var:tt,
        result = unknown,
    ) => {
        $crate::__regexm_parse! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_token = $first_tokens,
            var = $var,
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
        var = $var:tt,
        result = unknown,
    ) => {
        $crate::__regexm_parse! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_token = $first_tokens,
            var = $var,
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
        var = $var:tt,
        result = [$($result:tt)*],
    ) => {
        $crate::__regexm_parse! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_token = $first_tokens,
            var = $var,
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
        var = $var:tt,
        result = [$($result:tt)*],
    ) => {
        $crate::__regexm_parse! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_token = $first_tokens,
            var = $var,
            result = [$($result)* else if regex::Regex::new($pattern).unwrap().is_match($str) {
                $token
            }],
        }
    };

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

    (
        tokens = [_ => $default_token:expr$(,)?],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        result = unknown,
    ) => {
        if regex::Regex::new($first_pattern).unwrap().is_match($str) {
            $first_tokens
        } else {
            $default_token
        }
    };

    (
        tokens = [_ => $default_token:block],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        result = unknown,
    ) => {
        if regex::Regex::new($first_pattern).unwrap().is_match($str) {
            $first_tokens
        } else {
            $default_token
        }
    };

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
}

#[cfg(test)]
mod test {
    #[test]
    fn test_match() {
        regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => assert!(true),
            r"^\d{4}-\d{2}$" => assert!(false),
            _ => assert!(false),
        });

        regexm!(match "foo" {
            r"^\d{4}-\d{2}-\d{2}$" => assert!(false),
            r"^\d{4}-\d{2}$" => assert!(false),
            _ => assert!(true),
        });

        regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => {
                assert!(true);
                assert!(true);
            }
            r"^\d{4}-\d{2}$" => assert!(false),
            _ => assert!(false),
        });

        regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => assert!(true),
            _ => assert!(false),
        });

        regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}$" => assert!(false),
            r"^\d{4}-\d{2}-\d{2}$" => {
                assert!(true);
                assert!(true);
            }
            _ => assert!(false),
        });
    }

    #[test]
    fn test_let_match() {
        regexm!(let foo = match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => "ymd",
            r"^\d{4}-\d{2}$" => "ym",
            _ => "default"
        });
        assert_eq!(foo, "ymd");

        regexm!(let foo = match "foo" {
            r"^\d{4}-\d{2}-\d{2}$" => "ymd",
            r"^\d{4}-\d{2}$" => "ym",
            _ => "default"
        });
        assert_eq!(foo, "default");

        regexm!(let bar = match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => "ymd",
            _ => "default"
        });
        assert_eq!(bar, "ymd");
    }
}
