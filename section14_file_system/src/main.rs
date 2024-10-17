mod modules;

fn main() {
    println!("Hello, world! section14");

    modules::command::run();
    modules::io::run();
    modules::file_reader::run();
    modules::file_writer::run();
}
