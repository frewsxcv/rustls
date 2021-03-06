use msgs::enums::{ContentType, HandshakeType, AlertDescription};
use msgs::message::{Message, MessagePayload};

extern crate webpki;

#[derive(Debug)]
pub enum HandshakeError {
  InappropriateMessage { expect_types: Vec<ContentType>, got_type: ContentType },
  InappropriateHandshakeMessage { expect_types: Vec<HandshakeType>, got_type: HandshakeType },
  NoCertificatesPresented,
  DecryptError,
  AlertReceived(AlertDescription),
  WebPKIError(webpki::Error),
  General(String)
}

#[derive(Debug)]
pub struct Expectation {
  pub content_types: Vec<ContentType>,
  pub handshake_types: Vec<HandshakeType>
}

impl Expectation {
  pub fn check_message(&self, m: &Message) -> Result<(), HandshakeError> {
    if !self.content_types.contains(&m.typ) {
      return Err(HandshakeError::InappropriateMessage {
        expect_types: self.content_types.clone(),
        got_type: m.typ.clone()
      });
    }

    if let MessagePayload::Handshake(ref hsp) = m.payload {
      if self.handshake_types.len() > 0
        && !self.handshake_types.contains(&hsp.typ) {
        return Err(HandshakeError::InappropriateHandshakeMessage {
          expect_types: self.handshake_types.clone(),
          got_type: hsp.typ.clone()
        });
      }
    }

    Ok(())
  }
}

pub type ExpectFunction = fn() -> Expectation;
