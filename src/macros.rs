#[macro_export]
macro_rules! parse_input { ($x:expr, $t:ident) => ($x.trim().parse::<$t>().expect(&format!("Cannot parse '{}'",$x))) }
