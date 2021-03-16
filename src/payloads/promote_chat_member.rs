// This file is auto generated by `cg` <https://github.com/teloxide/cg> (24572cd + local changes).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, True};

impl_payload! {
    /// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Pass _False_ for all boolean parameters to demote a user. Returns _True_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub PromoteChatMember (PromoteChatMemberSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Unique identifier of the target user
            pub user_id: i64,
        }
        optional {
            /// Pass True, if the administrator's presence in the chat is hidden
            pub is_anonymous: bool,
            /// Pass True, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
            pub can_manage_chat: bool,
            /// Pass True, if the administrator can change chat title, photo and other settings
            pub can_change_info: bool,
            /// Pass True, if the administrator can create channel posts, channels only
            pub can_post_messages: bool,
            /// Pass True, if the administrator can edit messages of other users and can pin messages, channels only
            pub can_edit_messages: bool,
            /// Pass True, if the administrator can delete messages of other users
            pub can_delete_messages: bool,
            /// Pass True, if the administrator can manage voice chats, supergroups only
            pub can_manage_voice_chats: bool,
            /// Pass True, if the administrator can invite new users to the chat
            pub can_invite_users: bool,
            /// Pass True, if the administrator can restrict, ban or unban chat members
            pub can_restrict_members: bool,
            /// Pass True, if the administrator can pin messages, supergroups only
            pub can_pin_messages: bool,
            /// Pass True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
            pub can_promote_members: bool,
        }
    }
}
