use std::error::Error;
use mysql_common::frunk::labelled::chars::c;
use crate::boot::discovery::{Discovery, DiscoveryProvider};

pub fn bootstrap(mut provider: DiscoveryProvider) -> Option<Box<Error>>{
    let init = provider.init();
    if init.is_some() {
        return init;
    }
    let clusters = provider.list_clusters();
    let clusters = match clusters {
        Ok(cluster) => cluster,
        Err(err) => return Some(err),
    };

    for cluster in clusters {
        let cluster = provider.cluster(cluster);
        if cluster.err().is_some() {
            continue;
        }
    }

   todo!()
}

fn build_namespace() {
    todo!()
}