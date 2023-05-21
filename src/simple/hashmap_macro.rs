macro_rules! hashmap{
    ($($($key:expr => $val :expr)+$(,)?)*)=>{{
        let mut map = std::collections::hashmap::new();
        $($(map.insert($key,$val);)*)*
        map
    }}
}