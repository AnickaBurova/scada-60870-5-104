pub mod com;
pub mod scada{
	pub mod apdu;
	pub mod asdu;
	pub mod information_object;
	pub mod information_element;
}

use std::thread;
use com::Connection;
use std::io;
use std::io::Write;
use std::io::Read;
use std::io::Result;
use std::fmt;
use scada::apdu::Apdu;
use std::string::String;



	// private static final byte[] TESTFR_CON_BUFFER = new byte[] { 0x68, 0x04, (byte) 0x83, 0x00, 0x00, 0x00 };
	// private static final byte[] TESTFR_ACT_BUFFER = new byte[] { 0x68, 0x04, (byte) 0x43, 0x00, 0x00, 0x00 };
	// private static final byte[] STARTDT_ACT_BUFFER = new byte[] { 0x68, 0x04, 0x07, 0x00, 0x00, 0x00 };
	// private static final byte[] STARTDT_CON_BUFFER = new byte[] { 0x68, 0x04, 0x0b, 0x00, 0x00, 0x00 };
fn run(con : &mut Connection) -> Result<()> {
	// let startdtact = ;
	println!("Connection ready. Going to sleep...");
	try!(con.send(&Apdu::StartDTAct));
	try!(con.start_reading());
	loop {
		print!("Enter command: ");
		let mut input = String::new();
		try!(io::stdin().read_line(&mut input));
		// input = input.trim();
		match input.trim() {
			"q" => {
				println!("Quiting!");
				// try!(con.send_to_reader(0));
				break;
			}
			"s" => {
				try!(con.send_to_reader(10));
			}
			_ => println!("unknown command {}", &input),
		}

	}
	Ok(())
}

fn main() 
{
	println!("Preparing the connection.");
	let mut con = Connection::connect("localhost:2404").unwrap();
	run(&mut *con).unwrap();

	println!("Shuting down!");

}
