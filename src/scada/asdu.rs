// use super::information_object::InformationObject;
use std::clone::Clone;
use std::io::Read;
use std::io::Write;
use std::io::Result;
use super::super::com::ConnectionSettings;

// #[derive(Clone)]
// pub enum AsduType
// {
// 	/**
// 	 * 1 - Single-point information without time tag
// 	 */
// 	M_SP_NA_1 = 1, 
// 	/**
// 	 * 2 - Single-point information with time tag
// 	 */
// 	M_SP_TA_1 = 2, 
// 	/**
// 	 * 3 - Double-point information without time tag
// 	 */
// 	M_DP_NA_1 = 3, 
// 	/**
// 	 * 4 - Double-point information with time tag
// 	 */
// 	M_DP_TA_1 = 4, 
// 	/**
// 	 * 5 - Step position information
// 	 */
// 	M_ST_NA_1 = 5, 
// 	/**
// 	 * 6 - Step position information with time tag
// 	 */
// 	M_ST_TA_1 = 6, 
// 	/**
// 	 * 7 - Bitstring of 32 bit
// 	 */
// 	M_BO_NA_1 = 7, 
// 	/**
// 	 * 8 - Bitstring of 32 bit with time tag
// 	 */
// 	M_BO_TA_1 = 8, 
// 	/**
// 	 * 9 - Measured value, normalized value
// 	 */
// 	M_ME_NA_1 = 9, 
// 	/**
// 	 * 10 - Measured value, normalized value with time tag
// 	 */
// 	M_ME_TA_1 = 10, 
// 	/**
// 	 * 11 - Measured value, scaled value
// 	 */
// 	M_ME_NB_1 = 11, 
// 	/**
// 	 * 12 - Measured value, scaled value with time tag
// 	 */
// 	M_ME_TB_1 = 12, 
// 	/**
// 	 * 13 - Measured value, short floating point number
// 	 */
// 	M_ME_NC_1 = 13, 
// 	/**
// 	 * 14 - Measured value, short floating point number with time tag
// 	 */
// 	M_ME_TC_1 = 14, 
// 	/**
// 	 * 15 - Integrated totals
// 	 */
// 	M_IT_NA_1 = 15, 
// 	/**
// 	 * 16 - Integrated totals with time tag
// 	 */
// 	M_IT_TA_1 = 16, 
// 	/**
// 	 * 17 - Event of protection equipment with time tag
// 	 */
// 	M_EP_TA_1 = 17, 
// 	/**
// 	 * 18 - Packed start events of protection equipment with time tag
// 	 */
// 	M_EP_TB_1 = 18, 
// 	/**
// 	 * 19 - Packed output circuit information of protection equipment with time tag
// 	 */
// 	M_EP_TC_1 = 19, 
// 	/**
// 	 * 20 - Packed single-point information with status change detection
// 	 */
// 	M_PS_NA_1 = 20, 
// 	/**
// 	 * 21 - Measured value, normalized value without quality descriptor
// 	 */
// 	M_ME_ND_1 = 21, 
// 	/**
// 	 * 30 - Single-point information with time tag CP56Time2a
// 	 */
// 	M_SP_TB_1 = 30, 
// 	/**
// 	 * 31 - Double-point information with time tag CP56Time2a
// 	 */
// 	M_DP_TB_1 = 31, 
// 	/**
// 	 * 32 - Step position information with time tag CP56Time2a
// 	 */
// 	M_ST_TB_1 = 32, 
// 	/**
// 	 * 33 - Bitstring of 32 bits with time tag CP56Time2a
// 	 */
// 	M_BO_TB_1 = 33, 
// 	/**
// 	 * 34 - Measured value, normalized value with time tag CP56Time2a
// 	 */
// 	M_ME_TD_1 = 34, 
// 	/**
// 	 * 35 - Measured value, scaled value with time tag CP56Time2a
// 	 */
// 	M_ME_TE_1 = 35, 
// 	/**
// 	 * 36 - Measured value, short floating point number with time tag CP56Time2a
// 	 */
// 	M_ME_TF_1 = 36, 
// 	/**
// 	 * 37 - Integrated totals with time tag CP56Time2a
// 	 */
// 	M_IT_TB_1 = 37, 
// 	/**
// 	 * 38 - Event of protection equipment with time tag CP56Time2a
// 	 */
// 	M_EP_TD_1 = 38, 
// 	/**
// 	 * 39 - Packed start events of protection equipment with time tag CP56Time2a
// 	 */
// 	M_EP_TE_1 = 39, 
// 	/**
// 	 * 40 - Packed output circuit information of protection equipment with time tag CP56Time2a
// 	 */
// 	M_EP_TF_1 = 40, 
// 	/**
// 	 * 45 - Single command
// 	 */
// 	C_SC_NA_1 = 45, 
// 	/**
// 	 * 46 - Double command
// 	 */
// 	C_DC_NA_1 = 46, 
// 	/**
// 	 * 47 - Regulating step command
// 	 */
// 	C_RC_NA_1 = 47, 
// 	/**
// 	 * 48 - Set point command, normalized value
// 	 */
// 	C_SE_NA_1 = 48, 
// 	/**
// 	 * 49 - Set point command, scaled value
// 	 */
// 	C_SE_NB_1 = 49, 
// 	/**
// 	 * 50 - Set point command, short floating point number
// 	 */
// 	C_SE_NC_1 = 50, 
// 	/**
// 	 * 51 - Bitstring of 32 bits
// 	 */
// 	C_BO_NA_1 = 51, 
// 	/**
// 	 * 58 - Single command with time tag CP56Time2a
// 	 */
// 	C_SC_TA_1 = 58, 
// 	/**
// 	 * 59 - Double command with time tag CP56Time2a
// 	 */
// 	C_DC_TA_1 = 59, 
// 	/**
// 	 * 60 - Regulating step command with time tag CP56Time2a
// 	 */
// 	C_RC_TA_1 = 60, 
// 	/**
// 	 * 61 - Set-point command with time tag CP56Time2a, normalized value
// 	 */
// 	C_SE_TA_1 = 61, 
// 	/**
// 	 * 62 - Set-point command with time tag CP56Time2a, scaled value
// 	 */
// 	C_SE_TB_1 = 62, 
// 	/**
// 	 * 63 - C_SE_TC_1 Set-point command with time tag CP56Time2a, short floating point number
// 	 */
// 	C_SE_TC_1 = 63, 
// 	/**
// 	 * 64 - Bitstring of 32 bit with time tag CP56Time2a
// 	 */
// 	C_BO_TA_1 = 64, 
// 	/**
// 	 * 70 - End of initialization
// 	 */
// 	M_EI_NA_1 = 70, 
// 	/**
// 	 * 100 - Interrogation command
// 	 */
// 	C_IC_NA_1 = 100, 
// 	/**
// 	 * 101 - Counter interrogation command
// 	 */
// 	C_CI_NA_1 = 101, 
// 	/**
// 	 * 102 - Read command
// 	 */
// 	C_RD_NA_1 = 102, 
// 	/**
// 	 * 103 - Clock synchronization command
// 	 */
// 	C_CS_NA_1 = 103, 
// 	/**
// 	 * 104 - Test command
// 	 */
// 	C_TS_NA_1 = 104, 
// 	/**
// 	 * 105 - Reset process command
// 	 */
// 	C_RP_NA_1 = 105, 
// 	/**
// 	 * 106 - Delay acquisition command
// 	 */
// 	C_CD_NA_1 = 106, 
// 	/**
// 	 * 107 - Test command with time tag CP56Time2a
// 	 */
// 	C_TS_TA_1 = 107, 
// 	/**
// 	 * 110 - Parameter of measured value, normalized value
// 	 */
// 	P_ME_NA_1 = 110, 
// 	/**
// 	 * 111 - Parameter of measured value, scaled value
// 	 */
// 	P_ME_NB_1 = 111, 
// 	/**
// 	 * 112 - Parameter of measured value, short floating point number
// 	 */
// 	P_ME_NC_1 = 112, 
// 	/**
// 	 * 113 - Parameter activation
// 	 */
// 	P_AC_NA_1 = 113, 
// 	/**
// 	 * 120 - File ready
// 	 */
// 	F_FR_NA_1 = 120, 
// 	/**
// 	 * 121 - Section ready
// 	 */
// 	F_SR_NA_1 = 121, 
// 	/**
// 	 * 122 - Call directory, select file, call file, call section
// 	 */
// 	F_SC_NA_1 = 122, 
// 	/**
// 	 * 123 - Last section, last segment
// 	 */
// 	F_LS_NA_1 = 123, 
// 	/**
// 	 * 124 - Ack file, ack section
// 	 */
// 	F_AF_NA_1 = 124, 
// 	/**
// 	 * 125 - Segment
// 	 */
// 	F_SG_NA_1 = 125, 
// 	/**
// 	 * 126 - Directory
// 	 */
// 	F_DR_TA_1 = 126, 
// 	/**
// 	 * 127 - QueryLog, request archive file
// 	 */
// 	F_SC_NB_1 = 127, 
// }

