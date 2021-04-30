use iced::{button, Align, Button, Column, Element, Sandbox,Settings,Text};
fn main() -> iced::Result{
    MyCounter::run(Settings::default())
    // println!("Hello, world!");
}

#[derive(Default)]
struct MyCounter{
    value:i32,
    increment_button: button::State,
    decrement_button: button::State,
}


// Enum that has Debug, Clone and Copy traits
#[derive(Debug,Clone,Copy)]
enum Message{
    Increment,
    Decrement,
}

// implement the sanbox trait for MyCounter
impl Sandbox for MyCounter{
    type Message = Message; // type
    fn new() -> Self{
        Self::default() // default value for Self type (derived in line 7)
    }
    fn title(&self) -> String {
        String::from("My Counter :)")
    }

    // updates value based on message
    // used by elements
    fn update(&mut self,message: Message){
        match message{
            Message::Increment => self.value+=1,
            Message::Decrement => self.value -=1,
        };
    }
    
    // return element of type Message
    fn view(&mut self) -> Element<Message>{
        Column::new()
        .padding(40)
        .align_items(Align::Center)
        .push(
            Button::new(&mut self.increment_button,Text::new("Increment"))
            .on_press(Message::Increment),
        )
        .push(Text::new(self.value.to_string()).size(50))
        .push(
            Button::new(&mut self.decrement_button,Text::new("Decrement"))
            .on_press(Message::Decrement),
        )
        .into()
    }
} // sandbox trait
