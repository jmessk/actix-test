use actix::Actor;

struct Ping(usize);

impl actix::Message for Ping {
    type Result = usize;
}

struct MyActor {
    count: usize,
}

impl actix::Actor for MyActor {
    type Context = actix::Context<Self>;
}

impl actix::Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Self::Context) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

#[actix::main]
async fn main() {
    let addr = MyActor { count: 0 }.start();
    let res = addr.send(Ping(10)).await.unwrap();

    println!("RESULT: {}", res);

    actix::System::current().stop();
}
