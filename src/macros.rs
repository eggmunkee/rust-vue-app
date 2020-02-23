#![macro_use]

#[macro_use]
macro_rules! col_def { ($field:literal,$type:literal) => { format!("{} {}", &$field, &$type) } }
#[macro_use]
macro_rules! val_text { ($field:literal) => { format!("'{}'", &$field) } }
#[macro_use]
macro_rules! val_int { ($field:literal) => { format!("{}", &$field) } }
#[macro_use]
macro_rules! col_int { ($field:literal) => { col_def!($field,"INTEGER")  } }
#[macro_use]
macro_rules! col_text { ($field:literal) => { col_def!($field,"TEXT") } }
// define a list of comma-sep columns
#[macro_use]
macro_rules! columns {
    ($expr1:expr) => { format!("{}", $expr1) };
    ($expr1:expr, $($expr2:expr), *) => {
        format!("{}, {}", $expr1, columns!($($expr2)+))
    }
}
// define a table of a given name, with a list of columns (can use columns! to generate)
#[macro_use]
macro_rules! create_table {
    ($table_name:literal, $columns:expr) => {
        format!("CREATE TABLE {} ({}); ", &$table_name, $columns)
    };
}
#[macro_use]
macro_rules! drop_table {
    ($table_name:literal, $columns:expr) => {
        format!("DROP TABLE {}; ", &$table_name, $columns)
    };
}
#[macro_export]
macro_rules! insert_row {
    ($table_name:literal, $columns:expr) => {
        format!("INSERT INTO {} VALUES ({}); ", &$table_name, $columns)
    }
}