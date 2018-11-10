use std::error::Error;
extern crate regex;
extern crate db;
//extern crate email;

use std::str;
use std::io::{BufRead};
//use failure::{Backtrace, Fail};
use std::fs;

use std::fs::File;
use std::io::{BufReader};
//use std::io::{Read};
use self::regex::Regex;
//use util;

//use constants::LOG_DIR				;
use constants::DIR_EMAIL_READY		;
use constants::DIR_EMAIL_PROCESSED	;
use constants::DIR_EMAIL_ERROR		;
use constants::REGEX_EMAIL_S1		;
use constants::REGEX_EMAIL_S2		;
use constants::REGEX_EMAIL_S3		;
use constants::REGEX_EMAIL_S4		;
use constants::REGEX_EMAIL_S5		;
use constants::REGEX_EMAIL_S6		;
use constants::REGEX_EMAIL_S7		;
use constants::REGEX_EMAIL_S8		;
use constants::REGEX_EMAIL_FROM		;
use constants::REGEX_EMAIL_TO		;
use constants::REGEX_EMAIL_SUBJECT	;
use constants::REGEX_EMAIL_MEMO		;

use constants::SQL_DEPOSIT		;
use constants::SQL_DEPOSIT_PARAM		;


pub fn main_() -> Result<i32, Box<Error>> {
//	util::logger_init(LOG_DIR);
	info!("201811082022 email_proc::main_: started");


	let files_iter = fs::read_dir(DIR_EMAIL_READY)?;
	let count = files_iter.count();
	if count==0 {
		info!("201811092239 email_proc::main_: no email file found in {}. exit", DIR_EMAIL_READY) ;
		return Ok(0);
	}

	let pool 	= db::db::db_pool(None)		;
	let db_conn = db::db::db_conn(&pool)		;
	
	for entry in fs::read_dir(DIR_EMAIL_READY)? {
		let path			=	entry?.path();
		let full_path		=	path.to_str().unwrap() 	;
		let file_name		=	path.file_name().unwrap().to_str().unwrap();
		info!("201811092256 email_proc::main_: found file {}", &full_path) ;
		let (deposit_id, amount) = process_email(full_path)? ;
		match (deposit_id, amount ) {
			(Some(deposit_id), Some(amount) ) => {
				let sql_param = SQL_DEPOSIT_PARAM.to_string()
								.replace("DEPOSIT_ID"	, &deposit_id)
								.replace("ACTUAL_AMOUNT", &amount.to_string())
								.replace("REFERENCE_NO"	, &full_path) ;
				info!("201811081913 email_proc::main_: about to update database with {}", sql_param);

				let result=db::db::runsql_one_row(&db_conn, SQL_DEPOSIT, &[&sql_param] );
				let result_string= result.map(|r|r.to_string());
				info!("201811082341 email_proc::main_: db response:{:?}", result_string);

				if result_string.is_some() {
					let to_file = format!("{}/{}", DIR_EMAIL_PROCESSED, file_name) ;
					fs::rename(full_path, &to_file)?	;
					info!("201811082017 email_proc::main_: moved file {} to {}", full_path, to_file);
				}
				else {
					error!("201811090008 email_proc::main_: database returned no row");
				}
			}
			_	=>{
				let to_file = format!("{}/{}", DIR_EMAIL_ERROR, file_name) ;
				fs::rename(full_path, &to_file)?	;
				info!("201811082017 email_proc::main_: moved file {} to {}", full_path, to_file);
			}
		}

	}
	info!("201811082022 email_proc::main_: ended");
	Ok(0)
}


