use suborbital::runnable::*;
use suborbital::resp;
use suborbital::file;

struct ContentType{}

impl Runnable for ContentType {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        resp::content_type("text/html");

        let index = file::get_static("index.html");

        Ok(index.unwrap())
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &ContentType = &ContentType{};

#[no_mangle]
pub extern fn init() {
    use_runnable(RUNNABLE);
}
