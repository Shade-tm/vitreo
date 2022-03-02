use session;

fn main() {
    let mut imap_session = session::mail_sessions::connect_client();
    session::mail_sessions::get_mails(&mut imap_session);
}