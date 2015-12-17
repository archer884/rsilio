extern crate hyper;
extern crate time;

use hyper::Client;

pub struct MessagingService {
    sid: String,
    outgoing_number: String,
}

impl MessagingService {
    pub fn new<T: Into<String>>(sid: T, outgoing_number: T) -> MessagingService {
        MessagingService {
            sid: sid.into(),
            outgoing_number: outgoing_number.into(),
        }
    }

    pub fn send_message(&self, recipient: &str, message: &str) -> Result<(), ()> {
        Client::new().post("")
            .body(&format!(
                "To={recipient}\nFrom={sender}\nMessagingServiceSid={sid}\nBody={text}",
                recipient = recipient,
                sender = self.outgoing_number,
                sid = self.sid,
                text = message,
            )).send().map(|_| ()).map_err(|_| ())
    }
}
