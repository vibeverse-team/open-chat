use crate::{read_state, RuntimeState};
use group_canister::selected_updates_v2::{Response::*, *};
use ic_cdk_macros::query;

#[query]
fn selected_updates_v2(args: Args) -> Response {
    read_state(|state| selected_updates_impl(args, state))
}

fn selected_updates_impl(args: Args, state: &RuntimeState) -> Response {
    // Short circuit prior to calling `ic0.caller()` to maximise query caching.
    let latest_event_timestamp = state.data.chat.events.latest_event_timestamp().unwrap_or_default();
    if latest_event_timestamp <= args.updates_since {
        return SuccessNoUpdates(latest_event_timestamp);
    }

    let caller = state.env.caller();
    let user_id = match state.data.lookup_user_id(caller) {
        Some(id) => id,
        None => return CallerNotInGroup,
    };

    let updates = state
        .data
        .chat
        .selected_group_updates(args.updates_since, Some(user_id))
        .unwrap();

    if updates.has_updates() {
        Success(updates)
    } else {
        SuccessNoUpdates(latest_event_timestamp)
    }
}
