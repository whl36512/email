extern crate email;
use email::email_proc   ;


fn main()  {
	match email_proc::main_() {
		Ok(_)	=> {}
        Err(e) => {
            eprintln!("ERROR 201811072121 email_proc::main_() {}", e);
        }
	}
}


