use futures::future::join_all;
use reth_rpc_eth_types::{error::PrivateTransactionError, EthApiError};
use revm_primitives::Bytes;
use strum::IntoEnumIterator;
use tracing::{info, warn};

use super::builders::{Builder, BuilderKind};

pub(crate) struct EthPrivateTransaction;

impl EthPrivateTransaction {
    pub(crate) fn builders(&self) -> Vec<Builder> {
        let mut builders = Vec::new();
        for kind in BuilderKind::iter() {
            match kind.builder() {
                Ok(builder) => {
                    info!(target: "builder", "Sending tx to builder: {}", kind);
                    builders.push(builder);
                }
                Err(e) => warn!(target: "builder", "Failed to create builder for {}: {}", kind, e),
            }
        }
        builders
    }

    pub(crate) async fn send_tx_to_builders(&self, tx: Bytes, builders: Vec<Builder>) -> Result<(), EthApiError> {
        let results = join_all(builders.iter().map(|builder| builder.send_tx(tx.clone()))).await;

        if results.iter().all(|r| r.is_err()) {
            return Err(PrivateTransactionError::AllBuildersFailed.into());
        }

        Ok(())
    }
}