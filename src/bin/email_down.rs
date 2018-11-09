extern crate email;
use email::email_down	;

fn main()  {
	match email_down::main_() {
		Ok(_)	=> {}
        Err(e) => {
            eprintln!("ERROR 201811031803 email_down::main_() {}", e);
        }
	}
}


