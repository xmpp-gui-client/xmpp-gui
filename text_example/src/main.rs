use iced::{TextInput, text_input, Align, Column, Element, Sandbox,Settings,Text,Color};

fn main() -> iced::Result{
    MyBox::run(Settings::default())
}

#[derive(Default)]
struct MyBox<'a >{
    input: &'a str,
    output: String,
}
#[derive(Debug,Clone)]
enum Mess{
    TextChanged(String),
}

impl Sandbox for MyBox<'_>{
    type Message = Mess; // associate Message type w/ Mess
    fn new() -> Self{
        Self::default()
    }
    fn title(&self) -> String{
        "My TITLE!!".to_string()
    }
    fn update(&mut self,msg: Mess) {
        match msg{
            Mess::TextChanged(input) => {
                // let i = &self.input[..];
                self.output = String::from(input);
            },
        }
    }
    fn view(& mut self) -> Element<Message> {
        let mut state = text_input::State::new();
        let input = TextInput::new(
            &mut state,
            "This is a placeholder...",
            "Text",
            Mess::TextChanged,
        ).padding(10).into()
        // Column::new()
        // .padding(20)
        // .align_items(Align::Center)
        // .push(Text::new(self.output))
        // .push(input)
        // .into()
    }
}
