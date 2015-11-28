use iron::prelude::*;
use std::sync::{Arc, Mutex};
use iron::status;
use iron::mime::Mime;
use fccore::Core;

const TAG : &'static str = "countdown";

pub fn begin_countdown(core_ref : &Arc<Mutex<Core>>) -> Response {
    let mut core = core_ref.lock().unwrap();
    Response::with((status::Ok, "Ok".to_string()))
}

pub fn end_countdown(core_ref : &Arc<Mutex<Core>>) -> Response {
    let mut core = core_ref.lock().unwrap();
    Response::with((status::Ok, "Ok".to_string()))
}
