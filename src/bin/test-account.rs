
use std::fmt::{Debug, Display};

use clap::{App, Arg, crate_version};
use serde::Serialize;
use serde_json;

use mail_test_account::test_account_info;

const ETHEREAL_MAIL: &str = "ethereal.mail";

fn main() {
    let matches = App
        ::new("mail-test-account")
        .version(crate_version!())
        .author("Philipp Korber <p.korber@1aim.com>")
        .about("Return info for a mail account for testing purpose")
        .arg(Arg::with_name("account-provider")
            .short("a")
            .long("account-provider")
            .value_name("NAME")
            .help("Set the provider for test accounts (currently only ethereal.mail).")
            .takes_value(true))
        .get_matches();

    match matches.value_of("account-provider").unwrap_or(ETHEREAL_MAIL) {
        ETHEREAL_MAIL => (),
        _ => {
            eprintln!("account provider not supported, supported providers are:");
            eprintln!("- ethereal.mail");
            ::std::process::exit(1);
        }
    }

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
