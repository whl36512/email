
extern crate imap;
extern crate native_tls;

use self::native_tls::TlsConnector;
use self::imap::client::Session	;

use std::str;
use std::io::{self, BufRead};
use std::fs;
use std::fs::File;
use std::io::{Write};
//use std::io::{BufReader};
use std::io::{Read};
use std::error::Error;

//use constants::LOG_DIR	;
use constants::DIR_EMAIL_DOWNLOAD	;
use constants::DIR_EMAIL_READY	;
use constants::EMAIL_MOVE	;
use constants::EMAIL_SEARCH	;
use constants::DOMAIN	;
use constants::PORT	;
use constants::USER_NAME	;

//use util;


// To connect to the gmail IMAP server with this you will need to allow unsecure apps access.
// See: https://support.google.com/accounts/answer/6010255?hl=en
// Look at the gmail_oauth2.rs example on how to connect to a gmail server securely.

pub fn main_() -> Result<i32, Box<Error>> {
//	util::logger_init(&LOG_DIR)	 ;
	let (user_name, pw) = read_credential();

	let socket_addr 	= (DOMAIN, PORT);
	let ssl_connector 	= 	TlsConnector::builder().build()	?;
	let client			= imap::client::secure_connect(socket_addr, DOMAIN, &ssl_connector).unwrap(); 

	let user_name = match user_name.len() { 0 => USER_NAME.to_string(), _ => user_name } ;
	info!("201811082104 email_down::main_: Login to email account user_name={}", user_name)	;


	//let mut imap_session= client.login(USER_NAME, &pw).unwrap() ;
	let mut imap_session= client.login(&user_name, &pw).unwrap() ;

	let capabilities	= imap_session.capabilities()?;
	for capability in capabilities.iter() {
			info!("201811082106 email_down::main_: capabilities {}", capability);
	}

	let mailbox			= imap_session.select("INBOX")	?;
	info!("{}", mailbox);

	let search_result	= imap_session.ed_search()?;
	let uids			= search_result.split_whitespace()		;
	for uid in uids {
		match imap_session.ed_fetch(uid) {
			Ok(_) => { }
			Err(e) => error!("ERROR 201811031815 {}", e),
		};
	}
	imap_session.logout()?;
	Ok(0)
}

fn read_credential() ->  (String, String) {
	println!("Enter email and password");
	let stdin		= io::stdin();
	let mut iterator= stdin.lock().lines();
	let user_name 	= iterator.next().unwrap().unwrap();
	let pw			= iterator.next().unwrap().unwrap();
	(user_name, pw)
}

trait EmailDown {
	fn ed_search(&mut self)					->	Result<String, Box<Error>>	;
	fn ed_fetch(&mut self, uid: &str)		->	Result<i32	,  Box<Error>> 	;
	fn ed_command(&mut self, command: &str)	->	Result<String, Box<Error>>	;
}

impl <T:Read + Write> EmailDown for Session<T> {
	fn ed_search(&mut self ) -> Result<String, Box<Error>>
	{
		let command = EMAIL_SEARCH ;
		info!("201811031952 EmailDown: search command={}", command);
		let msg			= self.ed_command(&command)		?;
		info!("201811031952 EmailDown: search result={}", msg);
		Ok(msg)
	}
	
	fn ed_fetch(&mut self, uid: &str) -> Result<i32,  Box<Error>>
	{
		if uid == "*" || uid == "SEARCH"  {	return Ok(0)	};
		let path		= format!("{}/{}", DIR_EMAIL_DOWNLOAD, uid);
		let command		= format!("UID FETCH {} BODY[]", uid);
		info!("201811031806 EmailDown::fetch() uid={}", &uid	);

		let msg			= self.ed_command(&command)?;
		
		let mut output  = File::create(&path)?;
		write!(output, "{}", msg)?;
		info!("201811031806 EmailDown::fetch(): saved {} to {}",  &uid, &path	);

		// move file to ready directory
		let to_file=format!("{}/{}", DIR_EMAIL_READY,	uid);
		fs::rename(&path, &to_file)?	;
		info!("201811082017 EmailDown::fetch: moved file {} to {}", path, to_file);


		// move email to deposit folder
		let move_command = EMAIL_MOVE.to_string().replace("UID_TO_MOVE",  uid) ;
		let move_response= self.ed_command(&move_command)?		;
		info!("201811031806 EmailDown::fetch() response={}",  &move_response	);
		Ok(0)	
	}

	fn ed_command(&mut self, command: &str)	-> Result<String,  Box<Error>> {
		let msg_utf8 	=  self.run_command_and_read_response(&command )?;
		let msg			= 	str::from_utf8(&msg_utf8)?	;
		Ok(msg.to_string())
	}
}
