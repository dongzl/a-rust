use crate::boot::discovery::{Discovery, DiscoveryProvider};
use crate::boot::boot;
use crate::config::Filter;
use crate::proto::interface::FilterFactory;
use crate::server::server::Server;

pub fn run(config_path: String) {
    //TODO default config.
    let provider = DiscoveryProvider::new(config_path);
    let bootstrap = boot::bootstrap(provider);
    if bootstrap.is_some() {
        // TODO log
        return;
    }
    let filters = provider.list_filters();
    if filters.is_err() {
        // TODO log
        return;
    }
    for each in filters.unwrap() {
        let factory = each.factory(each.name);
        if factory.is_none() {
            panic!()
        }
        let factory = factory.unwrap();
        let filter: Filter = match factory.new_filter(each.config) {
            Ok(filter) => filter,
            Err(err) => panic!(),
        };
        filter.register(filter.name);
    }

    let server = Server::new();

    let listeners = match provider.list_listeners() {
        Ok(listeners) => listeners,
        Err(err) => {
            // TODO log
            return;
        },
    };

    for listener in listeners {

    }

    server.start();
}