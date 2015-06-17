#[feature(MemReader)]
pub mod com;
pub mod scada{
	pub mod apdu;
	pub mod asdu;
}

extern crate byteorder;

use std::io::Cursor;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::thread;
use com::Connection;
use std::io;
use std::io::Write;
use std::io::Read;
use std::io::Result;
use std::fmt;
use scada::apdu::Apdu;
use std::string::String;

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

macro_rules! write_html {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "{}", $e));

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}

macro_rules! print_html {
	() => ();
	($e: tt) => (print!("{}",$e ));
	($tag:ident [$($inner:tt)*] $($rest:tt)*) => {{
		print!("<{}>", stringify!($tag));
		print_html!($($inner)*);
		print!("</{}>", stringify!($tag));
		print_html!($($rest)*);
	}}
}

macro_rules! printallln {
	() => ();
	($v : expr) => (println!("{}",$v));
	($v : expr, $($rest:tt)*) =>{{
		print!("{}, ",$v);
		printallln!($($rest)*);
	}}
}

enum Test1 {
	A(u16),
	B(f32,u8),
	C(String,u16)
}

impl fmt::Display for Test1{
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
		match *self {
			Test1::A(ref v) => write!(f,"A = {}",v),
			Test1::B(ref a,ref b) => write!(f,"({}, {})",a,b),
			Test1::C(ref a,ref b) => write!(f,"{} = {}",a,b),
		}
	}
}

#[test]
fn test_byte_reading() {
	let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
	assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
	assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());	
}

#[test]
fn test_bytes_to_test1(){
	let mut b = vec![0u8;10];
	let mut cursor = Cursor::new(b);
	match 2{
		0 => {
			cursor.write_u8(0);
			cursor.write_u16::<BigEndian>(0xA0B);
		}
		1 => {
			cursor.write_u8(1);
			cursor.write_f32::<BigEndian>(0.12);
			cursor.write_u8(4);
		}
		2 => {
			cursor.write_u8(2);
			cursor.write_u16::<BigEndian>(5);
			cursor.write(&[65,66,67,68,69]);
			cursor.write_u16::<BigEndian>(0xABCD);
		}
		_ => {}
	}
	let mut buf = cursor.into_inner();

	// let mut buf = vec![0,0xA,0xB];
	let a = Test1::deserialise(&mut Cursor::new(buf)).unwrap();
	match a{
		Test1::A(a) => assert_eq!(0xA0B,a),
		Test1::B(a,b) => {assert_eq!(0.12,a);assert_eq!(4,b);}
		Test1::C(a,b) => {
			assert_eq!("ABCDE",a);
			assert_eq!(0xABCD,b);
		}
	};
}

impl Test1{
	pub fn deserialise<T : Read>(reader :&mut T)->Result<Test1>{
		let header = try!(reader.read_u8());
		match header{
			0 => {
				Ok(Test1::A(try!(reader.read_u16::<BigEndian>())))
			}
			1 => {
				Ok(Test1::B(try!(reader.read_f32::<BigEndian>()),try!(reader.read_u8())))
			}
			2 => {
				let slen = try!(reader.read_u16::<BigEndian>());
				let mut s = String::new();
				try!(reader.take(slen as u64).read_to_string(&mut s));
				Ok(Test1::C(s,try!(reader.read_u16::<BigEndian>())))
			}
			_ => panic!("Wrong type!"),
		}
	}
}

fn main() 
{
	use std::fmt::Write;
	printallln!(Test1::A(32),"Anca",3);
	let mut out = String::new();
	write_html!(&mut out, html[head[title["Macros guide"]]
		body[h1["Macros are the best!"]]]);
	println!("{}",out );
	print_html!( html[head[title["Macros guide"]]
		body[h1["Macros are the best!"]]]);
	println!("");
	println!("Preparing the connection.");
	let mut con = Connection::connect("localhost:2404").unwrap();
	run(&mut *con).unwrap();

	println!("Shuting down!");

}
