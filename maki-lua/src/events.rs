//! Events the host fires into plugin autocmds. [`HostEvent`] is the single
//! source of truth: `fire_autocmd` only accepts it, so a new event cannot be
//! fired without landing in this table, and the docs render straight from it.

macro_rules! host_events {
    ($($variant:ident: $when:literal, $extra:literal;)+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum HostEvent {
            $($variant,)+
        }

        impl HostEvent {
            pub const ALL: &[Self] = &[$(Self::$variant),+];

            pub const fn name(self) -> &'static str {
                match self { $(Self::$variant => stringify!($variant),)+ }
            }

            /// When the host fires this event, for the docs table.
            pub const fn fires_when(self) -> &'static str {
                match self { $(Self::$variant => $when,)+ }
            }

            /// Payload fields beyond `data.session_id`, for the docs table.
            pub const fn extra_fields(self) -> &'static str {
                match self { $(Self::$variant => $extra,)+ }
            }
        }
    };
}

host_events! {
    TurnStart: "a user message starts an agent turn", "";
    TurnEnd: "the agent finishes its turn", "";
    TurnError: "the turn fails", "`message`";
    ToolStart: "a tool call begins", "`tool_id`, `tool`";
    ToolDone: "a tool call finishes", "`tool_id`, `tool`";
    SessionReset: "the session is cleared; `session_id` names the session left behind", "";
    SessionFocusChanged: "focus moves to another session", "`previous_session_id` (absent on initial startup)";
    SessionStatusChanged: "a session moves between `working`, `needs_input`, and `idle`", "`status`, `title`, `focused`";
}
