#[macro_export]
macro_rules! old_println {
    ($input:expr) => {{
        use colored::*;
        let message = &($input);
        println!(
            "{}{}",
            format!("[{}]", module_path!()).blue().bold().underline(),
            message
        );
    }};
}
#[macro_export]
macro_rules! c_println {
    ($input:expr) => {{
        use colored::*;
        let message = $input;

        let module = format!("[{}]", module_path!());
        let formatted_module = module.blue().bold().underline();

        let module_length = formatted_module.chars().count();
        let padding = " ".repeat(module_length);

        let padded_message = message
            .lines()
            .enumerate()
            .map(|(index, line)| {
                if index == 0 {
                    line.to_string()
                } else {
                    format!("{}{}", padding, line)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");
        println!("{} {}", formatted_module, padded_message);
    }};
}


#[macro_export]
macro_rules! new_c_println {
($input:expr) => {
    //todo:
};
}