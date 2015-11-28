use rustc_serialize::json;
use std::string::{String, ToString};
use fccore::Core;
use std::sync::MutexGuard;
use iron::prelude::*;
use std::sync::{Arc, Mutex};
use iron::status;
use iron::mime::Mime;

#[derive(RustcEncodable)]
struct Status {
    pub alive: bool,
    pub armed_switch: bool,
    pub armed_cmd: bool,
    pub armed: bool,
    pub countdown: String,
    pub is_counting_down: bool
}

impl Status {

    pub fn from(core: &MutexGuard<Core>) -> Status {
        Status{
            alive: core.alive,
            armed_switch: core.armed_switch(),
            armed_cmd: core.armed_cmd(),
            armed: core.armed(),
            is_counting_down: core.is_counting_down,
            countdown: if core.is_counting_down { core.countdown_time.to_string() } else { "Unplanned".to_string() }
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        json::encode(self).unwrap()
    }
}

pub fn status_report(core_ref :&Arc<Mutex<Core>>) -> Response {
    let json_content_type : Mime = "application/json".parse::<Mime>().unwrap();
    Response::with((json_content_type, status::Ok, Status::from(&core_ref.lock().unwrap()).to_string()))
}