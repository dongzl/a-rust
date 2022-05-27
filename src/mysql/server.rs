use std::collections::HashMap;
use crate::proto::interface::Executor;

pub struct ServerConfig {
    pub server_version: String,
}

pub struct Listener<T> where T: Executor {
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