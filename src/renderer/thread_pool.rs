use std::{
    thread, thread::JoinHandle,
    sync::{ Arc, Mutex, mpsc::{ channel, Sender }},
};

// 匿名函数类型
type Workfn = Box<dyn FnOnce() -> () + Send + 'static>;
// 区分工作和停机状态
enum Msg
{
    Work(Workfn),
    Down
}
// 使用Msg命名空间
use Msg::*;

// 主构造函数
pub struct ThreadPool
{
    //线程数量
    count: usize,
    // 异步发送器
    sender: Sender<Msg>,
    // 带有 原子指针 异步接收器的线程列表  Option<Vec<JoinHandle<()>>>
    threads: Option<Vec<JoinHandle<()>>>
}

impl ThreadPool
{
    // 初始化函数
    pub fn new(count: usize) -> ThreadPool
    {
        assert!(count > 0);
        let mut threads = Vec::with_capacity(count);
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for _ in 0..count
        {
            let p_rec = Arc::clone(&receiver);
            threads.push(thread::spawn(move || loop {
                let f: Msg = p_rec.lock().unwrap().recv().unwrap();
                match f {
                    Work(f) => { f(); /*println!("{} works", i)*/ },
                    Down => { /*println!("{} down", i);*/ break }
                };
            }));
        }

        ThreadPool{ count, sender, threads: Some(threads)}
    }

    // 实例的exec方法
    pub fn exec(&self, f: Workfn)
    {
        self.sender.send(Work(Box::new(f))).unwrap();
    }

}

// Concur实例生命结束时会由rust运行drop（）
impl Drop for ThreadPool
{
    fn drop(&mut self) {
        // 发送停机消息
        for _ in 0..self.count
        {
            self.sender.send(Down).unwrap();
        }

        // 等待所有线程运行完毕
        for thread in self.threads.take().unwrap()
        {
            thread.join().unwrap();
        }
    }
}

#[cfg(test)]
mod test
{
    use std::fmt::Debug;
    use super::*;
    #[test]
    fn test_thread_pool()
    {
        let data = Arc::new(Mutex::new(vec![0; 480]));

        let mut thread_pool = ThreadPool::new(6);
        for i in 0..480
        {
            let local_array = Arc::clone(&data);
            thread_pool.exec(Box::new(move || {
                let mut array = local_array.lock().unwrap();
                array[i] = i.clone();
            }))
        }

        println!("first: {:?}", data.lock().unwrap());

        drop(thread_pool);

        println!("second: {:?}", data.lock().unwrap());
    }

}