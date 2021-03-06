extern crate email;
use email::email_down	;
use email::util;
use email::constants::LOG_DIR;

fn main()  {
    util::logger_init(&LOG_DIR)  ;
	match email_down::main_() {
		Ok(_)	=> {}
        Err(e) => {
            eprintln!("ERROR 201811031803 email_down::main_() {}", e);
        }
	}
}


