#[macro_export]
macro_rules! pl {
    () => {
        println!();
    };

    ($($x:expr),*) => {
        $(
            println!("{}", $x);
        )*
    };
}
