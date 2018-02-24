#[macro_export]
macro_rules! min {
    ( $x:expr, $( $y:expr ),+ ) => {
        {
            let mut a = $x;
            $(
                a = min(a, $y);
            )*
            a
        }
    };
}

#[macro_export]
macro_rules! max {
    ( $x:expr, $( $y:expr ),+ ) => {
        {
            let mut a = $x;
            $(
                a = max(a, $y);
            )*
            a
        }
    };
}