#[macro_export]
macro_rules! inherit {
    ($name:ident) => {
        pub mod $name;
        pub use $name::$name;
    };
}

#[macro_export]
macro_rules! inherits {
    ($name:ident, [$($inherit:ident),+]) => {
        pub mod $name;
        $(
            pub use $name::$inherit;
        )+
    };
}
