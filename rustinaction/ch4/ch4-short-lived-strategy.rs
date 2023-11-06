
#[derive(Debug)]
struct CubeSat{
    id:u64,
}

#[derive(Debug)]
struct MailBox{
    messages:Vec<Message>
}

#[derive(Debug)]
struct Message{
    to:u64,
    content:String,
}

struct GroundStation{}

impl GroundStation{
    fn connect(&self, sat_id:u64)-> CubeSat{
        CubeSat{
            id:sat_id
        }
    }

    fn send(&self, mailbox:&mut MailBox, msg:Message){
        mailbox.post(msg);
    }


}


impl MailBox{
    fn post(&mut self, msg:Message){
        self.messages.push(msg)
    }

    fn deliver(&mut self, cs:&CubeSat) -> Option<Message>{
        for i in 0..self.messages.len(){
            if self.messages[i].to == cs.id{
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

impl CubeSat{
    fn receive(&self, mailbox:&mut MailBox) -> Option<Message>{
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64>{
    vec![1,2,3]
}

fn main(){
    let mut mailbox = MailBox{messages:vec![]};
    let gs = GroundStation{};

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids{
        let _sat = gs.connect(sat_id);
        let mut content_str = String::from("hello from ");
        content_str.push_str(&sat_id.to_string());
        let msg = Message{to:sat_id, content: content_str};
        gs.send(&mut mailbox, msg);
    }
    gs.send(&mut mailbox, Message{to:1 as u64, content:String::from("xxxx")});

    println!("Mailbox:{:?}", mailbox);

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids{
        let sat = gs.connect(sat_id);

        let msg = sat.receive(&mut mailbox);
        println!("{:?},{:?}",sat, msg );
    }
}
