use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    #[serde(alias = "user")]
    pub username: String,
    #[serde(alias = "pass")]
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SmtpInfo {
    pub host: String,
    pub port: u16,
    /// Indicates if directly a TLS connection should be opend or a non-TLS connection followed by STARTTLS.
    #[serde(alias="secure")]
    pub use_tls_directly: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImapInfo {
    pub host: String,
    pub port: u16,
    /// Indicates if directly a TLS connection should be opend or a non-TLS connection followed by STARTTLS.
    #[serde(alias="secure")]
    pub use_tls_directly: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pop3Info {
    pub host: String,
    pub port: u16,
    /// Indicates if directly a TLS connection should be opend or a non-TLS connection followed by STARTTLS.
    #[serde(alias="secure")]
    pub use_tls_directly: bool
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WebInfo {
    pub uri: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAndServiceInfo {
    pub account: AccountInfo,

    #[serde(default)]
    pub smtp: Option<SmtpInfo>,
    #[serde(default)]
    pub imap: Option<ImapInfo>,
    #[serde(default)]
    pub pop3: Option<Pop3Info>,
    #[serde(default)]
    pub web: Option<WebInfo>
}


