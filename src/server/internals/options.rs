pub struct ServerOptions {
    host: String,
    port: u32,
    server_threads: usize,
    worker_threads: usize
}

impl ServerOptions {
    pub fn new(host: String, port: u32, server_threads: usize, worker_threads: usize) -> Self {
        Self {
            host,
            port,
            server_threads,
            worker_threads
        }
    }

    pub fn get_host(&self) -> String {
        self.host.clone()
    }

    pub fn get_port(&self) -> u32 {
        self.port
    }

    pub fn get_server_threads(&self) -> usize {
        self.server_threads
    }

    pub fn get_worker_threads(&self) -> usize {
        self.worker_threads
    }
}

impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            host: String::from("localhost"),
            port: 3000,
            server_threads: 1,
            worker_threads: 2
        }
    }
}
