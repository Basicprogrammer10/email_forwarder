use std::{
    fs::{self, File},
    net::TcpStream,
    ops::{Deref, DerefMut},
    path::Path,
};

use anyhow::Result;
use imap::{Client, Session};
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use native_tls::{Certificate, TlsStream};

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

        let imap_cert = match config.imap.certificate {
            Some(ref x) => Some(Certificate::from_pem(&fs::read(&x)?)?),
            None => None,
        };
        let imap_client = imap::ClientBuilder::new(&config.imap.host, config.imap.port)
            .starttls()
            .connect(|domain, tcp| {
                let mut conn = native_tls::TlsConnector::builder();
                conn.danger_accept_invalid_certs(true);
                if let Some(x) = imap_cert {
                    conn.add_root_certificate(x);
                }

                Ok(conn.build().unwrap().connect(domain, tcp)?)
            })?;
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

    pub fn check_emails(&mut self) -> Result<Vec<Vec<u8>>> {
        self.imap.select("INBOX")?;

        let mut messages = Vec::new();
        for i in self.imap.fetch("1", "RFC822")?.iter() {
            let body = i.body().unwrap_or_default();
            let str = std::str::from_utf8(body).unwrap();
            println!("{}", str);
            messages.push(body.to_vec());
        }

        Ok(messages)
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.imap.logout().unwrap();
    }
}
