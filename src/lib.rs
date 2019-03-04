//! A library for creating/storing/loading ethereal.mail accounts.
//!
//! **Disclaimer: This source code/library/crate is unrelated to the original ethereal.mail service**, through it
//! calls their api.
//!
//!

use log::{warn, debug};
use failure::Error;

const APP_NAME: &str = "mail-test-account";
const DEFAULT_TAG: &str = "test_account";

mod types;
mod new_account;
mod load_store_account;


pub use crate::{
    load_store_account::*,
    types::*,
    new_account::*,
};


/// Get the "test_account" tagged test account, if there is non create one and store it.
///
/// Take a look at [`test_account_info_with_tag()`] for more details about where the
/// account info is stored.
pub fn test_account_info() -> Result<AccountAndServiceInfo, Error> {
    test_account_info_with_tag(DEFAULT_TAG)
}

/// Get the test account for the given tag, if there is non create one and store it.
///
/// This uses the XDG directory spec for storing the account (as json). By default
/// it will store the account in `~/.config/mail-test-account/<tag>.json`.
///
/// If no account is available for given tag it will create a `ethereal.mail` account
/// and store it. The created accounts have random username/mail-address and password.
/// No mail send to them get ever delivered, or semantically validated. So while you can
/// use it for testing, it will _not_ fail if you e.g. switch up from and to address. But
/// you can use IMAP/POP3 to pull mails from there and validate them.
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
