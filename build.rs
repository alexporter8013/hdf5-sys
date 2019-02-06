use std::env;
use std::path::Path;

fn main() {
	let lib_var = env::var("HDF5_LIB").expect("Could not find HDF5_LIB");
	let lib_path = Path::new(&lib_var);

	let inc_var = env::var("HDF5_INCLUDE").expect("Could not find HDF5_INCLUDE");
	let inc_path = Path::new(&inc_var);
	println!("cargo:include={:?}", inc_path);
	println!("cargo:rustc-link-search=native={:?}", lib_path);
	println!("cargo:rustc-link-search=native={:?}", inc_path);
}