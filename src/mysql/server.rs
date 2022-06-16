use crate::proto::interface::Executor;
use std::collections::HashMap;
use crate::proto::interface;

pub struct ServerConfig {
    pub server_version: String,
}

pub struct Listener {
    pub config: ServerConfig,
    pub listener: String,
    pub conn_id: i32,
    pub conn_read_buffer_size: i32,
    pub capabilities: i32,
    pub character_set: i32,
    pub schema_name: String,
    pub statement_id: i32,
}
// {
//     config: ServerConfig,
//     listener: String, // TODO
//     // executor: T,
//     conn_id: u32,
//     conn_read_buffer_size: u16,
//     capabilities: u32,
//     character_set: u8,
//     schema_name: String,
//     statement_id: u32,
//     stmts: HashMap<String, String>,
// }

impl interface::Listener for Listener {
    fn set_executor(&self, executor: Box<dyn Executor>) {
        todo!()
    }

    fn listen(&self) {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}

impl Listener {
    pub fn new(config: crate::config::Listener) -> Self {
        let config = ServerConfig {
            server_version: config.server_version,
        };

        //TODO Listener

        Listener {
            config,
            listener: "".to_string(),
            conn_id: 0,
            conn_read_buffer_size: 0,
            capabilities: 0,
            character_set: 0,
            schema_name: "".to_string(),
            statement_id: 0
        }
    }
}
