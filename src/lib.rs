#[macro_export]
macro_rules! regexm {
    (
        match $str:tt {$pattern:expr => |$caps:ident| $expr:expr, $($rest:tt)*}
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $pattern,
            first_expr = {
                let closure = |$caps: regex::Captures| $expr;
                closure(regex::Regex::new($pattern).unwrap().captures($str).unwrap())
            },
            result = unknown,
        }
    };

    (
        match $str:tt {$pattern:expr => |$caps:ident| $expr:block $($rest:tt)*}
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $pattern,
            first_expr = {
                let closure = |$caps: regex::Captures| $expr;
                closure(regex::Regex::new($pattern).unwrap().captures($str).unwrap())
            },
            result = unknown,
        }
    };

    (
        match $str:tt {$pattern:expr => $expr:expr, $($rest:tt)*}
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $pattern,
            first_expr = $expr,
            result = unknown,
        }
    };

    (
        match $str:tt {$pattern:expr => $expr:block $($rest:tt)*}
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $pattern,
            first_expr = $expr,
            result = unknown,
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __regexm {
    (
        tokens = [$default:pat => |$caps:ident| $default_expr:expr$(,)?],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = [$($result:tt)*],
    ) => {
        $($result)* else {
            let closure = |$caps: regex::Captures| $default_expr;
            closure(regex::Regex::new($default).unwrap().captures($str).unwrap())
        };
    };

    (
        tokens = [$default:pat => |$caps:ident| $default_expr:block$(,)?],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = [$($result:tt)*],
    ) => {
        $($result)* else {
            let closure = |$caps: regex::Captures| $default_expr;
            closure(regex::Regex::new($default).unwrap().captures($str).unwrap())
        };
    };

    (
        tokens = [$default:pat => $default_expr:expr$(,)?],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = [$($result:tt)*],
    ) => {
        $($result)* else {$default_expr};
    };

    (
        tokens = [$default:pat => $default_expr:block$(,)?],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = [$($result:tt)*],
    ) => {
        $($result)* else {$default_expr};
    };

    (
        tokens = [],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = [$($result:tt)*],
    ) => {
        $($result)*
    };

    (
        tokens = [$default:pat => |$caps:ident| $default_expr:expr$(,)?],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = unknown,
    ) => {
        if regex::Regex::new($first_pattern).unwrap().is_match($str) {
            $first_expr
        } else {
            let closure = |$caps: regex::Captures| $default_expr;
            closure(regex::Regex::new($default).unwrap().captures($str).unwrap())
        };
    };

    (
        tokens = [$default:pat => |$caps:ident| $default_expr:block$(,)?],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = unknown,
    ) => {
        if regex::Regex::new($first_pattern).unwrap().is_match($str) {
            $first_expr
        } else {
            let closure = |$caps: regex::Captures| $default_expr;
            closure(regex::Regex::new($default).unwrap().captures($str).unwrap())
        };
    };

    (
        tokens = [$default:pat => $default_expr:expr$(,)?],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = unknown,
    ) => {
        if regex::Regex::new($first_pattern).unwrap().is_match($str) {
            $first_expr
        } else {$default_expr};
    };

    (
        tokens = [$default:pat => $default_expr:block$(,)?],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_token = $first_tokens:expr,
        result = unknown,
    ) => {
        if regex::Regex::new($first_pattern).unwrap().is_match($str) {
            $first_tokens
        } else {$default_expr};
    };

    (
        tokens = [$pattern:expr => |$caps:ident| $expr:expr, $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = unknown,
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_expr = $first_expr,
            result = [if regex::Regex::new($first_pattern).unwrap().is_match($str) {
                $first_expr
            } else if regex::Regex::new($pattern).unwrap().is_match($str) {
                let closure = |$caps: regex::Captures| $expr;
                closure(regex::Regex::new($pattern).unwrap().captures($str).unwrap())
            }],
        }
    };

    (
        tokens = [$pattern:expr => |$caps:ident| $expr:block $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = unknown,
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_expr = $first_expr,
            result = [if regex::Regex::new($first_pattern).unwrap().is_match($str) {
                $first_expr
            } else if regex::Regex::new($pattern).unwrap().is_match($str) {
                let closure = |$caps: regex::Captures| $expr;
                closure(regex::Regex::new($pattern).unwrap().captures($str).unwrap())
            }],
        }
    };

    (
        tokens = [$pattern:expr => $expr:expr, $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = unknown,
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_expr = $first_expr,
            result = [if regex::Regex::new($first_pattern).unwrap().is_match($str) {
                $first_expr
            } else if regex::Regex::new($pattern).unwrap().is_match($str) {
                $expr
            }],
        }
    };

    (
        tokens = [$pattern:expr => $expr:block $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = unknown,
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_expr = $first_expr,
            result = [if regex::Regex::new($first_pattern).unwrap().is_match($str) {
                $first_expr
            } else if regex::Regex::new($pattern).unwrap().is_match($str) {
                $expr
            }],
        }
    };

    (
        tokens = [$pattern:expr => |$caps:ident| $expr:expr, $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = [$($result:tt)*],
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_expr = $first_expr,
            result = [$($result)* else if regex::Regex::new($pattern).unwrap().is_match($str) {
                let closure = |$caps: regex::Captures| $expr;
                closure(regex::Regex::new($pattern).unwrap().captures($str).unwrap())
            }],
        }
    };

    (
        tokens = [$pattern:expr => |$caps:ident| $expr:block $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = [$($result:tt)*],
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_expr = $first_expr,
            result = [$($result)* else if regex::Regex::new($pattern).unwrap().is_match($str) {
                let closure = |$caps: regex::Captures| $expr;
                closure(regex::Regex::new($pattern).unwrap().captures($str).unwrap())
            }],
        }
    };

    (
        tokens = [$pattern:expr => $expr:expr, $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = [$($result:tt)*],
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_expr = $first_expr,
            result = [$($result)* else if regex::Regex::new($pattern).unwrap().is_match($str) {
                $expr
            }],
        }
    };

    (
        tokens = [$pattern:expr => $expr:block $($rest:tt)*],
        str = $str:expr,
        first_pattern = $first_pattern:expr,
        first_expr = $first_expr:expr,
        result = [$($result:tt)*],
    ) => {
        $crate::__regexm! {
            tokens = [$($rest)*],
            str = $str,
            first_pattern = $first_pattern,
            first_expr = $first_expr,
            result = [$($result)* else if regex::Regex::new($pattern).unwrap().is_match($str) {
                $expr
            }],
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_match_3_or_more_pattern() {
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
            r"^\d{4}-\d{2}-\d{2}$" => assert!(true),
            r"^\d{4}-\d{2}$" => assert!(false),
            r"^\d{4}$" => assert!(false),
            _ => assert!(false),
        });
    }

    #[test]
    fn test_match_3_or_more_pattern_block() {
        regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => {
                assert!(true);
                assert!(true);
            }
            r"^\d{4}-\d{2}$" => {
                assert!(false);
                assert!(false);
            }
            _ => {
                assert!(false);
                assert!(false)
            }
        });

        regexm!(match "foo" {
            r"^\d{4}-\d{2}-\d{2}$" => {
                assert!(false);
                assert!(false);
            }
            r"^\d{4}-\d{2}$" => {
                assert!(false);
                assert!(false);
            }
            _ => {
                assert!(true);
                assert!(true)
            }
        });

        regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => {
                assert!(true);
                assert!(true);
            }
            r"^\d{4}-\d{2}$" => {
                assert!(false);
                assert!(false);
            }
            r"^\d{4}$" => {
                assert!(false);
                assert!(false);
            }
            _ => {
                assert!(false);
                assert!(false)
            }
        });
    }

    #[test]
    fn test_match_2_or_less_pattern() {
        regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => assert!(true),
            _ => assert!(false),
        });

        regexm!(match "foo" {
            r"^\d{4}-\d{2}$" => assert!(false),
            _ => assert!(true),
        });
    }

    #[test]
    fn test_match_2_or_less_pattern_block() {
        regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => {
                assert!(true);
                assert!(true);
            }
            _ => {
                assert!(false);
                assert!(false)
            }
        });

        regexm!(match "foo" {
            r"^\d{4}-\d{2}$" => {
                assert!(false);
                assert!(false)
            }
            _ => {
                assert!(true);
                assert!(true);
            }
        });
    }

    #[test]
    fn test_let_match_3_or_more_pattern() {
        let foo = regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => "yyyy-mm-dd",
            r"^\d{4}-\d{2}$" => "yyyy-mm",
            _ => "default",
        });
        assert_eq!(foo, "yyyy-mm-dd");

        let foo = regexm!(match "foo" {
            r"^\d{4}-\d{2}-\d{2}$" => "yyyy-mm-dd",
            r"^\d{4}-\d{2}$" => "yyyy-mm",
            _ => "default",
        });
        assert_eq!(foo, "default");

        let foo = regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => "yyyy-mm-dd",
            r"^\d{4}-\d{2}$" => "yyyy-mm",
            r"^\d{4}$" => "yyyy",
            _ => "default",
        });
        assert_eq!(foo, "yyyy-mm-dd");
    }

    #[test]
    fn test_let_match_3_or_more_pattern_block() {
        let foo = regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => {
                let bar = "yyyy-mm-dd";
                bar
            }
            r"^\d{4}-\d{2}$" => {
                let bar = "yyyy-mm";
                bar
            }
            _ => "default",
        });
        assert_eq!(foo, "yyyy-mm-dd");

        let foo = regexm!(match "foo" {
            r"^\d{4}-\d{2}-\d{2}$" => {
                let bar = "yyyy-mm-dd";
                bar
            }
            r"^\d{4}-\d{2}$" => {
                let bar = "yyyy-mm";
                bar
            }
            _ => "default",
        });
        assert_eq!(foo, "default");

        let foo = regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => {
                let bar = "yyyy-mm-dd";
                bar
            }
            r"^\d{4}-\d{2}$" => {
                let bar = "yyyy-mm";
                bar
            }
            r"^\d{4}$" => {
                let bar = "yyyy";
                bar
            }
            _ => "default",
        });
        assert_eq!(foo, "yyyy-mm-dd");
    }

    #[test]
    fn test_let_match_2_or_less_pattern() {
        let foo = regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => "yyyy-mm-dd",
            _ => "default",
        });
        assert_eq!(foo, "yyyy-mm-dd");

        let foo = regexm!(match "foo" {
            r"^\d{4}-\d{2}$" => "yyyy-mm",
            _ => "default",
        });
        assert_eq!(foo, "default");
    }

    #[test]
    fn test_let_match_2_or_less_pattern_block() {
        let foo = regexm!(match "2021-01-01" {
            r"^\d{4}-\d{2}-\d{2}$" => {
                let bar = "yyyy-mm-dd";
                bar
            }
            _ => "default",
        });
        assert_eq!(foo, "yyyy-mm-dd");

        let foo = regexm!(match "foo" {
            r"^\d{4}-\d{2}$" => {
                let bar = "yyyy-mm";
                bar
            }
            _ => "default",
        });
        assert_eq!(foo, "default");
    }

    #[test]
    fn test_capture_groups_match_3_or_more_pattern() {
        regexm!(match "2021-01-01" {
            r"^(\d{4})-\d{2}-\d{2}$" =>
                |caps| assert_eq!(caps.get(1).map_or("", |m| m.as_str()), "2021"),
            r"^\d{4}-\d{2}$" => assert!(false),
            _ => assert!(false),
        });

        regexm!(match "2021-01" {
            r"^\d{4}-\d{2}-\d{2}$" => assert!(false),
            r"^(\d{4})-\d{2}$" => |caps| assert_eq!(caps.get(1).map_or("", |m| m.as_str()), "2021"),
            r"^\d{4}$" => assert!(false),
            _ => assert!(false),
        });
    }

    #[test]
    fn test_capture_groups_match_3_or_more_pattern_block() {
        regexm!(match "2021-01-01" {
            r"^(\d{4})-\d{2}-\d{2}$" => |caps| {
                let year = caps.get(1).map_or("", |m| m.as_str());
                assert_eq!(year, "2021")
            }
            r"^\d{4}-\d{2}$" => {
                assert!(false);
                assert!(false);
            }
            _ => {
                assert!(false);
                assert!(false)
            }
        });

        regexm!(match "2021-01" {
            r"^\d{4}-\d{2}-\d{2}$" => {
                assert!(false);
                assert!(false);
            }
            r"^(\d{4})-\d{2}$" => |caps| {
                let month = caps.get(1).map_or("", |m| m.as_str());
                assert_eq!(month, "2021")
            }
            r"^\d{4}$" => {
                assert!(false);
                assert!(false);
            }
            _ => {
                assert!(false);
                assert!(false)
            }
        });
    }

    #[test]
    fn test_capture_groups_match_2_or_less_pattern() {
        regexm!(match "2021-01-01" {
            r"^(\d{4})-\d{2}-\d{2}$" =>
                |caps| assert_eq!(caps.get(1).map_or("", |m| m.as_str()), "2021"),
            _ => assert!(false),
        });

        regexm!(match "foo" {
            r"^\d{4}-\d{2}$" => assert!(false),
            _ => assert!(true),
        });
    }

    #[test]
    fn test_capture_groups_match_2_or_less_pattern_block() {
        regexm!(match "2021-01-01" {
            r"^(\d{4})-\d{2}-\d{2}$" => |caps| {
                let year = caps.get(1).map_or("", |m| m.as_str());
                assert_eq!(year, "2021");
            }
            _ => {
                assert!(false);
                assert!(false)
            }
        });
    }
}
