use std::{
    default::Default,
};

use serde::{Serialize, Deserialize};
use failure::{Error, format_err, err_msg};
use reqwest::Client;
use serde_json;

use crate::types::*;

const NODEMAILER_NEW_USER_ENDPOINT: &str = "https://api.nodemailer.com/user";

pub fn create_new_ethereal_account() -> Result<AccountAndServiceInfo, Error> {
    let response: EtherealAccountResponse = Client::new()
        .post(NODEMAILER_NEW_USER_ENDPOINT)
        .json(&NodeMailerRequestCrateId::default())
        .send()?
        .json()?;

    response.into()
}

#[derive(Debug, Deserialize)]
struct EtherealAccountResponse {
    status: String,

    #[serde(default)]
    error: Option<serde_json::Value>,

    #[serde(default)]
    smtp: Option<SmtpInfo>,
    #[serde(default)]
    imap: Option<ImapInfo>,
    #[serde(default)]
    pop3: Option<Pop3Info>,
    #[serde(default)]
    web: Option<WebInfo>,

    user: Option<String>,
    pass: Option<String>
}

impl Into<Result<AccountAndServiceInfo, Error>>  for EtherealAccountResponse {

    fn into(self) -> Result<AccountAndServiceInfo, Error> {
        let EtherealAccountResponse {
            status, error,
            smtp, imap, pop3, web,
            user, pass
        } = self;

        if status != "success" {
            let error = error.unwrap_or(serde_json::Value::Null);
            return Err(format_err!("unexpected response, error={}", error));
        } else {
            if user.is_none() || pass.is_none() {
                return Err(err_msg("missing account info"));
            }
            let account = AccountInfo {
                username: user.unwrap(),
                password: pass.unwrap()
            };

            Ok(AccountAndServiceInfo {
                account,
                smtp, imap, pop3, web,
            })
        }
    }
}


#[derive(Debug, Serialize)]
struct NodeMailerRequestCrateId {
    requestor: String,
    version: String
}

impl Default for NodeMailerRequestCrateId {
    fn default() -> Self {
        NodeMailerRequestCrateId {
            requestor: env!("CARGO_PKG_NAME").to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    mod EtherealAccountResponse {
        use serde_json::{self, json};
        use super::super::*;

        #[test]
        fn deserialize_normal_response() {
            let input = json!({
                "status": "success",
                "user": "foo@example.com",
                "pass": "321",
                "smtp": {
                    "host": "smtp.example.com",
                    "port": 123,
                    "secure": false
                },
                "imap": {
                    "host": "imap.example.com",
                    "port": 124,
                    "secure": true
                },
                "pop3": {
                    "host": "pop3.example.com",
                    "port": 125,
                    "secure": true
                },
                "web": "https://web.example.com"
            });

            let res: Result<_,_> = serde_json
                ::from_value::<EtherealAccountResponse>(input)
                .unwrap()
                .into();

            let info = res.unwrap();

            assert_eq!(info.account.username, "foo@example.com");
            assert_eq!(info.account.password, "321");
            let smtp = info.smtp.unwrap();
            assert_eq!(smtp.host, "smtp.example.com");
            assert_eq!(smtp.port, 123);
            assert_eq!(smtp.use_tls_directly, false);

            let imap = info.imap.unwrap();
            assert_eq!(imap.host, "imap.example.com");
            assert_eq!(imap.port, 124);
            assert_eq!(imap.use_tls_directly, true);

            let pop3 = info.pop3.unwrap();
            assert_eq!(pop3.host, "pop3.example.com");
            assert_eq!(pop3.port, 125);
            assert_eq!(pop3.use_tls_directly, true);

            let web = info.web.unwrap();
            assert_eq!(web.uri, "https://web.example.com");
        }

        #[test]
        fn deserialize_error() {
            let input = json!({
                "status": "error",
                "error": "foobar"
            });

            let res: Result<_,_> = serde_json
                ::from_value::<EtherealAccountResponse>(input)
                .unwrap()
                .into();

            let err = res.unwrap_err();
            let err = format!("{}", err);
            assert_eq!(err.as_str(), "unexpected response, error=\"foobar\"")
        }

        #[test]
        fn deserialize_complex_error() {
            let input = json!({
                "status": "error",
                "error": { "foobar" : "barfoot" }
            });

            let res: Result<_,_> = serde_json
                ::from_value::<EtherealAccountResponse>(input)
                .unwrap()
                .into();

            let err = res.unwrap_err();
            let err = format!("{}", err);
            assert_eq!(err.as_str(), r#"unexpected response, error={"foobar":"barfoot"}"#)
        }
    }

}