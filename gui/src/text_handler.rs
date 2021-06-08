use iced::{button,text_input, Button, Element, Row, Sandbox, TextInput,Text};

// #[derive(Default)]
pub struct Handler{
    input: text_input::State,
    send_button: button::State,
}

#[derive(Debug,Clone)]
pub enum Event{
    SendMessage,
    TextInputChanged(String),
}

impl Handler{
    fn new() -> Self {
        Handler{input:text_input::State::new(),send_button:button::State::new()}
    }
    fn view(&mut self) -> Element<Event> {
        let val = "Text";
        Row::new()
        .push(
            TextInput::new(&mut self.input,"send message..",val,Event::TextInputChanged).padding(10),
        )
        .push(
            Button::new(&mut self.send_button,Text::new("Send")),
        ).into()
    }

    /// handles events that occur in chatbox
    fn update(&mut self, e: Event) {
        match e {
            Event::SendMessage => println!("sending message..."), 
            Event::TextInputChanged(s) => println!("text changed to: {}",s),
        }
    }

}
