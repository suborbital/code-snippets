use suborbital::runnable::*;
use suborbital::file;
use suborbital::req;
use suborbital::resp;
use suborbital::log;

struct Fileserver{}

impl Runnable for Fileserver {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let mut path = req::url();
        path.remove(0);
        
        log::warn(path.clone().as_str());

        let file = match file::get_static(path.as_str()) {
            Some(val) => val,
            None => return Err(RunErr::new(404, "not found"))
        };

        if path.ends_with(".css") {
            resp::content_type("text/css")
        }else if path.ends_with(".js") {
            resp::content_type("application/javascript")
        }
    
        Ok(file)
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &Fileserver = &Fileserver{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
