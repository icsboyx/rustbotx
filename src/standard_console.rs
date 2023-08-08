
#[macro_export]
macro_rules! c_println {
    ($input:expr) => {{
        use colored::*;
        let message = &($input);
        println!("{}{}", format!("[{}]",module_path!()).blue().bold().underline(), message );
        // let hash_string: String = std::iter::repeat("#").take(100).collect();
        // println!("{}",hash_string.blue().bold().underline());
    }};
}