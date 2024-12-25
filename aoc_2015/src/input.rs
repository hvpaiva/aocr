#[macro_export]
macro_rules! input {
    ($day:literal) => {{
        include_str!(concat!("../input/2015/", $day, ".txt"))
    }};
}
