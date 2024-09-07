#[derive(Debug, Clone)]
pub struct ThreadsInfo {
    pub threads: Vec<Thread>
}

#[derive(Debug, Clone)]
pub struct Thread {
    pub id: u64
}

