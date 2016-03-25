extern crate rustful;

use std::io::Write;
use std::thread::spawn;

use rustful::Server;
use rustful::handler::{RawHandler, Factory, Next, Encoder, Decoder, Meta};
use rustful::context::RawContext;
use rustful::response::{RawResponse, ResponseHead};

const ANSWER: &'static [u8] = b"Hello World!";

#[derive(Default)]
struct HandlerFactory;

impl Factory for HandlerFactory {
	type Handler = Handler;

	fn create(&self, _context: RawContext, response: RawResponse) -> Handler {
		Handler(Some(response.into()))
	}
}

impl Meta for HandlerFactory {}

struct Handler(Option<ResponseHead>);

impl RawHandler for Handler {
	fn on_request(&mut self) -> Next {
		Next::write()
	}

	fn on_request_readable(&mut self, _: &mut Decoder) -> Next {
		Next::write()
	}

	fn on_response(&mut self) -> (ResponseHead, Next) {
		use rustful::header::ContentLength;
		let mut head = self.0.take().expect("missing head");
		head.headers.set(ContentLength(ANSWER.len() as u64));

		(head, Next::write())
	}

	fn on_response_writable(&mut self, encoder: &mut Encoder) -> Next {
		let n = encoder.write(ANSWER).unwrap();
		debug_assert_eq!(n, ANSWER.len());
		Next::end()
	}
}

fn main() {
	Server {
		handlers: HandlerFactory,
		host: 8080.into(),
		..Server::default()
	}.run();
}