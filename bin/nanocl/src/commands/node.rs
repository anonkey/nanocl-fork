use nanocl_error::io::IoResult;

use crate::{
  config::CliConfig,
  models::{NodeArg, NodeCommand, NodeRow},
};

use super::{GenericCommand, GenericCommandLs};

impl GenericCommand for NodeArg {
  fn object_name() -> &'static str {
    "nodes"
  }
}

impl GenericCommandLs for NodeArg {
  type Item = NodeRow;
  type Args = NodeArg;
  type ApiItem = nanocld_client::stubs::node::Node;

  fn get_key(item: &Self::Item) -> String {
    item.name.clone()
  }
}

/// Function that execute when running `nanocl node`
pub async fn exec_node(cli_conf: &CliConfig, args: &NodeArg) -> IoResult<()> {
  let client = &cli_conf.client;
  match &args.command {
    NodeCommand::List(opts) => NodeArg::exec_ls(client, args, opts).await,
  }
}
