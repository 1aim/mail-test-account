use std::{
    path::Path,
    fs::File,
    io::{BufReader, BufWriter}
};

use serde::{Serialize, de::DeserializeOwned};
use serde_json;
use failure::{format_err, Error};
use xdg::BaseDirectories;

use crate::types::AccountAndServiceInfo;

/// Stores given [`AccountAndServiceInfo`] instance as json in the config dir.
///
/// The config dir is the dir defined through the xdg directory spec (i.e. it can
/// be influenced by environment variables). The default is:
/// `~/.config/ethereal-mail-test-account/<tag>.json` (replacing `<tag>` with the
/// tag).
///
/// # Error
///
/// This can fail for a number of reasons:
///
/// 1. The tag does not match `[a-Z0-9_-]+`.
/// 2. Getting config file path fails for one reason or another.
/// 3. Writing to the config file path fails.
pub fn store_account_info(tag: &str, info: &AccountAndServiceInfo) -> Result<(), Error> {
    let file_name = create_file_name(tag)?;

    let base = BaseDirectories::with_prefix(crate::APP_NAME)?;
    let path = base.place_config_file(file_name)?;
    write_json_file(path, info)?;
    Ok(())
}

/// Loads a [`AccountAndServiceInfo`] instance from a json file from the config dir.
///
/// The config dir is the dir defined through the xdg directory spec (i.e. it can
/// be influenced by environment variables). The default is:
/// `~/.config/ethereal-mail-test-account/<tag>.json` (replacing `<tag>` with the
/// tag).
///
/// If no config file is found `Ok(None)` is returned.
///
/// # Error
///
/// This can fail for a number of reasons:
///
/// 1. The tag does not match `[a-Z0-9_-]+`.
/// 2. Getting config file path fails for one reason or another.
/// 3. Reading the config file path fails (but the path does exist,
///    else it would have returned `Ok(None)`).
///
pub fn load_account_info(tag: &str) -> Result<Option<AccountAndServiceInfo>, Error> {
    let file_name = create_file_name(tag)?;

    let base = BaseDirectories::with_prefix(crate::APP_NAME)?;

    if let Some(file) = base.find_config_file(file_name) {
        let info = read_json_file(file)?;
        Ok(Some(info))
    } else {
        Ok(None)
    }
}

fn create_file_name(tag: &str) -> Result<String, Error> {
    check_tag(tag)?;
    Ok(format!("{}.json", tag))
}

fn check_tag(tag: &str) -> Result<(), Error> {
    if tag_is_valid(tag) {
        Ok(())
    } else {
        Err(format_err!("invalid tag, tag must match [a-Z0-9_-]+ but got: {:?}", tag))
    }
}

fn tag_is_valid(tag: &str) -> bool {
    tag.len() > 0 && tag.bytes().all(valid_tag_bch)
}

fn valid_tag_bch(bch: u8) -> bool {
    bch.is_ascii_alphanumeric() || bch == b'_' || bch == b'-'
}

fn write_json_file(file: impl AsRef<Path>, data: &impl Serialize) -> Result<(), Error> {
    let file = File::create(file)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, data)?;
    Ok(())
}

fn read_json_file<R>(file: impl AsRef<Path>) -> Result<R, Error>
    where R: DeserializeOwned
{
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let value = serde_json::from_reader(reader)?;
    Ok(value)
}



#[cfg(test)]
mod tests {


    mod tag_is_valid {
        use super::super::*;

        #[test]
        fn min_tag_len_1() {
            assert_eq!(tag_is_valid(""), false);
            assert_eq!(tag_is_valid("a"), true);
        }

        #[test]
        fn rejects_slash_and_backslash() {
            assert_eq!(tag_is_valid("/"), false);
            assert_eq!(tag_is_valid("\\"), false);
            assert_eq!(tag_is_valid("a/"), false);
            assert_eq!(tag_is_valid("a\\"), false);
            assert_eq!(tag_is_valid("a/a"), false);
            assert_eq!(tag_is_valid("a\\a"), false);
        }

        #[test]
        fn rejects_space() {
            assert_eq!(tag_is_valid("a b"), false);
            assert_eq!(tag_is_valid("a\tb"), false);
            assert_eq!(tag_is_valid("a\rb"), false);
            assert_eq!(tag_is_valid("a\nb"), false);
            assert_eq!(tag_is_valid("a b"), false);
        }

        #[test]
        fn allow_hyphon_and_underline() {
            assert_eq!(tag_is_valid("-"), true);
            assert_eq!(tag_is_valid("-a"), true);
            assert_eq!(tag_is_valid("a-"), true);
            assert_eq!(tag_is_valid("a-b"), true);
            assert_eq!(tag_is_valid("_"), true);
            assert_eq!(tag_is_valid("_a"), true);
            assert_eq!(tag_is_valid("a_"), true);
            assert_eq!(tag_is_valid("a_b"), true);
        }
    }
}