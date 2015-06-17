
use std::net::TcpStream;
use std::io::Result;
use std::net::Shutdown;
use std::io::Write;
use std::io::Read;
use super::scada::apdu::Apdu;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::io::Error;
use std::io::ErrorKind;

pub struct Connection
{
	stream : TcpStream,
	connection_settings : ConnectionSettings,
	read_thread_writer : Option<Sender<i32>>,
	read_thread_reader : Option<Receiver<Option<bool>>>,
}

impl Drop for Connection
{
	fn drop(&mut self)
	{
		println!("Connection is closing");
	}
}


impl Connection
{
	pub fn connect(address: &str) -> Result<Box<Connection>>
	{
		// connect to scada server
		let mut result = Box::new(Connection{
								stream : try!(TcpStream::connect(address)),
								connection_settings : ConnectionSettings::default(),
								read_thread_writer : None,
								read_thread_reader : None,
							});

		println!("Connection connect established.\nSending start data transfer.");
		// after connection, send stard data transfer
		// result.start_data_transfer();
		// println!("Data transfer sent.");

		Ok(result)
	}

	// pub fn get_writer(&mut self) -> &mut Write{
	// 	&self.stream as &mut Write
	// }

	pub fn start_data_transfer(&mut self) -> Result<()>
	{
		let startdt_act_buffer = [0x68u8,0x04,0x07,0,0,0];
		try!(self.stream.write_all(&startdt_act_buffer));
		Ok(())
	}

	pub fn send(&mut self, apdu : &Apdu) -> Result<()> {
		try!(apdu.serialise(&mut self.stream,&self.connection_settings));
		Ok(())
	}

	pub fn send_to_reader(&mut self,value : i32) -> Result<()> {
		match self.read_thread_writer{
			Some(ref writer) => try!(writer.send(value).map_err(|err| Error::new(ErrorKind::Other,err))),
			None => return Err(Error::new(ErrorKind::Other,"Reading thread is not running")),
		};
		Ok(())
	}

	pub fn start_reading(&mut self) -> Result<()> {
		let (tx, rx) = channel::<i32>();	
		self.read_thread_writer = Some(tx);
		let (mtx,mrx) = channel::<Option<bool>>();
		self.read_thread_reader = Some(mrx);

		let mut reader = try!(self.stream.try_clone());

		let cs = self.connection_settings.clone();

		let _ = thread::spawn(move || {
			println!("Running the reading thread.");
			loop {
				let apdu = match Apdu::deserialise(&mut reader,&cs){
					Ok(a) => a,
					Err(e) => {
						println!("Failed deserialising apdu: {}",e);
						return;
					}
				};
				println!("Read this apdu: {}",apdu );

				mtx.send(Some(true));
			}
			// for value in rx.iter() {
			// 	match value {
			// 		0i32 => {
			// 			println!("Quiting reading thread.");
			// 			break;
			// 		}
			// 		_ => println!("Reading thread got value {}", value),

			// 	}
			// }
		});


		Ok(())
	}
}

#[derive(Clone)]
pub struct ConnectionSettings {
	message_fragment_timeout : i32,
	cot_field_length : usize,
	common_address_field_length : usize,
	ioa_field_length : usize,
	max_time_no_ack_received : i32,
	max_time_no_ack_sent : i32,
	max_idle_time : i32,
	max_unconfirmed_ipdus_received : i32,

}

impl ConnectionSettings {
	pub fn new(message_fragment_timeout : i32
		, cot_field_length : usize
		, common_address_field_length : usize
		, ioa_field_length : usize
		, max_time_no_ack_received : i32
		, max_time_no_ack_sent : i32
		, max_idle_time : i32
		, max_unconfirmed_ipdus_received : i32
		) -> ConnectionSettings {
		ConnectionSettings{message_fragment_timeout : message_fragment_timeout
		, cot_field_length : cot_field_length
		, common_address_field_length : common_address_field_length
		, ioa_field_length : ioa_field_length
		, max_time_no_ack_received : max_time_no_ack_received
		, max_time_no_ack_sent : max_time_no_ack_sent
		, max_idle_time : max_idle_time
		, max_unconfirmed_ipdus_received : max_unconfirmed_ipdus_received
		}
	}
	pub fn default() -> ConnectionSettings {
		ConnectionSettings{message_fragment_timeout : 5000
		, cot_field_length : 2
		, common_address_field_length : 2
		, ioa_field_length : 3
		, max_time_no_ack_received : 15000
		, max_time_no_ack_sent : 10000
		, max_idle_time : 20000
		, max_unconfirmed_ipdus_received : 8
		}
	}

	pub fn get_message_fragment_timeout(&self) -> i32 { self.message_fragment_timeout }
	pub fn get_cot_field_length(&self) -> usize { self.cot_field_length }
	pub fn get_common_address_field_length(&self) -> usize { self.common_address_field_length }
	pub fn get_ioa_field_length(&self) -> usize { self.ioa_field_length }
	pub fn get_max_time_no_ack_received(&self) -> i32 { self.max_time_no_ack_received }
	pub fn get_max_time_no_ack_sent(&self) -> i32 { self.max_time_no_ack_sent }
	pub fn get_max_idle_time(&self) -> i32 { self.max_idle_time }
	pub fn get_max_unconfirmed_ipdus_received(&self) -> i32 { self.max_unconfirmed_ipdus_received }

	pub fn get_asdu_size(&self) -> usize{
		2usize + self.get_cot_field_length() + self.get_common_address_field_length()
	}
}
