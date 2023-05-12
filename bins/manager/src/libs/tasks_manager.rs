use messages;
use std::collections::LinkedList;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

pub const WELCOME_PORT: u16 = 4242;

#[derive(Debug)]
pub struct NoWorker {}

impl std::fmt::Display for NoWorker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There is no worker available.")
    }
}

impl std::error::Error for NoWorker {}

#[derive(Debug)]
pub struct FinishedTask {
    pub task: messages::Task,
    pub result: messages::Answer,
}

pub struct TasksManager {
    new_task_receiver: Arc<RwLock<Receiver<messages::Task>>>,
    new_task_sender: Sender<messages::Task>,
    stop_lock: Arc<RwLock<bool>>,
    threads: Arc<RwLock<Vec<JoinHandle<()>>>>,
}

impl Drop for TasksManager {
    fn drop(&mut self) {
        //let mut stop_lock = self.stop_lock.write().unwrap();
        //*stop_lock = true;

        // TODO join threads
        //for _th in self.threads.read() {
        //    todo!()
        //}
    }
}

impl TasksManager {
    pub async fn new() -> (Self, Receiver<FinishedTask>) {
        let (finished_task_sender, finished_task_receiver) = channel(4242);
        let (new_task_sender, new_task_receiver) = channel(4242);
        let tasks_manager = TasksManager {
            new_task_receiver: Arc::new(RwLock::new(new_task_receiver)),
            new_task_sender,
            stop_lock: Arc::new(RwLock::new(false)),
            threads: Arc::new(RwLock::new(Vec::new())),
        };

        welcome(
            tasks_manager.new_task_receiver.clone(),
            finished_task_sender,
            tasks_manager.stop_lock.clone(),
            tasks_manager.threads.clone(),
        );

        (tasks_manager, finished_task_receiver)
    }

    pub async fn push(&mut self, task: messages::Task) {
        self.new_task_sender.send(task).await;
    }
}

fn welcome(
    new_task_reciever: Arc<RwLock<Receiver<messages::Task>>>,
    finished_task_sender: Sender<FinishedTask>,
    stop_lock: Arc<RwLock<bool>>,
    threads: Arc<RwLock<Vec<JoinHandle<()>>>>,
) {
    let thread = tokio::spawn(async move {
        // TODO addresse parametrable
        let listener = TcpListener::bind((std::net::Ipv6Addr::LOCALHOST, WELCOME_PORT))
            .await
            .unwrap();

        // while !(stop_lock.read().unwrap()) {
        loop {
            let (conn, _) = listener.accept().await.unwrap();
            let new_task_receiver = new_task_reciever.clone();
            let finished_task_sender = finished_task_sender.clone();
            let stop_lock = stop_lock.clone();

            let _conn_thread = tokio::task::spawn(async move {
                handle_new_connection(conn, new_task_receiver, finished_task_sender, stop_lock)
                    .await;
            });
            // TODO register thread for drop
            //threads.write().unwrap().push(connThread);
        }
    });
    //threads.write().unwrap().push(thread);
}

async fn handle_new_connection(
    mut conn: TcpStream,
    new_task_receiver: Arc<RwLock<Receiver<messages::Task>>>,
    finished_task_sender: Sender<FinishedTask>,
    stop_lock: Arc<RwLock<bool>>,
) {
    loop {
        let task = new_task_receiver.write().await.recv().await.unwrap();
        send_task(&mut conn, task.clone()).await;
        let answer = read_answer(&mut conn).await;
        let finished_task = FinishedTask {
            result: answer,
            task,
        };
        finished_task_sender.send(finished_task).await.unwrap();
    }
}

pub async fn send_task(stream: &mut TcpStream, task: messages::Task) {
    println!("Send task");

    let serialized_task = serde_json::to_vec(&task).unwrap();

    let serialized_task_size = serialized_task.len() as u32;
    println!("Serialized size : {}", serialized_task_size);
    stream
        .write_all(&serialized_task_size.to_be_bytes())
        .await
        .unwrap();

    // Send serialized task
    stream.write_all(&serialized_task).await.unwrap();
}

pub async fn read_answer(stream: &mut TcpStream) -> messages::Answer {
    println!("Read answer");
    let mut binary_size = [0_u8; 4];
    stream.read_exact(&mut binary_size).await.unwrap();
    let size = u32::from_be_bytes(binary_size);
    println!("Will read:{} bytes", size);

    // Recover the message of size from the stream
    let mut msg = vec![0_u8; size as usize];
    stream.read_exact(&mut msg).await.unwrap();
    println!("Answer readed");

    // Deserialize the message into an Answer
    let answer: messages::Answer = serde_json::from_slice(&msg).unwrap();

    answer
}
