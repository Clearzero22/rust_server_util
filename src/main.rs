use rust_server_util::ThreadPool;
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
