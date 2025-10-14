#[allow(clippy::unnecessary_wraps)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let hello_proto_buf = std::path::PathBuf::from("../proto/hello.proto");
	let proto = std::path::PathBuf::from("../proto");

	let output_dir = std::path::PathBuf::from("output");
	std::fs::create_dir_all(&output_dir).expect("Failed to create output dir");

	tonic_prost_build::configure()
		.build_client(false)
		.out_dir(output_dir)
		.compile_protos(&[hello_proto_buf], &[proto])?;

	Ok(())
}
