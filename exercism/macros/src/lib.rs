#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {{
        #[allow(unused_mut)]
        let mut hm = ::std::collections::HashMap::new();
        $(hm.insert($key, $value);)*
        hm
    }};
    ($($key:expr => $value:expr,)*) => {
        $crate::hashmap!($($key => $value),*)
    };
}
