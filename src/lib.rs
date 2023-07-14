use std::sync::Mutex;

pub mod ops;
pub mod functions;


pub struct AppState{
    pub called: Mutex<u32>,
    pub connection : Mutex<disel_todo::PgConnection>,
}


