#![feature(log_syntax)]

#[macro_export]
macro_rules! logging{
    (@eval) => {};
    (@eval $head:tt $($tail:tt)*) => {
        log_syntax!($head);
        logging!(@eval $($tail)*);
    };

    ($($body:tt),*) => {
        logging!(@eval $($body)*);
    };
}

#[macro_export]
macro_rules! count0 {
    (@convert $_:tt) => { () };
    ($($body:tt),*) => {
        <[()]>::len(&[$(count0!(@convert $body)),*])
    };
}

#[macro_export]
macro_rules! count1 {
    (@accum $n:tt) => {
        $n
    };
    (@accum $n:tt $_head:tt $($tail:tt)*) => {{
        count1!(@accum ($n + 1) $($tail)*)
    }};

    () => {
        0
    };
    ($($body:tt),*) => {
        count1!(@accum 0 $($body)*)
    };
    ($($body:tt),*,) => {
        count1!($($body),*)
    };
}

#[macro_export]
macro_rules! hash_map {
    (@empty $_:expr) => {()};
    (@count $($pairs:expr)*) =>  {
        <[()]>::len(&[$(hash_map!(@empty $pairs)),*])
    };

    ($(($key:expr, $val:expr)),*) => {{
        use std::collections::HashMap;

        let count = hash_map!(@count $(($key, $val))*);
        let mut ret = HashMap::with_capacity(count);
        $(ret.insert($key, $val);)*
        ret
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count0() {
        assert_eq!(0, count0!());
        assert_eq!(1, count0!(1));
        assert_eq!(2, count0!(1, 2));
        assert_eq!(2, count0!(1, (1 + 1)));
    }

    #[test]
    fn count1() {
        assert_eq!(0, count1!());
        assert_eq!(1, count1!(1));
        assert_eq!(2, count1!(1, 2));
        assert_eq!(2, count1!(1, (1 + 1)));
    }

    #[test]
    fn hash_map() {
        let expected = {
            let mut ret = std::collections::HashMap::new();
            ret.insert("aaa", 3);
            ret.insert("bbb", 4);
            ret
        };

        assert_eq!(expected, hash_map!(("aaa", 2 + 1), ("bbb", 4)));
    }

    #[test]
    fn logging() {
        logging!(foo, bar, baz);
    }
}
