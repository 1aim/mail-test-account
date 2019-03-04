# mail-test-account &emsp;

Documentation can be [viewed on docs.rs](https://docs.rs/mail).

A library for loading/storing and creating new mail test accounts.
The accounts will be for servers/services which provide a
Mail Submission Agent (MSA) which accepts but never delivers mails
and from which you can access the mail.

Currently `ethereal.mail` is used as a service for creating new accounts.

Before creating a account the library tries to load account infos from
a config dir (using the xdg directory spec). The default account stored
and loaded by the `test_account_info()` function are stored at
`~/.config/mail-test-account/test_account.json`. If a different tag is
used the file name will change (the default tag is `test_account`).
The config dir can be changed in the same way you can change a xdg
config dir (E.g. using the `XDG_CONFIG_HOME` environment variable).


## Targets

This crate can be used in three ways:

1. As a library (intended usage).
2. As a command line tool (`cargo run --features="clap"`),
   which outputs pretty printed JSON.
3. Run the readme example (`cargo run --example readme`),
   which outputs information formated for human consumption.

## Example

You can run the example below with `cargo run --example readme`.

```rust
use mail_test_account::test_account_info;

fn main() {
    let info = test_account_info().unwrap();

    println!("ACCOUNT/CREDENTIALS");
    println!(" username: {}", info.account.username);
    println!(" password: {}", info.account.password);


    if let Some(smtp) = info.smtp {
        println!("SMTP");
        println!(" host: {}", smtp.host);
        println!(" port: {}", smtp.port);
        if smtp.use_tls_directly {
            println!(" use non standard direct TLS instead of STARTTLS");
        } else {
            println!(" use STARTTLS");
        }
    }

    if let Some(imap) = info.imap {
        println!("IMAP");
        println!(" host: {}", imap.host);
        println!(" port: {}", imap.port);
    }

    if let Some(pop3) = info.pop3 {
        println!("POP3");
        println!(" host: {}", pop3.host);
        println!(" port: {}", pop3.port);
    }

    if let Some(web) = info.web {
        println!("WEBSITE");
        println!(" uri: {}", web.uri);
    }
}
```

Outputs something like:

```
ACCOUNT/CREDENTIALS
 username: ----@ethereal.email
 password: -------------------
SMTP
 host: smtp.ethereal.email
 port: 587
 use STARTTLS
IMAP
 host: imap.ethereal.email
 port: 993
POP3
 host: pop3.ethereal.email
 port: 995
WEBSITE
 uri: https://ethereal.email
```


## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
