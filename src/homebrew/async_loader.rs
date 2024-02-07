use async_trait::async_trait;
use std::sync::mpsc::{channel, Receiver};

#[async_trait]
pub trait Load: Send {
    fn spawn() -> Self;
    async fn load(&mut self);
}

pub struct AsyncLoader<T>
where
    T: Load,
{
    rx: Receiver<T>,
}

impl<T> AsyncLoader<T>
where
    T: Load + 'static,
{
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let futu = async move {
            let mut loader = T::spawn();
            loader.load().await;
            tx.send(loader).unwrap();
        };
        tokio::spawn(futu);
        Self { rx }
    }
}

impl<T> AsyncLoader<T>
where
    T: Load,
{
    pub fn get(&self) -> Option<T> {
        self.rx.try_recv().ok()
    }
}
