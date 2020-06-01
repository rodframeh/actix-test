use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "usize")] // se define el tipo de respuesta de suma
struct Suma (usize, usize);

// Definicion del actor
struct Sumador;

impl Actor for Sumador{
	type Context = Context<Self>;
}

// Definimos el Controlador de Mensajes para el mensaje Suma
impl Handler<Suma> for Sumador{
	type Result =usize; //tipo de respuesta del mensaje
	
	fn handle(&mut self, msg: Suma, ctx: &mut Context<Self>)-> Self::Result{
		msg.0+msg.1
	}
}

#[actix_rt::main]// inicia el sistema y se bloquea hasta que se resuelva el future
async fn main() {
	let addr = Sumador.start();
	let res= addr.send(Suma(35,35)).await; // se envia el mensaje y se obtiene el resultado futuro
	
	match res {
		Ok(resultado)=> println!("SUMA: {}",resultado),
		_ => println!("La comunicacion con el actor fallo"),
	}
}
