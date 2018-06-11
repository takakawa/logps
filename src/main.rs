use std::env;
use std::time::{SystemTime,UNIX_EPOCH};
use std::io;


fn main() {
	for argument in env::args(){
		println!("args = {}",argument);
	}
        let mut log_cnt = 0;
	let mut start = SystemTime::now();	
	

	loop {
		let mut guess = String::new();
		let bytes = io::stdin().read_line(&mut guess)
		.ok()
		.expect("Failed to read line");
		log_cnt += 1;

		match start.elapsed() {
			Ok(elapsed) => {
				if elapsed.as_secs() >= 1 {
					println!("elapsed: {}s logps: {}",elapsed.as_secs(),log_cnt);
					log_cnt = 0;
					start = SystemTime::now();
				}
			}
			Err(e)=>{
				println!("Error: {:?}",e);
			}

		}
	}
}
