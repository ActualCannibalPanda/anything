#[macro_export]
macro_rules! add_multiple {
    ($anything:ident, $($x:expr),*) => {
        $(
            $anything.insert($x);
        )*
    };
}

#[macro_export]
macro_rules! create_anything {
    ($($x:expr),*) => {
        {
            let mut anything = Anything::new();
            $(
                anything.insert($x);
            )*
            anything
        }
    };
}
