use std::{fs, path::PathBuf};

fn main() {
	// TODO: Use environment variables to find the LLVM build directory
	let install_directory = PathBuf::from(
		std::env::var("LLVM_SYS_PREFIX").unwrap_or("/usr/local".to_string()),
	);

	let bindings = bindgen::Builder::default()
		.use_core()
		.layout_tests(false)
		.clang_arg("-std=c2x")
		.clang_arg(&format!(
			"-I{}/include",
			install_directory.to_str().unwrap()
		));

	let lib_directory = PathBuf::from(install_directory).join("lib");
	let library_prefixes = ["MLIR", "lld", "LLVM", "clang"];

	for prefix in library_prefixes {
		for entry in
			fs::read_dir(lib_directory.canonicalize().unwrap()).unwrap()
		{
			let path = entry.unwrap().path();
			let filename = path.file_name().unwrap().to_str().unwrap();

			if filename.starts_with(&format!("lib{}", prefix))
				&& filename.ends_with(".a")
			{
				println!(
					"cargo:rustc-link-lib={}",
					filename.trim_start_matches("lib").trim_end_matches(".a")
				);
			}
		}
	}

	// LLVM
	println!(
		"cargo:rustc-link-search={}",
		lib_directory.to_str().unwrap()
	);
	println!("cargo:rustc-link-lib=LLVMCore");
	println!("cargo:rustc-link-lib=LLVMSupport");
	println!("cargo:rustc-link-lib=LLVMObject");
	println!("cargo:rustc-link-lib=LLVMMC");
	println!("cargo:rustc-link-lib=LLVMBPFAsmParser");
	println!("cargo:rustc-link-lib=LLVMAVRCodeGen");
	println!("cargo:rustc-link-lib=LLVMSelectionDAG");

	// LLD
	println!("cargo:rustc-link-lib=lldCommon");
	println!("cargo:rustc-link-lib=lldELF");
	println!("cargo:rustc-link-lib=lldCOFF");
	println!("cargo:rustc-link-lib=lldMinGW");
	println!("cargo:rustc-link-lib=lldWasm");
	println!("cargo:rustc-link-lib=lldMachO");

	// C++ standard library
	println!(
		"cargo:rustc-link-lib={}",
		std::env::var("CXXSTDLIB").unwrap_or("stdc++".to_string())
	);
	println!("cargo:rustc-link-lib=zstd");
	println!("cargo:rustc-link-lib=z");

	cc::Build::new().file("src/wrapper.cc").compile("wrapper");
	println!("cargo:rustc-link-lib=wrapper");

	let bindings = bindings
		.header("src/wrapper.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.allowlist_function("LLDELFLink")
		.generate()
		.expect("Unable to generate bindings");

	let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
