// things components used from the iced crate
use iced::{button, Button, Color, Element, Row,Column, Sandbox, Text,TextInput,text_input};
use xmpp::{ClientBuilder,ClientType,Agent};//, ClientFeature, Event};

/// first component required by Iced (state)
#[derive(Default)]
pub struct Handler<'a> {
    connection_status: &'a str,
    connect_button: button::State, // used to indicate connection to server
    disconnect_button: button::State, // used to indicate disconnection from server

    jid_input: text_input::State,
    passwd_input: text_input::State,
    passwd: String,
    jid: String,

    // client: Agent,
}


/// second component required by Iced (interactions)
#[derive(Debug, Clone)]
pub enum Event {
    Connect,
    Disconnect,
    JidChanged(String),
    PasswdChanged(String),
}

impl Handler<'_> {
    fn status_color(status: &str) -> Color {
        let max = 255.0;
        match status {
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
    type Message = Event;
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "XMPP Client".to_string()
    }
    /// third component needed (view)
    fn view(&mut self) -> Element<Event> {
        Column::new() // first row
            .push(
                Row::new()
                .push(
                    TextInput::new(&mut self.jid_input,"JID",&self.jid,Event::JidChanged).padding(10),
                )
                .push(
                    TextInput::new(&mut self.passwd_input,"Password",&self.passwd,Event::PasswdChanged).padding(10),
                )
                .max_width(500)
            )
            .push(
                Row::new()
                .push(
                    Button::new(&mut self.connect_button, Text::new("Connect"))
                    .on_press(Event::Connect), // send connect event when clicked
                )
                .push(
                    Button::new(&mut self.disconnect_button, Text::new("Disconned"))
                    .on_press(Event::Disconnect), // send disconnect event
                )
                .push(
                    // display connection status
                    Text::new(&self.connection_status.to_string())
                    .size(30)
                    .color(Self::status_color(self.connection_status))
                )
                .spacing(10)
            )
            .into()
    }

    /// fourth component needed (reaction to interactions -- udpate state)
    fn update(&mut self, message: Event) {
        match message {
            Event::Connect => {
                self.connection_status = "connected";
                let jid = &self.jid;
                let passwd = &self.passwd;
                // self.client = ClientBuilder::new(jid,passwd).set_client(ClientType::Pc,"cs510rust").build().unwrap();
            }
            Event::Disconnect => {
                self.connection_status = "disconnected";
            }
            Event::JidChanged(s) => {
                self.jid = s;
            },
            Event::PasswdChanged(s) => {
                self.passwd = s;
            }
        }
    }
}
