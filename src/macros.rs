#[macro_export]
macro_rules! pl {
    () => {
        println!()
    };

    ($($x:expr),*) => {
        $(
            println!("{}", $x)
        )*
    };
}

#[macro_export]
macro_rules! pli {
    ($($x:expr),*) => {
        pl!($x)
    };
}
