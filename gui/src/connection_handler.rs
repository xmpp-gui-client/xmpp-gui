// things components used from the iced crate
use iced::{button, Button, Color, Element, Row, Sandbox, Text};

/// first component required by Iced (state)
#[derive(Default)]
pub struct Handler<'a> {
    connection_status: &'a str,
    connect_button: button::State, // used to indicate connection to server
    disconnect_button: button::State, // used to indicate disconnection from server
}

/// second component required by Iced (interactions)
#[derive(Debug, Clone, Copy)]
pub enum Msg {
    // TextInputChanged(String),
    Connect,
    Disconnect,
}

impl Handler<'_> {
    fn change_color(&self) -> Color {
        let max = 255.0;
        match self.connection_status {
            "connected" => Color {
                r: 0.,
                g: max,
                b: 0.,
                a: max,
            },
            _ => Color {
                r: max,
                g: 0.,
                b: 0.,
                a: max,
            },
        }
    }
}

impl Sandbox for Handler<'_> {
    type Message = Msg;
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "xmpp Handler client".to_string()
    }
    /// third component needed (view)
    fn view(&mut self) -> Element<Msg> {
        Row::new()
            .push(
                // display connection status
                Text::new(&self.connection_status.to_string())
                    .size(30)
                    .color(self.change_color()),
            )
            .push(
                Button::new(&mut self.connect_button, Text::new("connect")).on_press(Msg::Connect), // send connect message when clicked
            )
            .push(
                Button::new(&mut self.disconnect_button, Text::new("disconned"))
                    .on_press(Msg::Disconnect),
            )
            .into()
    }

    /// fourth component needed (reaction to interactions -- udpate state)
    fn update(&mut self, message: Msg) {
        match message {
            Msg::Connect => {
                self.connection_status = "connected";
            }
            Msg::Disconnect => {
                self.connection_status = "disconnected";
            }
        }
    }
}
