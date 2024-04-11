fn main() {}

trait Text {
    fn value(&self) -> String;
    fn clone_box(&self) -> Box<dyn Text>;
}

impl Clone for Box<dyn Text> {
    fn clone(&self) -> Box<dyn Text> {
        self.clone_box()
    }
}

impl Text for Box<dyn Text> {
    fn value(&self) -> String {
        self.as_ref().value()
    }

    fn clone_box(&self) -> Box<dyn Text> {
        self.as_ref().clone_box()
    }
}

#[derive(Clone)]
pub struct PlainText {
    text: String,
}

impl From<&str> for PlainText {
    fn from(s: &str) -> PlainText {
        PlainText { text: s.into() }
    }
}

impl Text for PlainText {
    fn value(&self) -> String {
        self.text.clone()
    }

    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl AsRef<dyn Text> for PlainText {
    fn as_ref(&self) -> &(dyn Text + 'static) {
        self
    }
}

#[derive(Clone)]
pub struct RepeatedText {
    text: Box<dyn Text>,
    n: usize,
}

impl RepeatedText {
    fn with_parts(text: &dyn Text, n: usize) -> RepeatedText {
        RepeatedText {
            text: text.clone_box(),
            n,
        }
    }
}

impl Text for RepeatedText {
    fn value(&self) -> String {
        self.text.value().repeat(self.n)
    }

    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

#[test]
fn test_text_repeated() {
    let t1 = PlainText::from("Hi");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(&t2, 3);
    let t4 = RepeatedText::with_parts(&t3, 5);
    assert_eq!(t1.value(), "Hi");
    assert_eq!(t2.value(), "[+]");
    assert_eq!(t3.value(), "[+]".repeat(3));
    assert_eq!(t4.value(), "[+]".repeat(15));
}

