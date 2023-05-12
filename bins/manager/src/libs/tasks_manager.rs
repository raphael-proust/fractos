use messages;
use std::collections::LinkedList;
use std::sync::mpsc::{channel, Receiver, Sender};
use tokio::net::{TcpListener, TcpStream};

pub const WELCOME_PORT: u16 = 4242;

#[derive(Debug)]
pub struct NoWorker {}

impl std::fmt::Display for NoWorker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There is no worker available.")
    }
}

impl std::error::Error for NoWorker {}

pub struct FinishedTask {
    pub task: messages::Task,
    pub result: messages::Answer,
}

pub struct TasksManager {
    finished_task_sender: Sender<FinishedTask>,
    new_task_sender: Sender<messages::Task>,
}

impl Drop for TasksManager {
    fn drop(&mut self) {}
}

impl TasksManager {
    #[tokio::main]
    pub async fn new() -> (Self, Receiver<FinishedTask>) {
        let (new_worker_sender, new_worker_receiver) = channel();
        handle_new_connections(new_worker_sender);

        let (new_task_sender, new_task_receiver) = channel();
        handle_new_tasks(new_worker_receiver, new_task_receiver);

        let (finished_task_sender, finished_task_receiver) = channel();
        let tasks = TasksManager {
            finished_task_sender,
            new_task_sender,
        };
        (tasks, finished_task_receiver)
    }

    pub fn push(&mut self, task: messages::Task) {
        self.new_task_sender.send(task);
    }
}

pub fn handle_new_tasks(
    new_worker_receiver: Receiver<TcpStream>,
    new_task_receiver: Receiver<messages::Task>,
) {
    tokio::spawn(async move {
        let mut workers = LinkedList::<TcpStream>::new();

        loop {
            while let Ok(worker) = new_worker_receiver.try_recv() {
                workers.push_front(worker);
            }

            let task = new_task_receiver.recv().unwrap();
            let mut worker = workers.pop_front().unwrap();
            send_task(&mut worker, task);
            workers.push_back(worker);
        }
    });
}

fn handle_new_connections(new_worker_sender: Sender<TcpStream>) {
    tokio::spawn(async move {
        // TODO addresse parametrable
        let listener = TcpListener::bind((std::net::Ipv6Addr::LOCALHOST, WELCOME_PORT))
            .await
            .unwrap();

        loop {
            let (conn, _) = listener.accept().await.unwrap();
            new_worker_sender.send(conn);
        }
    });
}

fn send_task(_conn: &mut TcpStream, _task: messages::Task) {}
