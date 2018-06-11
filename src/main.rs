use std::env;
use std::time::{SystemTime};
use std::io;


fn main() {
	for argument in env::args(){
		println!("args = {}",argument);
	}
        let mut log_cnt = 0;
	let mut start = SystemTime::now();	
	

	loop {
		let mut log = String::new();
		io::stdin().read_line(& mut log)
		.ok()
		.expect("Failed to read line");

		log_cnt += 1;
		match start.elapsed() {
			Ok(elapsed) => {
				if elapsed.as_secs() >= 1 {
					println!("logps: {} [{}s]",log_cnt,elapsed.as_secs(),);
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
