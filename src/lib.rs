#![no_std]

#[macro_export]
macro_rules! fn_compose {
    ( $arg: expr ) => { $arg };
    ( $arg: expr => $f: ident ) => { $f($arg) };
    ( $arg: expr => $f: ident $( $largs: expr, )* _ $( ,$rargs: expr )* ) => {
        $f($($largs,)* $arg $(,$rargs)*)
    };
    ( $arg: expr => $f: ident => $( $tokens: tt )* ) => {{
        fn_compose!($f($arg) => $($tokens)*)
    }};
    ( $arg: expr => $f: ident $( $largs: expr, )* _ $( ,$rargs: expr )* => $( $tokens: tt )* ) => {{
        fn_compose!($f($($largs,)* $arg $(,$rargs)*) => $($tokens)*)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_fn() {
        assert_eq!(fn_compose!(4), 4);
    }

    #[test]
    fn one_simple_fn() {
        let add_one = |x| x + 1;
        assert_eq!(fn_compose!(4 => add_one), 5);
    }

    #[test]
    fn one_arg_fn() {
        let add = |x, y| x + y;
        assert_eq!(fn_compose!(4 => add 5, _), 9);
    }

    #[test]
    fn two_simple_fn() {
        let add_one = |x| x + 1;
        let double = |x| 2 * x;
        assert_eq!(fn_compose!(4 => double => add_one), 9);
    }

    #[test]
    fn two_arg_fn() {
        let mul = |a, b, c, d, e| a * b * c * d * e;
        let add = |a, b, c| a + b + c;
        assert_eq!(fn_compose!(4 => add 3, _, 9 => mul 1, 4, 2, _, 10), 1280);
    }
}
