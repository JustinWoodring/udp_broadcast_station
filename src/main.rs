use cursive::view::SizeConstraint::Fixed;
use cursive::view::SizeConstraint::AtLeast;
use cursive::view::Identifiable;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use cursive::Cursive;
use cursive::views::{LinearLayout, Panel, Dialog, EditView, Button, TextView, BoxView};
use cursive::direction::Orientation;

fn main() {
	//Init Cursive
	let mut siv = Cursive::default();
	
	//Setup Broadcast
	siv.add_layer(
		BoxView::with_min_size((50, 15),
		Panel::new(
			LinearLayout::new(Orientation::Vertical)
			.child(
				Dialog::new()
				.title("Enter a callsign.")
				.padding((1,1,1,0))
				.content(
					EditView::new()
					.with_id("callsign"),
				)
			).child(
				Dialog::new()
				.title("Enter a broadcast name.")
				.padding((1,1,1,0))
				.content(
					EditView::new()
					.with_id("broadcast_name"),
				),
			).child(
				Button::new("Ok", |s| {
					let callsign = s.call_on_id(
						"callsign",
						|view: &mut EditView| view.get_content(),
					).unwrap();
					let broadcast_name = s.call_on_id(
						"broadcast_name",
						|view: &mut EditView| view.get_content(),
					).unwrap();
					init_broadcast(s, &callsign, &broadcast_name);
				})
			)
		).title("Broadcast Setup")
	
	));
	
	siv.run();
}


//Init Broadcast
fn init_broadcast(s: &mut Cursive, callsign : &str, broadcast_name : &str){
	//Init Mutexes
	let message = Arc::new(Mutex::new(String::from("Broadcast Going Online...")));
	let message2 = message.clone();
	
	
	let die = Arc::new(Mutex::new(false));
	let die2 = die.clone();
	
	let broadcast = thread::spawn(move || {
        let socket = UdpSocket::bind("10.0.0.72:1980").unwrap();
		socket.set_broadcast(true).expect("set_broadcast call failed");
		loop{
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
	
	let messaget = Arc::new(Mutex::new(String::from("Broadcast Going Online...")));
	let mut need_refresh = Arc::new(Mutex::new(true));
	
	let csign = Arc::new(Mutex::new(String::from(callsign)));
	let bcast = Arc::new(Mutex::new(String::from(broadcast_name)));
	let mut csign2 = csign.clone();
	let mut bcast2 = bcast.clone();
	
	s.pop_layer();
	

		let mut need_refresh = need_refresh.clone();
		let mut messaget = messaget.clone();
		let mut messaget2 = messaget.clone();
		let mut messaget3 = messaget.clone();
		let mut message3 = message.clone();
		let mut die3 = die.clone();
		let mut die4 = die.clone();
		let mut csign3 = csign2.clone();
		let mut bcast3 = bcast2.clone();
		

			s.pop_layer();
			s.add_layer(
				Panel::new(
					LinearLayout::new(Orientation::Horizontal)
					.child(
						Panel::new(
							BoxView::new(AtLeast(50), AtLeast(50),
							LinearLayout::new(Orientation::Vertical)
								.child(
									TextView::new(format!("Current Message: {}", *messaget.lock().unwrap())).with_id("current_message"),
								).child(
									Dialog::new()
										.title("Message Input Field")
										.padding((1,1,1,0))
										.content(
											EditView::new()
											.with_id("message"),
										).button("Change", move |s| {
											let new_message = s.call_on_id(
												"message",
												|view: &mut EditView| view.get_content(),
											).unwrap();
											s.call_on_id(
												"current_message",
												|view: &mut TextView| view.set_content(format!("Current Message: {}", new_message))
											).unwrap();
											//*need_clone.lock().unwrap()=true;
											let mut message3 = message3.lock().unwrap();
											let mut csign3 = csign3.lock().unwrap();
											let mut bcast3 = bcast3.lock().unwrap();
											*message3 = format!("=[{} from {}]= \n =Begin Message= {} =End Message=.", (csign3), (bcast3), new_message);
											(*messaget2.lock().unwrap()) = format!("=[{} from {}]= \n =Begin Message= {} =End Message=.", (csign3), (bcast3), new_message);
											if (*new_message) == "quit"{
												s.quit();
												let mut die3 = die3.lock().unwrap();
												*die3 = true;
											}
										})
						))).title("Message Setup"),
					).child(
						BoxView::new(AtLeast(50), AtLeast(25),
							Panel::new(
								TextView::new("Doesn't work")
							).title("Broadcast Log"),
						)
					),
				).title(format!("Broadcasting as {} from {}", *csign.lock().unwrap(), *bcast.lock().unwrap()))
			);
			s.run();
		}
	
