use iced::Color;
use xmpp::{ClientBuilder, ClientFeature, ClientType, Event};
// use ;

#[derive(Default)]
pub struct State {
    pub jid: String,
    pub passwd: String,
    pub status: String,
    pub room: String,
    pub room_status: String,
    domain: String,
    features: Vec<String>,
}

impl State {
    // fn new() -> Self{
    //     Self::default()
    // }
    pub fn color(&self) -> Color {
        let max = 255.0;
        match &self.status.to_lowercase()[..] {
            "connected" => Color {
                r: 0.,
                g: max,
                b: 0.,
                a: max,
            },
            "not connected" => Color {
                r: max,
                g: 0.,
                b: 0.,
                a: max,
            },
            _ => Color::BLACK,
        }
    }

    /// validates the states jid
    fn valid_jid(&self) -> bool {
        if self.jid.contains('@') {
            let v: Vec<&str> = self.jid.split('@').collect();
            if v.len() > 1 && !v[0].is_empty() && !v[1].is_empty() {
                return true;
            }
        }
        false
    }

    #[tokio::main]
    async fn conn(&mut self) {
        let mut client_builder =
            ClientBuilder::new(&self.jid, &self.passwd).set_client(ClientType::Pc, "xmpp-rs");
        for f in self.features.iter() {
            client_builder = client_builder.enable_feature(self.get_feature(f));
        }
        let mut client = client_builder.build().unwrap();
        if let Some(events) = client.wait_for_events().await {
            match events[0] {
                Event::Online => {
                    self.set_status("Connected");
                }
                Event::Disconnected => {
                    self.set_status("Not connected");
                }
                _ => {
                    self.set_status("Failed");
                }
            }
        }
    }
    pub fn connect(&mut self) {
        if !self.valid_jid() {
            self.status = "Invalid JID".to_string();
            return;
        }
        self.conn();
    }

    // #[tokio::main]
    fn join(&mut self) {
        let mut client_builder =
            ClientBuilder::new(&self.jid, &self.passwd).set_client(ClientType::Pc, "xmpp-rs");
        for f in self.features.iter() {
            client_builder = client_builder.enable_feature(self.get_feature(f));
        }
        self.room_status = "failed to connect".to_string();
    }
    pub fn join_room(&mut self) {
        if !self.valid_jid() || self.status.to_lowercase() != "connected" {
            self.room_status = "Not joined".to_string();
            return;
        }
        let v: Vec<&str> = self.jid.split('@').collect();
        if v.len() > 1 && !v[0].is_empty() && !v[1].is_empty() {
            self.domain = v[1].to_string();
        }
        self.join();
    }
    pub fn set_jid(&mut self, jid: &str) {
        self.jid = jid.to_string();
    }

    pub fn set_passwd(&mut self, pass: &str) {
        self.passwd = pass.to_string();
    }
    pub fn set_status(&mut self, status: &str) {
        self.status = status.to_string();
    }

    fn add_feature(&mut self, feature: &str) {
        let exists = self.feature_exist(feature);
        if !exists {
            self.features.push(feature.to_lowercase());
        }
    }
    fn remove_feature(&mut self, feature: &str) {
        let f = &feature.to_lowercase()[..];
        self.features.retain(|x| x != f);
    }
    pub fn feature_exist(&self, feature: &str) -> bool {
        let feature = &feature.to_lowercase()[..];
        self.features.iter().any(|f| f == feature)
    }

    pub fn set_room(&mut self, name: &str) {
        self.room = name.to_string();
    }

    fn get_feature(&self, f: &str) -> ClientFeature {
        match &f.to_lowercase()[..] {
            "joinrooms" => ClientFeature::JoinRooms,
            "avatars" => ClientFeature::Avatars,
            _ => ClientFeature::ContactList,
        }
    }

    pub fn feature_toggle(&mut self, feature: &str) -> bool {
        if self.feature_exist(feature) {
            self.remove_feature(feature);
            return false;
        }
        self.add_feature(feature);
        true
    }
}

#[test]
fn test_valid_jid() {
    let mut state = State::default();

    state.set_jid("test");
    assert_eq!(state.valid_jid(), false);

    state.set_jid("test@");
    assert_eq!(state.valid_jid(), false);

    state.set_jid("@test");
    assert_eq!(state.valid_jid(), false);

    state.set_jid("test@test");
    assert_eq!(state.valid_jid(), true);
}
#[test]
fn test_connect_successful() {
    // this test uses a known valid account on an xmpp server
    let mut state = State::default();
    state.set_jid("moe@jabber.de");
    state.set_passwd("1111");
    state.connect();
    assert_eq!(state.status.to_lowercase(), "connected");
}

#[test]
fn test_connect_failure_invalid_jid() {
    let mut state = State::default();
    state.set_jid("test@");
    state.connect();
    assert_eq!(state.status.to_lowercase(), "invalid jid");
}

#[test]
fn test_connect_failure_disconnect() {
    let mut state = State::default();
    state.set_jid("non_existant@localhost"); // I own the cs510rust.com & have no server set up on it
    state.connect();
    assert_eq!(state.status.to_lowercase(), "not connected");
}

#[test]
fn test_add_feature() {
    let mut s = State::default();
    s.add_feature("JoinRooms");
    assert!(s.feature_exist("joinrooms"));

    s.add_feature("avatars");
    s.add_feature("ContactList");
    s.remove_feature("joinrooms");

    assert!(s.feature_exist("Avatars"));
    assert!(s.feature_exist("ContactList"));
    assert!(!s.feature_exist("JoinRooms"));
}

#[test]
fn test_remove_feature() {
    let mut s = State::default();
    s.add_feature("Avatars");
    s.add_feature("ContactList");
    s.add_feature("JoinRooms");

    assert!(s.feature_exist("Avatars"));
    s.remove_feature("Avatars");
    assert!(!s.feature_exist("Avatars"));

    assert!(s.feature_exist("JoinRooms"));
    assert!(s.feature_exist("ContactList"));

    assert!(s.feature_exist("ContactList"));
    s.remove_feature("ContactList");
    assert!(!s.feature_exist("ContactList"));

    assert!(s.feature_exist("JoinRooms"));
    s.remove_feature("JoinRooms");
    assert!(!s.feature_exist("JoinRooms"));

    s.remove_feature("Avatars");
    assert!(!s.feature_exist("Avatars"));
}
