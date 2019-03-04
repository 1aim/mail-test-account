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