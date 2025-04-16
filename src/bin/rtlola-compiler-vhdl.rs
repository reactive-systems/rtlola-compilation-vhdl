use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = rtlola_compiler_vhdl::Config::new(&args)?;

    config.generate_vhdl_files();

    Ok(())
}
