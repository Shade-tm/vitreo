use imap;
use native_tls::{TlsConnector, TlsStream};
use std::fs::File;
use std::io::{self, BufRead};
use std::net::TcpStream;
use std::path::Path;


pub fn connect_client () -> imap::Session<TlsStream<TcpStream>>{
    let domain = "imap.gmail.com";
    let tls = TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, 993), domain, &tls).unwrap();
    let (mail, pass) = get_account_info();
    let imap_session = client.login(mail, pass).unwrap();

    imap_session
}

pub fn get_mails (imap_session: &mut imap::Session<TlsStream<TcpStream>>) {
    imap_session.select("INBOX").unwrap();

    let messages = imap_session.fetch("1", "RFC822").unwrap();

    for message in messages.iter() {
        if let Some(body) = message.body() {
            println!("{}", std::str::from_utf8(body).unwrap());
        } else {
            println!("Message didn't have a body!");
        }
    }
    imap_session.logout().unwrap();
}

fn get_account_info () -> (String, String) {
    let path = Path::new("./account_data.txt");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };

    let mut lines = io::BufReader::new(file).lines();
    let mail = lines.next().unwrap().unwrap();
    let pass = lines.next().unwrap().unwrap();
    (mail, pass)
}