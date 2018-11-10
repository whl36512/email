extern crate email;
use email::email_proc   ;

use email::util;
use email::constants::LOG_DIR;

fn main()  {
    util::logger_init(&LOG_DIR)  ;
	match email_proc::main_() {
		Ok(_)	=> {}
        Err(e) => {
            eprintln!("ERROR 201811072121 email_proc::main_() {}", e);
        }
	}
}


