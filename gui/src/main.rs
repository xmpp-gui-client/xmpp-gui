// things components used from the iced crate
use iced::{button, Sandbox,Button, Color, Element, Row,Column, Text,TextInput,text_input,Settings};
use xmpp::{ClientBuilder, ClientFeature, ClientType, Event};
// use xmpp_parsers::{message::MessageType, Jid};
mod client;
use client::State;


fn main() -> iced::Result {
    App::run(Settings::default())
}


/// first component required by Iced (state)
#[derive(Default)]
pub struct App {
    // connection_status: &'a str,
    connect_button: button::State, // used to indicate connection to server
    disconnect_button: button::State, // used to indicate disconnection from server

    jid_input: text_input::State,
    passwd_input: text_input::State,
    // passwd: String,
    // jid: String,
    state: State,
}


/// second component required by Iced (interactions)
#[derive(Debug, Clone)]
pub enum GuiEvent {
    Connect,
    Disconnect,
    JidChanged(String),
    PasswdChanged(String),
}

// impl App<'_> {
//     // fn status_color(status: &str) -> Color {
//     //     let max = 255.0;
//     //     match status {
//     //         "connected" => Color {
//     //             r: 0.,
//     //             g: max,
//     //             b: 0.,
//     //             a: max,
//     //         },
//     //         _ => Color {
//     //             r: max,
//     //             g: 0.,
//     //             b: 0.,
//     //             a: max,
//     //         },
//     //     }
//     // }
//     fn valid_jid(jid:&str) -> bool {
//         if jid.contains("@"){
//             let v:Vec<&str> = jid.split("@").collect();
//             if v.len() > 1{
//                 return true;
//             }
//         }
//         false
//     }
// }

impl Sandbox for App {
    // type Executor = executor::Default;
    // type Flags = ();
    type Message = GuiEvent;
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "XMPP Client".to_string()
    }
    /// third component needed (view)
    fn view(&mut self) -> Element<GuiEvent> {
        Column::new() // first row
            .push(
                Row::new()
                .push(
                    TextInput::new(&mut self.jid_input,"JID",&self.state.jid,GuiEvent::JidChanged).padding(10),
                )
                .push(
                    TextInput::new(&mut self.passwd_input,"Password",&self.state.passwd,GuiEvent::PasswdChanged).padding(10),
                )
                .max_width(500)
            )
            .push(
                Row::new()
                .push(
                    Button::new(&mut self.connect_button, Text::new("Connect"))
                    .on_press(GuiEvent::Connect), // send connect event when clicked
                )
                .push(
                    Button::new(&mut self.disconnect_button, Text::new("Disconned"))
                    .on_press(GuiEvent::Disconnect), // send disconnect event
                )
                .push(
                    // display connection status
                    Text::new(&self.state.status)
                    .size(30)
                    .color(self.state.color())
                )
                .spacing(10)
            )
            .into()
    }

    /// fourth component needed (reaction to interactions -- udpate state)
    fn update(&mut self, message: GuiEvent){
        match message {
            GuiEvent::Connect => {
                // let jid = &self.state.jid;
                // let passwd = &self.state.passwd;
                // if self.state.valid_jid() {
                //     self.state.status = "Invalid JID".to_string();
                // }
                self.state.connect();
            },
            GuiEvent::Disconnect => {
                self.state.status = "Disconnected".to_string();
            },
            GuiEvent::JidChanged(s) => {
                self.state.jid = s.to_string();
            },
            GuiEvent::PasswdChanged(s) => {
                self.state.passwd = s.to_string();
            },
        }
    }
}
