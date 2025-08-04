mod abi;
mod pb;
use hex_literal::hex;
use pb::invoice_contract::v1::{RemittanceEvent, RemittanceEvents};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
#[allow(unused_imports)]
use std::str::FromStr;
#[allow(unused_imports)]
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const INVOICE_TRACKED_CONTRACT: [u8; 20] = hex!("1f98431c8ad98523631ae4a59f267346ea31f984");

#[substreams::handlers::map]
fn map_invoice_contract_events(
    blk: eth::Block,
) -> Result<RemittanceEvents, substreams::errors::Error> {
    let mut remittance_events = Vec::new();

    remittance_events.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == INVOICE_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::invoice_contract::events::Remittance::match_and_decode(log)
                        {
                            return Some(RemittanceEvent {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                                asset: event.asset,
                                invoice_id: event.invoice_id.to_string(),
                                payee: event.payee,
                                payer: event.payer,
                            });
                        }
                        None
                    })
            })
            .collect(),
    );

    Ok(RemittanceEvents {
        events: remittance_events,
    })
}
