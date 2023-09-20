
struct Stream<T> {
    name: String,
    messages: Vec<T>
}

fn init<T>(name: String) -> Stream<T> {
    Stream {
        name,
        messages: Vec::new(),
    }
}

impl<T> Stream<T> {

    fn push_message(&mut self, message: T){
        self.messages.push(message)
    }

    fn get_last(&self) -> &T {
        &self.messages[self.messages.len()-1]
    }
}

#[test]
fn test_stream(){
    let mut stream = init("some_stream_name".to_string());
    let test_messages= ["message1".to_string(), "message2".to_string(), "message3".to_string()];
    for m in test_messages {
        stream.push_message(m)
    }

    let last_message = stream.get_last();
    println!("{:?} => {:?}",stream.name, last_message)
}