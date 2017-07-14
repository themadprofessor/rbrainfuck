#[macro_export]
macro_rules! string_build {
    () => {{String::new()}};
    ($s: expr) => {{$s.to_string()}};
    ($($s: expr),+) => {{
        let mut len = 0;
        $(len += $s.len();)+;
        let mut string = String::with_capacity(len);
        $(string.push_str($s);)+;
        string
    }}
}

#[macro_export]
macro_rules! is_one {
    ($i: ident, $next: expr, $($opt:expr),+) => {{
        $i == $next || is_one!($i, $($opt),*)
    }};

    ($i: ident, $opt:expr) => {{
        $i == $opt
    }}
}

/*
macro_rules! is_intern {
    ($i: ident, $next: expr, $($next:expr),+) => {{
        $i == $next || is_intern!($i, $next)
    }};

    ($i: ident, $last: expr) => {{
        $i == $last
    }}
}
*/

#[cfg(test)]
mod tests {
    use test;

    #[test]
    fn string_build_empty() {
        assert_eq!(String::new(), string_build!());
    }

    #[test]
    fn build_single() {
        let s = "Testing";
        assert_eq!(s, string_build!(s));
    }

    #[test]
    fn string_build_multi() {
        assert_eq!("testdootdeet", string_build!("test", "doot", "deet"));
    }

    #[bench]
    fn string_build(b: &mut test::Bencher) {
        b.iter(|| string_build!("asdf", "qwerdf", "asfdgh", "ghjk", "sasldkfs"));
    }

    #[test]
    fn is_one_single_char() {
        let c = '+';
        assert_eq!(true, is_one!(c, '+'));
    }

    #[test]
    fn is_one_multi_char() {
        let c = '+';
        assert_eq!(true, is_one!(c, '-', '>', '+'));
    }

    #[test]
    fn is_one_fail_single_char() {
        let c = '+';
        assert_eq!(false, is_one!(c, '-'));
    }

    #[test]
    fn is_one_fail_multi_char() {
        let c = '+';
        assert_eq!(false, is_one!(c, '-', '>', '.'));
    }
}