/// # Examples
///
/// ```
/// # #[macro_use] extern crate rust_macro_examples;
///
/// assert_eq!(0, count!());
/// assert_eq!(1, count!(3));
/// assert_eq!(2, count!(3, 5));
/// ```
#[macro_export]
macro_rules! count {
    () => {
        0usize
    };
    ($($vals:stmt),*) => {count!($($vals)* ,)};
    ($_head:stmt, $($tail:stmt),*) => {
        1usize + count!($($tail)*)
    };
}

// /// # Examples
// ///
// /// ```
// /// # #[macro_use] extern crate rust_macro_examples;
// ///
// /// let my_vec: Vec<i32> = my_vec![1,2,3];
// /// let vec: Vec<i32> = vec![1,2,3];
// ///
// /// assert_eq!(vec, my_vec);
// /// ```
// #[macro_export]
// macro_rules! my_vec {
//     (@count, $n:tt,) => { $n };
//     (@count, $n:tt, $_head:tt, $($tail:tt),*) => {
//         let n = $n;
//         my_vec!(@count, n + 1, $($tail),*)
//     };
//     ($($vals:tt),*) => {
//         let count = my_vec!(@count, 0, $($vals),*);
//         let mut ret = Vec::with_capacity(count);
//         {
//             let v = $vals;
//             ret.push(v);
//         }*;
//         ret
//     };
// }
