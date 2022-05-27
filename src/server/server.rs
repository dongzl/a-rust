use crate::proto::interface::Listener;

pub struct Server<T> where T: Listener {
    pub listeners: Vec<T>,
}

impl <T: Listener>Server<T>  {
    pub fn new() -> Self {
        return Server {
            listeners: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: T) {
        &self.listeners.push(listener);
    }

    pub fn start(&self) {
        for each in &self.listeners {
            //TODO start listener
        }
    }
}