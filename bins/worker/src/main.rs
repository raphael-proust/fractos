use messages::Answer;
use messages::Task;

use clap::Parser;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = 4242)]
    port: u16,

    /// Number of times to greet
    #[arg(short, long, default_value = "localhost")]
    addr: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let port = args.port.to_string();

    let addr = format!("{}:{}", args.addr, port);
    println!("try to connect to {}", addr);

    let mut stream = TcpStream::connect(addr)?;

    let mut binary_size = [0_u8; 8];
    stream.read_exact(&mut binary_size)?;
    let size = usize::from_be_bytes(binary_size);

    let mut msg = vec![0_u8; size];
    stream.read_exact(&mut msg)?;
    let task: Task = serde_json::from_slice(&msg).unwrap();
    use worker::handle_task;

    let answer: Answer = handle_task(task);
    let serialized_answer = serde_json::to_vec(&answer).unwrap();

    let _ = stream.write(&serialized_answer);
    Ok(())
}
