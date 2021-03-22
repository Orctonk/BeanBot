use std::vec;

use serenity::{
    model::channel::Message,
    prelude::*
};

//Kanske borde ha en lambda för get commands? För moduler som vill t.e.x. bevaka för ord mitt i meningar och så.
//Alternativt kanske vi kan kolla på att använda flera EventHandlers bara? En handler per modul känns som bra uppdelning ändå
//Då kan varje modul behandla meddelanden som den vill.
pub trait Module {
    fn get_commands(&self) -> &'static String;
    fn dispatch(&self,ctx : Context, msg: Message); 
}