use crate::{error, help, code_block};



pub fn missing_main_fn() {
    error!("Main function not found, please create one lol");
    help!("Heres a basic main function with code that prints hello world:\n{}", 
        code_block!(
            concat!(
                "include \"std.mcl\"\n",
                "\n",
                "fn main with void retuns void then\n",
                "    \"Hello world!\\n\" puts\n",
                "done\n"
            )
        )
    );
}