mod connection_handler;
use connection_handler::Handler as ConnHandler;
use iced::{Result, Sandbox, Settings};

fn main() -> Result {
    ConnHandler::run(Settings::default())
}
