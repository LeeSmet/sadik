use std::{net::SocketAddr, path::Path};

use datacake::{
    eventual_consistency::{EventuallyConsistentStoreExtension, ReplicatedStoreHandle},
    lmdb::LmdbStorage,
    node::{ConnectionConfig, DCAwareSelector, DatacakeNodeBuilder},
};

/// The node is the main entity in the system. It handles everything from serving the frontend
/// API's to communicating with peers and the storage system.
pub struct Node {
    store_handle: ReplicatedStoreHandle<LmdbStorage>,
}

impl Node {
    /// Setup and start a new node, causing it to connect to its configured peers and exchange
    /// metadata storage information.
    pub async fn new(
        node_id: u8,
        metastore_path: &Path,
        cluster_listen_addr: SocketAddr,
        public_addr: SocketAddr,
        seeds: &[String],
    ) -> Result<Self, String> {
        let connection_cfg = ConnectionConfig::new(cluster_listen_addr, public_addr, seeds);
        let node = DatacakeNodeBuilder::<DCAwareSelector>::new(node_id, connection_cfg)
            .connect()
            .await
            .map_err(|e| e.to_string())?;
        let storage = LmdbStorage::open(metastore_path)
            .await
            .map_err(|e| e.to_string())?;
        let store = node
            .add_extension(EventuallyConsistentStoreExtension::new(storage))
            .await
            .map_err(|e| e.to_string())?;
        let store_handle = store.handle();

        Ok(Node { store_handle })
    }
}