fn process_email(path: &str ) -> Result <(Option<String>, Option<String>), Box<Error>>{
	let mut pass		: [usize; 12]	= [0;12];
	let mut amount		: String		= "".to_string();
	let mut deposit_id	: String		= "".to_string();

	let input	=	File::open(&path) ?	;
	let lines 	=	BufReader::new(input).lines()	;

	info!("201811072941 email_proc::process_email Opened email file for processing. path={}", path);

	lazy_static! {
		static ref re_s1		: Regex	= Regex::new(REGEX_EMAIL_S1).unwrap();
		static ref re_s2		: Regex	= Regex::new(REGEX_EMAIL_S2).unwrap();
		static ref re_s3		: Regex	= Regex::new(REGEX_EMAIL_S3).unwrap();
		static ref re_s4		: Regex	= Regex::new(REGEX_EMAIL_S4).unwrap();
		static ref re_s5		: Regex	= Regex::new(REGEX_EMAIL_S5).unwrap();
		static ref re_s6		: Regex	= Regex::new(REGEX_EMAIL_S6).unwrap();
		static ref re_s7		: Regex	= Regex::new(REGEX_EMAIL_S7).unwrap();
		static ref re_s8		: Regex	= Regex::new(REGEX_EMAIL_S8).unwrap();
		static ref re_from		: Regex	= Regex::new(REGEX_EMAIL_FROM).unwrap();
		static ref re_to		: Regex	= Regex::new(REGEX_EMAIL_TO).unwrap();
		static ref re_subject 	: Regex	= Regex::new(REGEX_EMAIL_SUBJECT).unwrap();
		static ref re_memo 		: Regex	= Regex::new(REGEX_EMAIL_MEMO).unwrap();
	}

	for (num, line) in lines.enumerate() {
		//each match can appear only once. if not, add 10000,which indicates fraud
		let l = &line.unwrap();
		if  re_s1.is_match(l)	&& pass[0]==0	{	pass[0]	= num						;	continue;}
		if  re_s1.is_match(l)	&& pass[0]!=0	{	pass[0]	= 100000+num				;	continue;}
		if  re_s2.is_match(l)	&& pass[1]==0	{	pass[1]	= num						;	continue;}
		if  re_s3.is_match(l)	&& pass[2]==0	{	pass[2]	= num						;	continue;}
		if  re_s4.is_match(l)	&& pass[3]==0	{	pass[3]	= num						;	continue;}
		if  re_s5.is_match(l)	&& pass[4]==0	{	pass[4]	= num						;	continue;}
		if  re_s5.is_match(l)	&& pass[4]!=0	{	pass[4]	= 10000+num					;	continue;}
		if  re_s6.is_match(l)	&& pass[1]!=0 &&	pass[5]==0	{	pass[5]	= num		;	continue;}
		if  re_s6.is_match(l)	&& pass[1]!=0 &&	pass[5]!=0	{	pass[5]	= 10000+num	;	continue;}
		if  re_s7.is_match(l)	&& pass[2]!=0 &&	pass[6]==0	{	pass[6]	= num		;	continue;}
		if  re_s7.is_match(l)	&& pass[2]!=0 &&	pass[6]!=0	{	pass[6]	= 10000+num	;	continue;}
		if  re_s8.is_match(l)	&& pass[3]!=0 &&	pass[7]==0	{	pass[7]	= num		;	continue;}
		if  re_s8.is_match(l)	&& pass[3]!=0 &&	pass[7]!=0	{	pass[7]	= 10000+num	;	continue;}
		
		if re_from.is_match(l)	&& pass[8]==0 	{	pass[8]	= num						;	continue;}
		if re_from.is_match(l)	&& pass[8]!=0	{	pass[8]	= 10000+num					;	continue;}
		if re_to.is_match(l)	&& pass[9]==0	{	pass[9]	= num						;	continue;}
		if re_to.is_match(l)	&& pass[9]!=0	{	pass[9]	= 10000+num					;	continue;}

		if re_subject.is_match(l)	&& pass[10]==0 	{
			pass[10]	= num;
			for cap in re_subject.captures_iter(l) {
				amount = cap[1].to_string();
			}
			continue;
		}
		if re_subject.is_match(l)	&& pass[10]!=0 	{pass[10]	= 10000+num;continue;}

		if re_memo.is_match(l) &&pass[11]==0	{	
			pass[11]	= num;	
			for cap in re_memo.captures_iter(l) {
				deposit_id= cap[1].to_string();
			}
			continue;
		}
		if re_memo.is_match(l) &&pass[11]!=0	{	pass[11]	= 10000+num;	continue;}
	}

	for (num, line) in pass.iter().enumerate() {
		info!("201811081041 email_proc::process_email path={} pass[{}]={}", path, num, line);
	}
	info!("201811081041 email_proc::process_email path={} deposit_id={}", path, deposit_id );
	info!("201811081041 email_proc::process_email path={} amount={}", path, amount );

	if		pass[1]	==	pass[0]+1 
		&&	pass[2]	==	pass[1]+1
		&&	pass[3]	==	pass[2]+1
		&&	pass[4]	> 	pass[0]
		&&	pass[4]	< 	10000
		&&	pass[5]	==	pass[4]+1
		&&	pass[6]	==	pass[5]+1
		&&	pass[7]	==	pass[6]+1
		&&	pass[8]	>	pass[7]
		&&	pass[8]	<	10000
		&&	pass[9]	==	pass[8]+1
		&&	pass[10]==	pass[9]+2 
		&&	deposit_id.len()==36
		&&	amount.len() 	>=4	{
		info!("201811070001 email_proc::process_email Security check passed for {}", &path);
		return Ok((Some(deposit_id), Some(amount) ));
	}
	else {
		error! ("201811081721 email_proc::process_email Security check failed for {} ", &path);
		return Ok((None, None ));
	}
}

/*
fn save_to_db() {
	
}
fn get_file() {
}
fn rename_file(){
}
fn (
*/


