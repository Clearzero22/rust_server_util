use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread
};

use chrono::Utc;

type Job = Box<dyn FnOnce() + Send + 'static>;

// 线程池
struct ThreadPool {
    // 工作线程
    workers: Vec<Worker>,
    // 任务队列
    sender: Option<mpsc::Sender<Job>>,
}

// 工作线程
struct Worker {
    // 工作线程编号
    id: usize,
    // 实例化的线程句柄
    handle: Option<thread::JoinHandle<()>>,
}

// 为ThreadPoll 实现方法

impl ThreadPool {
    fn new(size: usize) -> Self {
        // 线程池的数量需要大于零
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Job>();
        // receiver需要多个线程间共享所有权，使用Arc
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 1..=size {
            let receiver = receiver.clone();
            workers.push(Worker::new(id, receiver));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    //实现execute方法
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// 解决方案 修改Threadpoll的结构体，使得 sender 字段 成为Option
// 为Threadpoll实现Drop trait
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 需要等待所有线程都执行完成
        // 先释放channel
        drop(self.sender.take());
        // 等待线程结束
        for worker in &mut self.workers {
            let now = Utc::now();
            let formatted_time = now.format("%y-%m-%d").to_string();
            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
                println!("Worker {} shutdown in {}", worker.id, formatted_time);
            }
            println!("{}Shutting down worker {}", formatted_time, worker.id);
        }
        // println!("Shutting down all workers");
        let nextnow = Utc::now();
        let formatted_time = nextnow.format("%y-%m-%d").to_string();
        println!("{}Shutting down thread pool", formatted_time);
    }
}

// 为Worker实现方法
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || {
            // 从channel拉取job执行

            // 线程需要一直运行，用loop循环

            loop {
                let job: Result<Box<dyn FnOnce() + Send>, mpsc::RecvError> =
                    receiver.lock().unwrap().recv();

                match job {
                    Ok(job) => {
                        println!("Worker {id} is working...");

                        job();
                    }

                    Err(_) => {
                        println!("Worker {id} disconnected ; exit");

                        break;
                    }
                }
            }
        });

        Worker {
            id,
            handle: Some(handle),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

// 创建一个测试函数test_thread_pool 测试一下创建ThreadPool是否正常

#[cfg(test)]
mod test {
    use crate::ThreadPool;
    #[test]
    fn test_thread_pool() {
        let pool = ThreadPool::new(4);

        let f1 = || {
            let result = 1 + 1;
            println!("result:{result}");
        };

        pool.execute(f1);
    }
}
