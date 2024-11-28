// https://stackoverflow.com/a/44378174/4276533 for measuring time
#[macro_export] macro_rules! measure_time {
    ($f:ident($($arg:expr),*)) => {{
        println!("Measuring the function: {}", stringify!($f));
        let start = std::time::SystemTime::now();

        let return_value = $f($($arg),*);

        let time_elapsed = std::time::SystemTime::now()
            .duration_since(start)
            .expect("Time went backwards");
        println!("It took {:?}", time_elapsed);

        return_value
    }};
}

