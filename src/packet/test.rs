include!(concat!(env!("OUT_DIR"), "/test.rs"));

#[test]
fn test_little_endian() {
	let mut buf = vec![0; 100];
	buf[0] = 1; // set lower bit
	buf[1] = 2; // set second bit
	buf[2] = 255; // set all bits
	buf[3] = 1; // set lower bit
	buf[4] = 7; // set lower bit
	let pkt = MutableLittleEndianPacket::owned(buf).unwrap();
	println!("{:?}", pkt);
	assert!(pkt.get_fifteen_bits() == 513);
	assert!(pkt.get_five_bits() == 31);
	assert!(pkt.get_sixteen_bits() == 39);
	assert!(pkt.get_three_bits() == 7);
}
