use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::io;

fn main() {
    println!("Broadcast Setup:");
	
	//Callsign
	println!("Please enter a callsign:");
    let mut callsign = String::new();
    io::stdin().read_line(&mut callsign)
        .expect("Failed to read line");

	//Broadcast Name
	println!("Please enter a broadcast name:");
    let mut bcast = String::new();
    io::stdin().read_line(&mut bcast)
        .expect("Failed to read line");


	let message = Arc::new(Mutex::new(String::from("Broadcast Going Online...")));
	let message2 = message.clone();
	
	let die = Arc::new(Mutex::new(false));
	let die2 = die.clone();
	
	
	//Kick Broadcast
	let broadcast = thread::spawn(move || {
        let socket = UdpSocket::bind("127.0.0.1:1980").unwrap();
		socket.set_broadcast(true).expect("set_broadcast call failed");
		loop{
			//socket.send_to()
			//socket2 = socket.clone();
				sleep(std::time::Duration::new(2,0));
				let diet = die2.lock().unwrap();
			if!(*diet){
				let messaget = message2.lock().unwrap();
				socket.send_to(messaget.as_bytes(), "255.255.255.255:1980").unwrap();
			}else{
				println!("Shutdown Notice Received. Shutting Down Broadcast Thread.");
				break;
			}
		}
    });
	
	//Tell user they're broadcasting.
	println!("Now Broadcasting as {} from {}", callsign.trim(), bcast.trim());
	//Enter your message
	loop{
		let message3 = message.clone();
		let die3 = die.clone();
		println!("Change the broadcast message?:");
	    let mut text = String::new();
	    io::stdin().read_line(&mut text)
        	.expect("Failed to read line");
		text = text.trim().to_string();
		if text=="q" {
			let mut die3 = die3.lock().unwrap();
			*die3 = true;
			println!("Shutdown issued to thread.");
			break();
		}
		let mut message3 = message3.lock().unwrap();
		text = format!("=[{} from {}]= \n =Begin Message= {} =End Message=.", callsign.trim(), bcast.trim(), text);
		*message3 = text;
	}

	let res = broadcast.join();

}