// impl AsduType{
// 	fn deserialise<T : Read>(reader : &mut T) -> Result<AsduType>{
// 		let mut buf = [0u8;1];
// 		let _ = try!(reader.read(&mut buf));
// 		Ok(buf[0] as AsduType)
// 	}

// }

#[derive(Clone)]
pub enum CauseOfTransmission
{
	PERIODIC = 1,
	BACKGROUND_SCAN = 2,
	SPONTANEOUS = 3,
	INITIALIZED = 4,
	REQUEST = 5,
	ACTIVATION = 6,
	ACTIVATION_CON = 7,
	DEACTIVATION = 8,
	DEACTIVATION_CON = 9,
	ACTIVATION_TERMINATION = 10,
	RETURN_INFO_REMOTE = 11,
	RETURN_INFO_LOCAL = 12,
	FILE_TRANSFER = 13,
	INTERROGATED_BY_STATION = 20,
	INTERROGATED_BY_GROUP_1 = 21,
	INTERROGATED_BY_GROUP_2 = 22,
	INTERROGATED_BY_GROUP_3 = 23,
	INTERROGATED_BY_GROUP_4 = 24,
	INTERROGATED_BY_GROUP_5 = 25,
	INTERROGATED_BY_GROUP_6 = 26,
	INTERROGATED_BY_GROUP_7 = 27,
	INTERROGATED_BY_GROUP_8 = 28,
	INTERROGATED_BY_GROUP_9 = 29,
	INTERROGATED_BY_GROUP_10 = 30,
	INTERROGATED_BY_GROUP_11 = 31,
	INTERROGATED_BY_GROUP_12 = 32,
	INTERROGATED_BY_GROUP_13 = 33,
	INTERROGATED_BY_GROUP_14 = 34,
	INTERROGATED_BY_GROUP_15 = 35,
	INTERROGATED_BY_GROUP_16 = 36,
	REQUESTED_BY_GENERAL_COUNTER = 37,
	REQUESTED_BY_GROUP_1_COUNTER = 38,
	REQUESTED_BY_GROUP_2_COUNTER = 39,
	REQUESTED_BY_GROUP_3_COUNTER = 40,
	REQUESTED_BY_GROUP_4_COUNTER = 41,
	UNKNOWN_TYPE_ID = 44,
	UNKNOWN_CAUSE_OF_TRANSMISSION = 45,
	UNKNOWN_COMMON_ADDRESS_OF_ASDU = 46,
	UNKNOWN_INFORMATION_OBJECT_ADDRESS = 47
}

