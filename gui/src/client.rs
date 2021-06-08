use iced::Color;

#[derive(Default)]
pub struct State {
    pub jid: String,
    pub passwd: String,
    // pub domain: &'a str,
    pub status: String,
}

// impl Default for State<'_>{

//     fn default() -> Self{
//         Self{
//             jid: "".to_string(),
//             passwd: "",
//             // domain: "cs510rust",
//             status: "disconnected",
//         }
//     }
// }


impl State{
    fn new() -> Self{
        Self::default()
    }
    pub fn color(&self) -> Color{
        let max = 255.0;
        match &self.status.to_lowercase()[..] {
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

    /// validates the states jid
    fn valid_jid(&self) -> bool {
        if self.jid.contains("@"){
            let v:Vec<&str> = self.jid.split("@").collect();
            if v.len() >1{
                return true;
            }
        }
        false
        // self.jid.is_empty() || self.domain.is_empty()
    }
    
    pub fn connect(&mut self){
        if !self.valid_jid(){
            self.status = "Invalid JID".to_string();
            return;
        }
        self.status = "Connected".to_string();
    }

    fn set_jid(&mut self,jid:&str){
        self.jid = jid.to_string();
    }

}


#[test]
fn test_valid_jid(){
    let mut state = State::new();
    
    state.set_jid("test");
    assert_eq!(state.valid_jid(),false);

    state.set_jid("test@");
    assert_eq!(state.valid_jid(),true);

    state.set_jid("@test");
    assert_eq!(state.valid_jid(),true);

    state.set_jid("test@test");
    assert_eq!(state.valid_jid(),true);
}