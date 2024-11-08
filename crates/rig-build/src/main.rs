use llvm_sys::LLDELFLink;
use std::ffi::{c_char, CString};

fn rig_link_elf(new_args: &[String]) {
	unsafe {
		let c_args: Vec<CString> = new_args
			.into_iter()
			.map(|s| {
				CString::new(s.clone()).expect("Failed to convert to C string")
			})
			.collect();

		let mut raw_args: Vec<*const c_char> =
			c_args.iter().map(|s| s.as_ptr()).collect();

		if !LLDELFLink(raw_args.as_mut_ptr(), raw_args.len()) {
			std::process::exit(1);
		}
	}
}

fn main() {
	let mut args = std::env::args();

	let arg0 = args.next().unwrap();
	let arg0 = arg0.as_str();

	if arg0.ends_with("ld.lld") {
		let new_args: Vec<String> = args.collect();

		rig_link_elf(&new_args);
	} else if arg0.ends_with("rig") {
		let arg1 = args.next().unwrap();
		let arg1 = arg1.as_str();

		let mut new_args: Vec<String> = vec![];

		new_args.push(arg1.to_string());
		new_args.append(&mut args.collect());

		match arg1 {
			"ld.lld" => {
				rig_link_elf(&new_args);
			}
			cmd => {
				panic!("unknown command: {cmd}");
			}
		}
	} else {
		panic!("unknown command: {arg0}");
	}
}
