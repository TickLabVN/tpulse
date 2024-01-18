use ipipe::Pipe;
use std::io::BufRead;

pub fn handle_metrics() {
    let mut pipe =
        Pipe::with_name("tpulse_pipe").expect("Could not create named pipe for tpulse application");
    for line in std::io::BufReader::new(pipe).lines() {
        println!("{}", line.unwrap())
    }
}
