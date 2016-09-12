extern crate hyper;

use hyper::Client;
use hyper::header::{Authorization, Basic, ContentType};

use std::io::Read;

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

    pub fn send_message(&self, recipient: &str, message: &str) -> Result<String, String> {
        let res = Client::new()
            .post(&format!(
            "https://api.twilio.com/2010-04-01/Accounts/{sid}/Messages",
            sid = self.sid,
        ))
            .header(Authorization(Basic {
                username: self.sid.to_owned(),
                password: Some(self.token.to_owned()),
            }))
            .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
            .body(&format!(
            "To={recipient}&From={sender}&Body={text}&",
            recipient = recipient,
            sender = self.outgoing_number,
            text = message,
        ))
            .send();

        match res {
            Ok(mut res) => {
                let mut buf = String::new();
                res.read_to_string(&mut buf).ok();
                Ok(buf)
            }
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
