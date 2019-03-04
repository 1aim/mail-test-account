use std::fmt::{Debug, Display};

use serde::Serialize;
use serde_json;

use ethereal_mail_test_account::test_account_info;

fn main() {
    match test_account_info() {
        Ok(info) => print_as_json(&info),
        Err(err) => print_error(&err)
    }
}


fn print_as_json(data: &impl Serialize) {
    println!("{}", serde_json::to_string_pretty(data).unwrap());
}

fn print_error<T>(err: &T)
    where T: Display + Debug
{
    eprintln!("Error: {err}\nDetails: {err:?}", err=err);
}