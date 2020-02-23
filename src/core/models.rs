
// define columns of types
//use crate::{create_table, columns, col_text, col_in, insert_row, val_text, val_int};

pub fn init_core(connection: &mut sqlite::Connection) {
    let cmd = create_table!("app_init",columns!(col_text!("table_name"),col_int!("init")));
    match connection.execute( &cmd ) {
        Ok(_) => { println!("App_init [created].\nCmd: {}", &cmd); },
        Err(e) => {
            println!("App__init already exists.. skipping.\n Error: {}\n Cmd: {}", e, &cmd);
        }
    }
    
}

// pub fn init_table(connection: &sqlite::Connection, name: &str)  {
//     cmd = insert_row!("app__init",columns!(val_text!(&name),val_int!(1)));
//     match connection.execute( &cmd ) {
//         Ok(_) =>  { println!("App__init Users [created]."); },
//         Err(e) => {
//             println!("Error adding Users init value..\n. Error: {}\n Cmd: {}", e, &cmd);
//         }
//     }
// }

pub fn is_table_inited(connection: &sqlite::Connection, name: &str) -> bool {
    let cmd = format!("SELECT count(*) FROM app_init WHERE table_name = '{}' and init > 0", name );
    
    match connection.prepare( &cmd ) {
        Ok(s) if s.count() > 0 => {
            println!("count > 0 case, cmd: {}", &cmd);
             true
        }, // success - true if count > 0
        Err(e) => {
            println!("Error checking is_table_inited: {}", e);
            false
        },
        _ => {
            println!("Other case");
            false
        },  // not inited
    }
}

