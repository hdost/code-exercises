#[macro_export]
macro_rules! hashmap {
    () => { ::std::collections::HashMap::new() };
    ($($i:expr => $j:expr),+$(,)?) => {
        {
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($i,$j);
        )*
        map
        }
    };
}
/*
()
(some => some)
(some => some,)
(some => some, na => na)
(some => some, na => na,)
*/
