use iced::{button, Align, Button, Row, Element, Sandbox,Settings,Text,Color};
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
impl MyCounter{
    fn ToText(text: &str)-> Text{
        Text::new(text.to_string())
    }
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
        Row::new()
        .padding(40)
        .align_items(Align::Center)
        .push(
            Self::ToText("something")
        )
        .push(
            Text::new("something".to_string())
        )
        .push(
            Text::new("something".to_string())
        )
        .into()
    }

    // fn background_color(&self) -> Color {
    //     Color::TRANSPARENT
    // }
} // sandbox trait
