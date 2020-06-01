use actix::prelude::*;
use std::time::Duration;

#[derive(Message)]
#[rtype(result="()")]
struct Ping{
	pub id:usize,
}

//Definicion del Actor
struct Juego{
	contador: usize,
	direccion: Recipient<Ping>,
}

impl Actor for Juego{
	type Context= Context <Juego>;
}

// Controlador para el Ping mensaje
impl Handler<Ping> for Juego{
	type Result=();
	
	fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>){
		self.contador+=1;
		
		if self.contador>10{
			System::current().stop();
		}else{
			println!("Ping recibido {:?}", msg.id);
			
			// esperamos 100 nano segundos
			ctx.run_later(
				Duration::new(0,100), move | actor, _ | {
					actor.direccion.do_send(Ping{id: msg.id+1});
				}
			);
		}
	}
}

fn main(){
	let system= System::new("test");
	
	let direccion=Juego::create(|ctx|{
		let direccion=ctx.address();
	let direccion2= Juego{
		contador:0,
		direccion: direccion.recipient(),
	}.start();
	
	direccion2.do_send(Ping {id:10});
	
	Juego{
		contador:0,
		direccion: direccion2.recipient(),
	}
	});
	
	system.run();
}