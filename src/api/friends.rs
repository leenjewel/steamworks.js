use napi_derive::napi;

#[napi]
pub mod friends {
    use napi::bindgen_prelude::BigInt;
    use steamworks::SteamId;

    #[napi]
    pub enum PersonaState {
        Offline = 0,
        Online = 1,
        Busy = 2,
        Away = 3,
        Snooze = 4,
        LookingToTrade = 5,
        LookingToPlay = 6,
        Invisible = 7,
    }

    #[napi(object)]
    pub struct FriendGameInfo {
        pub game_id: BigInt,
        pub game_ip: String,
        pub game_port: u32,
        pub query_port: u32,
        pub lobby_id: BigInt,
    }

    #[napi(object)]
    pub struct Friend {
        pub steam_id: BigInt,
        pub name: String,
        pub nickname: Option<String>,
        pub state: PersonaState,
        pub game_played: Option<FriendGameInfo>,
    }

    #[napi]
    pub enum FriendFlags {
        None = 0x00,
        Blocked = 0x01,
        FriendshipRequested = 0x02,
        Immediate = 0x04,
        ClanMember = 0x08,
        OnGameServer = 0x10,
        RequestingFriendship = 0x80,
        RequestingInfo = 0x100,
        All = 0xFFFF,
    }

    /// Get an array of friends matching the provided flags.
    ///
    /// @param flags - The flags to filter friends by.
    /// @returns An array of friend objects containing steamId, name, nickname, state and gamePlayed.
    ///
    /// @example
    /// ```js
    /// const friends = client.friends.getFriends(client.friends.FriendFlags.Immediate)
    /// console.log(friends)
    /// ```
    #[napi]
    pub fn get_friends(flags: i32) -> Vec<Friend> {
        let client = crate::client::get_client();
        let flags = steamworks::FriendFlags::from_bits_truncate(flags as u16);
        client
            .friends()
            .get_friends(flags)
            .into_iter()
            .map(|f| Friend {
                steam_id: BigInt::from(f.id().raw()),
                name: f.name(),
                nickname: f.nick_name(),
                state: match f.state() {
                    steamworks::FriendState::Offline => PersonaState::Offline,
                    steamworks::FriendState::Online => PersonaState::Online,
                    steamworks::FriendState::Busy => PersonaState::Busy,
                    steamworks::FriendState::Away => PersonaState::Away,
                    steamworks::FriendState::Snooze => PersonaState::Snooze,
                    steamworks::FriendState::LookingToTrade => PersonaState::LookingToTrade,
                    steamworks::FriendState::LookingToPlay => PersonaState::LookingToPlay,
                    steamworks::FriendState::Invisible => PersonaState::Invisible,
                },
                game_played: f.game_played().map(|g| FriendGameInfo {
                    game_id: BigInt::from(g.game.raw()),
                    game_ip: g.game_address.to_string(),
                    game_port: g.game_port as u32,
                    query_port: g.query_port as u32,
                    lobby_id: BigInt::from(g.lobby.raw()),
                }),
            })
            .collect()
    }

    /// Get the name of a friend.
    ///
    /// @param steam_id64 - The Steam ID of the friend.
    /// @returns The name of the friend.
    ///
    /// @example
    /// ```js
    /// const name = client.friends.getFriendName(76561197985341433n)
    /// console.log(name)
    /// ```
    #[napi]
    pub fn get_friend_name(steam_id64: BigInt) -> String {
        let client = crate::client::get_client();
        client
            .friends()
            .get_friend(SteamId::from_raw(steam_id64.get_u64().1))
            .name()
    }
}
