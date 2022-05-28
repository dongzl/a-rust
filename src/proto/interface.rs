use std::error::Error;
use crate::config::Filter;

pub trait FilterFactory {
    fn new_filter(&self, config: String) -> Result<Filter, Box<dyn Error>>;
}

pub trait Listener {
    fn set_executor(executor: Box<dyn Executor>);

    fn listen();

    fn close();
}

// Executor
pub trait Executor {
    // TODO
}