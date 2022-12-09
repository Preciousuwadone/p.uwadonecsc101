use std::io;

fn main() {

	fn Pub_service() {

		let mut staff1 = String::new();
		println!("Are you a Public Servant?");
		io::stdin().read_line(&mut staff1).expect("Not valid for promotion");
		let s:char = staff1.trim().parse().expect("Not valid for promotion");
		println!("{}",staff1);


		println!("Are you a lawyer?");
		let mut staff1 = String::new();
		io::stdin().read_line(&mut staff1).expect("Not valid for promotion");
		let s:char = staff1.trim().parse().expect("Not valid for promotion");
		println!("{}",staff1);


		println!("do you have 5-8 years of work experience?");
		let mut staff1 = String::new();
		io::stdin().read_line(&mut staff1).expect("Not valid for promotion");
		let s:char = staff1.trim().parse().expect("Not valid for promotion");
		println!("{}",staff1);


		println!("then you are an associate member.");
		let mut staff1 = String::new();
		io::stdin().read_line(&mut staff1).expect("Not valid for promotion");
		let s:char = staff1.trim().parse().expect("Not valid for promotion");
		println!("{}",staff1);





		


	}
}