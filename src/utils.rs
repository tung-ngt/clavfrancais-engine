#[macro_export] macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*)
    };
}
