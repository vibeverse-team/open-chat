use crate::lifecycle::UPGRADE_BUFFER_SIZE;
use crate::memory::get_upgrades_memory;
use crate::{take_state, LOG_MESSAGES};
use canister_tracing_macros::trace;
use ic_cdk_macros::pre_upgrade;
use ic_stable_structures::writer::{BufferedWriter, Writer};
use tracing::info;

#[pre_upgrade]
#[trace]
fn pre_upgrade() {
    info!("Pre-upgrade starting");

    let state = take_state();

    let messages_container = LOG_MESSAGES.with(|l| l.take());
    let log_messages = messages_container.logs.drain_messages();
    let trace_messages = messages_container.traces.drain_messages();

    let cycles_dispenser_client_state = cycles_dispenser_client::serialize_to_bytes();

    let stable_state = (state.data, log_messages, trace_messages, cycles_dispenser_client_state);

    let mut memory = get_upgrades_memory();
    let writer = BufferedWriter::new(UPGRADE_BUFFER_SIZE, Writer::new(&mut memory, 0));

    serializer::serialize(stable_state, writer).unwrap();
}