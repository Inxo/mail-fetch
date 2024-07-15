use imap::Session;
use native_tls::{TlsConnector, TlsStream};
use std::net::TcpStream;

pub fn connect(server: &str, port: u16, email: &str, password: &str, tls: &TlsConnector) -> anyhow::Result<Session<TlsStream<TcpStream>>> {
    let client = imap::connect((server, port), server, &tls)?;
    let imap_session = client.login(email, password).map_err(|e| e.0)?;
    Ok(imap_session)
}