use std::sync::mpsc::{channel, Receiver, TryRecvError};

use super::packagelist::PackageList;

pub struct PackageListLoader {
    rx: Receiver<PackageList>,
}

impl PackageListLoader {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        tokio::spawn(async move {
            let mut package_list = PackageList::new();
            package_list.load_all().await;
            tx.send(package_list).unwrap();
        });
        Self { rx }
    }

    pub fn try_recv(&self) -> Result<PackageList, TryRecvError> {
        self.rx.try_recv()
    }
}
