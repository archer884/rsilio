extern crate hyper;

use hyper::Client;

pub struct MessagingService {
    sid: String,
    token: String,
    outgoing_number: String,
}

impl MessagingService {
    pub fn new<T: Into<String>>(sid: T, token: T, outgoing_number: T) -> MessagingService {
        MessagingService {
            sid: sid.into(),
            token: token.into(),
            outgoing_number: outgoing_number.into(),
        }
    }

    pub fn send_message(&self, recipient: &str, message: &str) -> Result<(), ()> {
        Client::new().post(&format!(
            "https://{sid}:{token}@api.twilio.com/2010-04-01/Accounts/{sid}/Messages",
            sid = self.sid,
            token = self.token,
        )).body(&format!(
            "To={recipient}\nFrom={sender}\nBody={text}",
            recipient = recipient,
            sender = self.outgoing_number,
            text = message,
        )).send().map(|_| ()).map_err(|_| ())
    }
}
