use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{
    AccessGate, ChannelId, Document, FieldTooLongResult, FieldTooShortResult, GroupPermissions, GroupPermissionsPrevious,
    GroupSubtype, Milliseconds, Rules,
};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub rules: Rules,
    pub subtype: Option<GroupSubtype>,
    pub avatar: Option<Document>,
    pub history_visible_to_new_joiners: bool,
    // TODO: remove this after the website starts using permissions_v2
    pub permissions: Option<GroupPermissionsPrevious>,
    #[serde(default)]
    pub permissions_v2: Option<GroupPermissions>,
    pub events_ttl: Option<Milliseconds>,
    pub gate: Option<AccessGate>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    NameTooShort(FieldTooShortResult),
    NameTooLong(FieldTooLongResult),
    NameReserved,
    DescriptionTooLong(FieldTooLongResult),
    RulesTooShort(FieldTooShortResult),
    RulesTooLong(FieldTooLongResult),
    AvatarTooBig(FieldTooLongResult),
    MaxChannelsCreated(u32),
    NameTaken,
    UserSuspended,
    NotAuthorized,
    CommunityFrozen,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub channel_id: ChannelId,
}
