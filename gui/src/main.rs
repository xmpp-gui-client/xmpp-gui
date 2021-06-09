
//! # Mohammed Alsaid
//! This is a simple GUI client the uses `xmpp` crate to build an XMPP client. 

use iced::{
    button, text_input, Button, Checkbox, Column, Element, Row, Sandbox, Settings, Text, TextInput,
};

/// Clinet module that handles the state of the XMPP client. It performs XMPP functionalities on behalf of the user using `xmpp` create.
mod client;
use client::State;

/// Runs the GUI application with the default settings of iced.
fn main() -> iced::Result {
    App::run(Settings::default())
}

/// Holds the GUI state of the client.
#[derive(Default)]
pub struct App {
    /// something
    connect_button: button::State, 
    disconnect_button: button::State, // used to indicate disconnection from server
    join_room_button: button::State,

    jid_input: text_input::State,
    passwd_input: text_input::State,
    room_name_input: text_input::State,

    avatars_feature: bool,
    join_room_feature: bool,
    contact_list_feature: bool,
    state: State,
}

/// used to describe the interactions/events that arise between the components.
#[derive(Debug, Clone)]
pub enum GuiEvent {
    Connect,
    Disconnect,
    JoinRoom,
    JidChanged(String),
    PasswdChanged(String),
    ToggleAvatar(bool),
    ToggleJoinRoom(bool),
    ToggleContactList(bool),
    RoomNameChanged(String),
}

impl Sandbox for App {
    type Message = GuiEvent;
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "XMPP Client".to_string()
    }
    
    fn view(&mut self) -> Element<GuiEvent> {
        Column::new() 
            .push(
                Row::new()
                    .push(
                        TextInput::new(
                            &mut self.jid_input,
                            "JID",
                            &self.state.jid,
                            GuiEvent::JidChanged,
                        )
                        .padding(10),
                    )
                    .push(
                        TextInput::new(
                            &mut self.passwd_input,
                            "Password",
                            &self.state.passwd,
                            GuiEvent::PasswdChanged,
                        )
                        .padding(10),
                    )
                    .max_width(500),
            )
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.connect_button, Text::new("Connect"))
                            .on_press(GuiEvent::Connect), 
                    )
                    .push(
                        Button::new(&mut self.disconnect_button, Text::new("Disconned"))
                            .on_press(GuiEvent::Disconnect),
                    )
                    .push(
                        Text::new(&self.state.status)
                            .size(30)
                            .color(self.state.color()),
                    )
                    .spacing(10),
            )
            .push(Text::new("Features:").size(30))
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .push(
                                Checkbox::new(
                                    self.join_room_feature,
                                    "JoinRoom".to_owned(),
                                    GuiEvent::ToggleJoinRoom,
                                )
                                .size(30),
                            )
                            .push(
                                Checkbox::new(
                                    self.avatars_feature,
                                    "Avatars".to_owned(),
                                    GuiEvent::ToggleAvatar,
                                )
                                .size(30),
                            )
                            .push(
                                Checkbox::new(
                                    self.contact_list_feature,
                                    "ContactList".to_owned(),
                                    GuiEvent::ToggleContactList,
                                )
                                .size(30),
                            ),
                    )
                    .push(
                        Column::new()
                            .push(
                                TextInput::new(
                                    &mut self.room_name_input,
                                    "Room",
                                    &self.state.room,
                                    GuiEvent::RoomNameChanged,
                                )
                                .padding(10),
                            )
                            .max_width(300)
                            .push(
                                Row::new()
                                    .push(
                                        Button::new(
                                            &mut self.join_room_button,
                                            Text::new("Join room"),
                                        )
                                        .on_press(GuiEvent::JoinRoom),
                                    )
                                    .push(Text::new(&self.state.room_status).size(30))
                                    .spacing(10),
                            ),
                    )
                    .spacing(50),
            )
            .padding(100)
            .spacing(20)
            .into()
    }

    fn update(&mut self, message: GuiEvent) {
        match message {
            GuiEvent::Connect => {
                self.state.set_status("");
                self.state.connect();
            }
            GuiEvent::Disconnect => {
                self.state.status = "Disconnected".to_string();
            }
            GuiEvent::JidChanged(s) => {
                self.state.set_status("");
                self.state.set_jid(&s);
            }
            GuiEvent::PasswdChanged(s) => {
                self.state.set_status("");
                self.state.set_passwd(&s);
            }
            GuiEvent::ToggleContactList(_) => {
                self.contact_list_feature = self.state.feature_toggle("ContactList");
            }
            GuiEvent::ToggleAvatar(_) => {
                self.avatars_feature = self.state.feature_toggle("avatars");
            }
            GuiEvent::ToggleJoinRoom(_) => {
                self.join_room_feature = self.state.feature_toggle("joinRooms");
            }
            GuiEvent::RoomNameChanged(name) => self.state.set_room(&name),
            GuiEvent::JoinRoom => self.state.join_room(),
        }
    }
}
