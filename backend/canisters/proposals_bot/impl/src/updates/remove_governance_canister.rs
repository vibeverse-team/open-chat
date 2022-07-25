use crate::guards::caller_is_service_owner;
use crate::{mutate_state, read_state};
use canister_tracing_macros::trace;
use ic_cdk_macros::update;
use proposals_bot_canister::remove_governance_canister::{Response::*, *};

#[update(guard = "caller_is_service_owner")]
#[trace]
async fn remove_governance_canister(args: Args) -> Response {
    if let Some(chat_id) = read_state(|state| state.data.nervous_systems.get_chat_id(&args.governance_canister_id)) {
        if args.delete_group {
            let delete_group_args = group_canister::c2c_delete_group::Args {};
            if let Err(error) = group_canister_c2c_client::c2c_delete_group(chat_id.into(), &delete_group_args).await {
                return InternalError(format!("{:?}", error));
            }
        }

        mutate_state(|state| state.data.nervous_systems.remove(&args.governance_canister_id));
        Success
    } else {
        NotFound
    }
}