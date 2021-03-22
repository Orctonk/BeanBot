use std::vec;

use serenity::{
    model::channel::Message,
    prelude::*
};

pub trait Module {
    fn get_commands(&self) -> &'static String;
    fn dispatch(&self,ctx : Context, msg: Message); 
}