
use tera::{Tera,Context};


pub fn get_template_context() -> Result<(Tera, Context),()> {
    // Use globbing
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    // fresh tera context
    let context = Context::new();  

    Ok((tera, context))
}