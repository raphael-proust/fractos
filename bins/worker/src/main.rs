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

fn connect(args: Args) -> std::io::Result<TcpStream> {
    let port = args.port.to_string();
    let addr = format!("{}:{}", args.addr, port);
    println!("try to connect to {}", addr);

    let stream = TcpStream::connect(&addr)?;
    println!("Connected to {}", addr);
    Ok(stream)
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut stream = connect(args)?;

    loop {
        // Recover the size for the expected data
        let mut binary_size = [0_u8; 4];
        stream.read_exact(&mut binary_size)?;
        let size = u32::from_be_bytes(binary_size);

        println!("Will read a task of size: {}", size);

        // Recover the message of size from the stream
        let mut msg = vec![0_u8; size as usize];
        stream.read_exact(&mut msg)?;
        println!("Messaged readed");

        // Deserialize the message into a Task
        let task: Task = serde_json::from_slice(&msg)?;

        // Handle the task into an Answer
        println!("Handle task");
        use worker::handle_task;
        let answer: Answer = handle_task(&task);

        // Serialize the answer and send it in the stream
        let serialized_answer = serde_json::to_vec(&answer)?;

        // Send the size of the serialized answer

        let answer_size = serialized_answer.len() as u32;
        println!("Will send an answer of size: {}", answer_size);
        let _ = stream.write(&answer_size.to_be_bytes());

        // Send the serialized answer
        println!("Send the answer");
        let _ = stream.write(&serialized_answer);
    }
}
