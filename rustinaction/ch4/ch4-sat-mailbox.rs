
type Message = String;
#[derive(Debug)]
struct CubeSat{
    id: u64,
    mailbox: MailBox,
}

#[derive(Debug)]
struct MailBox{
    messages:Vec<Message>,
}

struct GroundStation;

impl CubeSat{
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

impl GroundStation{
    fn send(&self,to:&mut CubeSat, msg:Message){
        to.mailbox.messages.push(msg);
    }
}

fn main(){

    let mut cube:CubeSat = CubeSat{
        id:1,
        mailbox:MailBox{
            messages:Vec::new()
        }
    };

    let gs:GroundStation = GroundStation {};

    gs.send(&mut cube, Message::from("hello world!"));

    let res = cube.recv();

    println!("Received message:{:?}", res);

}