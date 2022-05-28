use crate::proto::interface::Executor;
use std::collections::HashMap;

pub struct ServerConfig {
    pub server_version: String,
}

pub struct Listener<T>
where
    T: Executor,
{
    config: ServerConfig,
    listener: String, // TODO
    executor: T,
    conn_id: u32,
    conn_read_buffer_size: u16,
    capabilities: u32,
    character_set: u8,
    schema_name: String,
    statement_id: u32,
    stmts: HashMap<String, String>,
}

impl<T: Executor> crate::proto::interface::Listener for Listener<T> {
    fn set_executor(executor: Box<dyn Executor>) {
        todo!()
    }

    fn listen() {
        todo!()
    }

    fn close() {
        todo!()
    }
}

impl<T: Executor> Listener<T> {
    pub fn new(executor: T, config: crate::config::Listener) -> Self {
        todo!()
    }
}
