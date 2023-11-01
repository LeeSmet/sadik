use std::{net::SocketAddr, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(long)]
    /// The unique ID of the node.
    node_id: u8,

    #[arg(long)]
    /// The seed nodes of the cluster.
    ///
    /// These are used to auto discover other peers in the system.
    seeds: Vec<String>,

    #[arg(long, default_value = "[::]:8001")]
    /// The address to use to communicate with other nodes in the cluster.
    cluster_listen_addr: SocketAddr,

    #[arg(long)]
    /// The public address of the node.
    ///
    /// This is communicated to other nodes, so they are able to connect back to this node. This
    /// address should be set if the cluster address is not a public address (e.g. in case a
    /// reverse proxy is used).
    public_addr: Option<SocketAddr>,

    #[arg(long)]
    /// The path to the storage directory where the replicated metadata will be stored.
    data_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();

    let _node = sadik::Node::new(
        args.node_id,
        &args.data_dir,
        args.cluster_listen_addr,
        args.public_addr.unwrap_or(args.cluster_listen_addr),
        &args.seeds,
    )
    .await?;

    Ok(())
}

