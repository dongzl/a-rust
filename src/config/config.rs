use std::collections::HashMap;
use std::error::Error;
use serde::Deserialize;
use crate::config::config_model::Configuration;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigOptions {
    pub store_name: String,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Center {
    initialize: i32,
    // store_operate: StoreOperate,
    // conf_holder: atomic.Value,
    // lock:         sync.RWMutex
    // observers    []Observer
    // watchCancels []context.CancelFunc
}

impl Center {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // TODO
        let center = Center {
            initialize: 0,
        };
        return Ok(center);
    }

    pub fn load(&self) -> Result<Configuration, Box<dyn Error>> {
        todo!()
    }
}