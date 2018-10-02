use super::super::HTTPVersion;

pub struct ServerOptions {
    host: String,
    port: u32,
    server_threads: usize,
    worker_threads: usize,
    default_http_version: HTTPVersion
}

impl ServerOptions {
    pub fn new(host: String, port: u32, server_threads: usize, worker_threads: usize, default_http_version: HTTPVersion) -> Self {
        Self {
            host,
            port,
            server_threads,
            worker_threads,
            default_http_version
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

    pub fn get_default_http_version(&self) -> HTTPVersion {
        self.default_http_version.clone()
    }
}

impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            host: String::from("localhost"),
            port: 3000,
            server_threads: 1,
            worker_threads: 2,
            default_http_version: HTTPVersion::HTTP20
        }
    }
}
