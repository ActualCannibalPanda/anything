#[macro_export]
macro_rules! add_anything {
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
            let mut anything = anything::Anything::new();
            $(
                anything.insert($x);
            )*
            anything
        }
    };
}
