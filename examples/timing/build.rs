

fn main() {
    let my_generator = Box::new(rums::Generator{});

    prost_build::Config::new()
        .out_dir("generated/proto")
        .service_generator(my_generator)
        .compile_protos(&["src/hello.proto"], &["src/"]).unwrap();
}