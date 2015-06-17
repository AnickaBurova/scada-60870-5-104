// use super::asdu::Asdu;
use super::super::com::ConnectionSettings;
use std::io::Write;
use std::io::Read;
use std::io::Result;
use std::clone::Clone;
use std::fmt;
use std::fmt::Display;

pub enum Apdu {
	TestFRCon,
	TestFRAct,
	StopDTCon,
	StopDTAct,
	StartDTAct,
	StartDTCon,
	IFormat{send_seq_num : u16, receive_seq_num : u16},
	SFormat{receive_seq_num : u16}
}

impl Apdu {

	pub fn deserialise<T : Read>(reader : &mut T, connection_settings : &ConnectionSettings) -> Result<Apdu>{
		let mut buf = [0u8;2];
		let readsize = try!(reader.read(&mut buf));
		assert!(buf[0] == 0x68u8);
		assert!(readsize == 2);
		let apdulength = buf[1];
		let mut abuf = [0u8;253];
		let readsize = try!(reader.read(&mut abuf));
		assert!(readsize == apdulength as usize);
		match abuf[0]{
			0x83 => Ok(Apdu::TestFRCon),
			0x43 => Ok(Apdu::TestFRAct),
			0x23 => Ok(Apdu::StopDTCon),
			0x13 => Ok(Apdu::StopDTAct),
			0x0b => Ok(Apdu::StartDTCon),
			0x07 => Ok(Apdu::StartDTAct),
			b if b & 1u8 == 0u8 => {
				let send_seq_num = ((abuf[0] & 0xfeu8) >> 1) as u16 + ((abuf[1] as u16) << 7);
				let rcv_seq_num = ((abuf[2] & 0xfeu8) >> 1) as u16 + ((abuf[3] as u16) << 7);
				//TODO: deserialise asdu
				Ok(Apdu::IFormat{send_seq_num : send_seq_num, receive_seq_num : rcv_seq_num})
			}
			b if b & 2u8 == 0u8 => {
				// 0 and 1 are not used for send_seq_num
				let rcv_seq_num = ((abuf[2] & 0xfeu8) >> 1) as u16 + ((abuf[3] as u16) << 7);
				Ok(Apdu::SFormat{ receive_seq_num : rcv_seq_num})
			}
			_ => Ok(Apdu::StartDTAct),
		}
	}

	pub fn serialise<T : Write>(&self, writer : &mut T, connection_settings : &ConnectionSettings) -> Result<()> {
		let mut buf : [u8;6] = 
			match *self{
				Apdu::TestFRCon => [0x68u8,4,0x83,0,0,0],
				Apdu::TestFRAct => [0x68u8,4,0x43,0,0,0],
				Apdu::StopDTCon => [0x68u8,4,0x23,0,0,0],
				Apdu::StopDTAct => [0x68u8,4,0x13,0,0,0],
				Apdu::StartDTCon => [0x68u8,4,0xb,0,0,0],
				Apdu::StartDTAct => [0x68u8,4,0x7,0,0,0],
				Apdu::SFormat{ref receive_seq_num} => [0x68u8,4,7,0,((receive_seq_num << 1) & 0xffu16 ) as u8, ((receive_seq_num << 7) & 0xffu16) as u8],
				Apdu::IFormat{ref send_seq_num, ref receive_seq_num} =>
					[0x68u8,0u8,((send_seq_num << 1) & 0xffu16 ) as u8, ((send_seq_num << 7) & 0xffu16) as u8,
									((receive_seq_num << 1) & 0xffu16 ) as u8, ((receive_seq_num << 7) & 0xffu16) as u8],
			};
		try!(writer.write_all(&buf));
		Ok(())
	}
}

impl Display for Apdu{
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result{
		match *self{
			Apdu::TestFRCon => write!(f,"Test fr confirm"),
			Apdu::TestFRAct => write!(f,"Test fr activation"),
			Apdu::StopDTCon => write!(f,"Stop data transfer confirm"),
			Apdu::StopDTAct => write!(f,"Stop data transfer activation"),
			Apdu::StartDTCon => write!(f,"Start data transfer confirm"),
			Apdu::StartDTAct => write!(f,"Start data transfer activation"),
			Apdu::SFormat{ref receive_seq_num} => write!(f,"SFormat receive seq number: {}",receive_seq_num),
			Apdu::IFormat{ref send_seq_num, ref receive_seq_num} => write!(f,"IFormat send seq number: {}, receive seq number: {}",send_seq_num,receive_seq_num),
		}
	}
}