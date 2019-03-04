//! A library for creating/storing/loading ethereal.mail accounts.
//!
//! **Disclaimer: This source code/library/crate is unrelated to the original ethereal.mail service**, through it
//! calls their api.
//!
//!

use log::{warn, debug};
use failure::Error;

const APP_NAME: &str = "ethereal-mail-test-account";
const DEFAULT_TAG: &str = "test_account";

mod types;
mod new_account;
mod load_store_account;


pub use crate::{
    load_store_account::*,
    types::*,
    new_account::*,
};


pub fn test_account_info() -> Result<AccountAndServiceInfo, Error> {
    test_account_info_with_tag(DEFAULT_TAG)
}

pub fn test_account_info_with_tag(tag: &str) -> Result<AccountAndServiceInfo, Error> {
    if let Some(info) = load_account_info(tag)? {
        Ok(info)
    } else {
        let info = create_new_ethereal_account()?;

        let res = store_account_info(tag, &info);
        if let Err(err) = res {
            warn!("could not store new test account: {err}\ndbg: {err:?}", err=err);
            debug!("non-stored ethereal.mail credentials: {:?}", &info.account);
        }

        Ok(info)
    }
}
