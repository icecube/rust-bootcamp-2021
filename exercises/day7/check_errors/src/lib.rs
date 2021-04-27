
pub trait PartialMessage {
    fn get(&self) -> &str;
}

pub struct StaticMessage {
    msg: &'static str
}
impl StaticMessage {
    pub fn new(s: &str) -> StaticMessage {
        StaticMessage{msg: s}
    }
}
impl PartialMessage for StaticMessage {
    fn get(&self) -> &str {
        return self.msg
    }
}

pub struct RefMessage {
    msg: &str
}
impl RefMessage {
    pub fn new(s: &str) -> RefMessage {
        RefMessage{msg: s}
    }
}
impl PartialMessage for RefMessage {
    fn get(&self) -> &str {
        return self.msg
    }
}


pub fn choice(left: &impl PartialMessage, right: &impl PartialMessage) -> &dyn PartialMessage {
    if &left.get()[0..1] == "t" {
        left
    } else {
        right
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_static() {
        let s = StaticMessage::new("testing");
        let val = s.get();
        assert_eq!(val, "testing");
    }
    
    #[test]
    fn choice_static() {
        let s = StaticMessage::new("testing");
        let s2 = StaticMessage::new("v2");
        let val = choice(&s, &s2);
        assert_eq!(val.get(), "testing");
    }

    #[test]
    fn get_ref() {
        let v1 = String::from("testing");
        let s = RefMessage::new(v1.as_str());
        let val = s.get();
        assert_eq!(val, "testing");
    }
    
    #[test]
    fn choice_ref() {
        let v1 = String::from("testing");
        let v2 = String::from("v2");
        let s = RefMessage::new(v1.as_str());
        let s2 = RefMessage::new(v2.as_str());
        let val = choice(&s, &s2);
        assert_eq!(val.get(), "testing");
    }
    
    #[test]
    fn choice_both() {
        let v1 = String::from("testing");
        let s = RefMessage::new(v1.as_str());
        let s2 = StaticMessage::new("v2");
        let val = choice(&s, &s2);
        assert_eq!(val.get(), "testing");
    }
}
