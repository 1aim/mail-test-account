use serde::{Serialize, Deserialize};

// Username and Password
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    /// Username
    ///
    /// Can be crated from a serde (de-)serialization format using `user` instead of `username`.
    #[serde(alias = "user")]
    pub username: String,

    /// Password
    ///
    /// Note no special password clearing or hiding mechanism is used, it's just a string.
    /// It's fine as this is _only_ for throwaway test accounts which don't deliver any mail.
    ///
    /// Can be created from a serde (de-)serialization format using `pass` instead of `password`.
    #[serde(alias = "pass")]
    pub password: String,
}

/// Info needed for connecting with the test account's MSA using SMTP.
#[derive(Debug, Serialize, Deserialize)]
pub struct SmtpInfo {
    /// The host/domain name.
    ///
    /// For example, `smtp.ethereal.mail`.
    pub host: String,

    /// The port to connect to.
    pub port: u16,

    /// Indicates if directly a TLS connection should be opend or a non-TLS connection followed by STARTTLS.
    #[serde(alias="secure")]
    pub use_tls_directly: bool
}

/// Info needed for connecting with a IMAP server which contains all mails "send" through the test account.
#[derive(Debug, Serialize, Deserialize)]
pub struct ImapInfo {
    /// The host/domain name.
    pub host: String,
    /// The port to connect to.
    pub port: u16,
    /// Indicates if directly a TLS connection should be opend or a non-TLS connection followed by STARTTLS.
    #[serde(alias="secure")]
    pub use_tls_directly: bool
}

/// Info needed for connecting with a POP3 server which contains all mails "send" through the test account.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pop3Info {
    /// The host/domain name.
    pub host: String,
    /// The port to connect to.
    pub port: u16,
    /// Indicates if directly a TLS connection should be opend or a non-TLS connection followed by STARTTLS.
    #[serde(alias="secure")]
    pub use_tls_directly: bool
}

/// A a Website which allows accessing mails send through the test account.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WebInfo {

    /// The URI to the Website.
    pub uri: String
}

/// The full info needed to use the test account, including credentials and server info.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAndServiceInfo {

    /// The authentication credentials.
    pub account: AccountInfo,

    /// The SMTP server info.
    #[serde(default)]
    pub smtp: Option<SmtpInfo>,

    /// The IMAP server info.
    #[serde(default)]
    pub imap: Option<ImapInfo>,

    /// The POP3 server info.
    #[serde(default)]
    pub pop3: Option<Pop3Info>,

    /// The websites server info.
    #[serde(default)]
    pub web: Option<WebInfo>
}


