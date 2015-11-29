use iron::prelude::*;
use std::sync::{Arc, Mutex};
use iron::status;
use fccore::Core;

const TAG : &'static str = "countdown";

pub fn begin_countdown(core_ref : &Arc<Mutex<Core>>) -> Response {
    core_ref.lock().unwrap().begin_countdown();
    Response::with((status::Ok, "Ok".to_string()))
}

pub fn end_countdown(core_ref : &Arc<Mutex<Core>>) -> Response {
    core_ref.lock().unwrap().end_countdown();
    Response::with((status::Ok, "Ok".to_string()))
}
