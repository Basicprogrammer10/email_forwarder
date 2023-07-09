use std::{fs, net::TcpStream, path::Path};

use anyhow::Result;
use imap::Session;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use native_tls::{TlsConnector, TlsStream};

use crate::config::Config;

pub struct App {
    config: Config,
    imap: Session<TlsStream<TcpStream>>,
    smtp: SmtpTransport,
}

impl App {
    pub fn new(config: impl AsRef<Path>) -> Result<Self> {
        let raw_config = fs::read_to_string(config)?;
        let config = toml::from_str::<Config>(&raw_config)?;

        let imap_tls = TlsConnector::builder().build()?;
        let imap_client = imap::connect(
            (config.imap.host.to_owned(), config.imap.port),
            &config.imap.host,
            &imap_tls,
        )?;
        let imap = imap_client
            .login(&config.imap.username, &config.imap.password)
            .unwrap();

        let smtp_credentials = Credentials::new(
            config.smtp.username.to_owned(),
            config.smtp.password.to_owned(),
        );
        let smtp = SmtpTransport::relay(&config.smtp.host)
            .unwrap()
            .credentials(smtp_credentials)
            .build();

        Ok(Self { config, imap, smtp })
    }

    pub fn check_emails(&mut self) -> Result<()> {
        self.imap.select("INBOX")?;

        // fetch message number 1 in this mailbox, along with its RFC822 field.
        // RFC 822 dictates the format of the body of e-mails
        let messages = self.imap.fetch("1", "RFC822")?;
        let message = if let Some(m) = messages.iter().next() {
            m
        } else {
            return Ok(());
        };

        // extract the message's body
        let body = message.body().expect("message did not have a body!");
        let body = std::str::from_utf8(body)
            .expect("message was not valid utf-8")
            .to_string();

        dbg!(body);

        Ok(())
    }
}

// impl std::error::Error for  {}
