use actix::{Actor, Addr, Arbiter, Context, System};

struct MiActor;

impl Actor for MiActor{
	type Context = Context<Self>;
	
	fn started(&mut self, ctx: &mut Self::Context){
		println!("Estoy vivo");
		//System::current().stop();
	}
}


fn main() {
    let system = actix::System::new("test");

	let addr=MiActor.start();

    system.run();
}