pub struct AsduProperties{
	cause_of_transmission : CauseOfTransmission,
	is_test : bool,
	is_negative_confirm : bool,
	originator_address : u8,
	common_address : u16,
}

pub enum InformationElement{
	NormalizedValue(f64),
	ScaledValue(u16),
}

pub struct InformationObject{
	object_address : i32,
	elements : Vec<InformationElement>,
}

pub enum InformationObjects{
	SequenceOfObjects(Vec<InformationObject>),
	SequenceOfElements(InformationObject),
}

pub enum Asdu{
	Placeholder,
	M_ME_NA_1(AsduProperties,InformationObjects),
}

macro_rules! is_bit_on {
	($value : expr, $bit : expr) => (($value & $bit) != 0)
}

macro_rules! pinc {
	($val : ident) => ({let temp = $val;$val += 1; temp})
}

impl Asdu{
	pub fn deserialise<T : Read>(reader : &mut T, connection_settings : ConnectionSettings) -> Result<Asdu>{
		let mut buf = vec![0u8;connection_settings.get_asdu_size()]; // 5 is maximum asdu header size
		try!(reader.read(&mut buf));
		let mut rindex = 0;
		let typeid = buf[rindex];
		rindex = 1;
		let is_sequence_of_elements = is_bit_on!(buf[rindex], 0x80u8);
		let sequence_length = buf[rindex] & 0x7fu8;
		rindex = 2;
		let cot = buf[rindex] & 0x3fu8;
		let is_test = is_bit_on!(buf[rindex], 0x80);
		let is_negative_confirm = is_bit_on!(buf[rindex], 0x40);
		rindex = 3;
		let originator_address = match connection_settings.get_cot_field_length(){
			2 => buf[pinc!(rindex)],
			// {
			// 	let t = buf[rindex];
			// 	rindex+=1;
			// 	t
			// }
			_ => 0u8,
		};
		let common_address = match connection_settings.get_common_address_field_length() {
			1 => buf[rindex] as u16,
			_ => (buf[rindex] as u16) + ((buf[rindex] as u16) << 8),
		};
		let props = AsduProperties{
			cause_of_transmission : CauseOfTransmission::ACTIVATION,
			is_test : is_test,
			is_negative_confirm : is_negative_confirm,
			originator_address : originator_address,
			common_address : common_address,
		};
		match typeid{
			// 9 => Ok(M_ME_NA_1(props,SequenceOfElements(InformationObject{}))),
			_ => Ok(Asdu::Placeholder),
		}
	}
}