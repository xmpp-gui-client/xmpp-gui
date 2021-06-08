mod connection_handler;
use iced::{Result, Sandbox, Settings};
use connection_handler::Handler as ConnHandler;

fn main() -> Result {
    ConnHandler::run(Settings::default())
}
