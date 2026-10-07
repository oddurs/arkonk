//! Steamworks behind the opt-in `steam` feature. Without it every call is a no-op,
//! so the game loop calls Steam unconditionally and default builds never link it.
pub use imp::{Steam, restart_through_steam};

#[cfg(feature = "steam")]
mod imp {
    use crate::{
        achievements::{self, Achievement, MEDALS_STAT},
        presence::Presence,
    };
    use ark::{game::Game, profile::Profile};
    use steamworks::{AppId, CallbackResult, Client, SteamAPIInitError};

    /// The one place the Steam app id lives. 480 is Valve's shared Spacewar test
    /// app; replace it with the id from the Steamworks partner site to ship.
    const APP_ID: u32 = 480;

    /// True when Steam is relaunching the game and this process must exit before
    /// opening a window.
    pub fn restart_through_steam() -> bool {
        // Relaunching for app 480 would start Spacewar instead of this build, and
        // debug builds must stay runnable straight from cargo.
        APP_ID != 480
            && !cfg!(debug_assertions)
            && steamworks::restart_app_if_necessary(AppId(APP_ID))
    }

    pub struct Steam(Option<Session>);
    struct Session {
        client: Client,
        stats_ready: bool,
        pending: Vec<Achievement>,
        medals: u32,
        overlay_opened: bool,
        presence: Option<Presence>,
        warned_missing: bool,
    }
    impl Steam {
        pub fn off() -> Self {
            Self(None)
        }
        /// Connects to the running Steam client. Without one the game carries on
        /// alone; progress still saves and unlocks on the next launch with Steam.
        pub fn init(profile: &Profile) -> Self {
            match Client::init_app(APP_ID) {
                Ok(client) => {
                    let mut session = Session {
                        client,
                        stats_ready: false,
                        pending: Vec::new(),
                        medals: 0,
                        overlay_opened: false,
                        presence: None,
                        warned_missing: false,
                    };
                    session.queue(profile, Vec::new());
                    Self(Some(session))
                }
                Err(e) => {
                    let (SteamAPIInitError::FailedGeneric(detail)
                    | SteamAPIInitError::NoSteamClient(detail)
                    | SteamAPIInitError::VersionMismatch(detail)) = &e;
                    crate::diagnostics::error(format_args!(
                        "Steam unavailable, continuing without it: {e} ({detail})"
                    ));
                    Self(None)
                }
            }
        }
        /// Runs once per frame: dispatches callbacks and stores pending unlocks
        /// once Steam has delivered this user's stats.
        pub fn run_callbacks(&mut self) {
            let Some(s) = &mut self.0 else { return };
            let me = s.client.user().steam_id();
            let (mut overlay, mut ready) = (false, false);
            s.client.process_callbacks(|callback| match callback {
                CallbackResult::GameOverlayActivated(o) => overlay |= o.active,
                CallbackResult::UserStatsReceived(r) if r.steam_id == me => match r.result {
                    Ok(()) => ready = true,
                    Err(e) => crate::diagnostics::error(format_args!(
                        "Steam could not load achievements: {e}"
                    )),
                },
                _ => {}
            });
            s.overlay_opened |= overlay;
            s.stats_ready |= ready;
            if s.stats_ready && !s.pending.is_empty() {
                s.store();
            }
        }
        /// Whether the overlay opened since the last call; the game treats it
        /// like losing focus.
        pub fn overlay_opened(&mut self) -> bool {
            self.0
                .as_mut()
                .is_some_and(|s| std::mem::take(&mut s.overlay_opened))
        }
        /// Call where a sector clear has just been recorded in `profile`.
        pub fn cleared(&mut self, game: &Game, profile: &Profile) {
            if let Some(s) = &mut self.0 {
                s.queue(profile, achievements::from_clear(game));
            }
        }
        /// What friends see: menus, or the sector being played.
        pub fn presence(&mut self, playing: bool, game: &Game) {
            let Some(s) = &mut self.0 else { return };
            let presence = Presence::of(playing, game);
            if s.presence == Some(presence) {
                return;
            }
            s.presence = Some(presence);
            let friends = s.client.friends();
            let mut accepted = true;
            if let Some((sector, name)) = presence.sector() {
                accepted &= friends.set_rich_presence("sector", Some(&sector));
                accepted &= friends.set_rich_presence("name", Some(&name));
            }
            accepted &= friends.set_rich_presence("steam_display", Some(presence.token()));
            if !accepted {
                crate::diagnostics::error(format_args!(
                    "Steam rejected rich presence {presence:?}"
                ));
            }
        }
    }
    impl Session {
        fn queue(&mut self, profile: &Profile, extra: Vec<Achievement>) {
            for a in achievements::from_profile(profile).into_iter().chain(extra) {
                if !self.pending.contains(&a) {
                    self.pending.push(a);
                }
            }
            self.medals = profile.medals();
        }
        fn store(&mut self) {
            let stats = self.client.user_stats();
            let mut changed = false;
            let mut missing = Vec::new();
            for a in self.pending.drain(..) {
                let achievement = stats.achievement(a.api_name());
                match achievement.get() {
                    Ok(true) => {}
                    Ok(false) => match achievement.set() {
                        Ok(()) => changed = true,
                        Err(()) => missing.push(a.api_name()),
                    },
                    Err(()) => missing.push(a.api_name()),
                }
            }
            // Never lower the stat: progress from another machine may be ahead.
            let medals = i32::try_from(self.medals).unwrap_or(i32::MAX);
            match stats.get_stat_i32(MEDALS_STAT) {
                Ok(current) if current < medals => {
                    changed |= stats.set_stat_i32(MEDALS_STAT, medals).is_ok();
                }
                Ok(_) => {}
                Err(()) => missing.push(MEDALS_STAT),
            }
            if !missing.is_empty() && !self.warned_missing {
                self.warned_missing = true;
                crate::diagnostics::error(format_args!(
                    "Steam app {APP_ID} does not define {}; see docs/steam/achievements.md",
                    missing.join(", ")
                ));
            }
            if changed && stats.store_stats().is_err() {
                crate::diagnostics::error(format_args!(
                    "Steam did not accept the achievement update"
                ));
            }
        }
    }
}

#[cfg(not(feature = "steam"))]
mod imp {
    use ark::{game::Game, profile::Profile};

    pub fn restart_through_steam() -> bool {
        false
    }
    pub struct Steam;
    impl Steam {
        pub fn off() -> Self {
            Self
        }
        pub fn init(_: &Profile) -> Self {
            Self
        }
        pub fn run_callbacks(&mut self) {}
        pub fn overlay_opened(&mut self) -> bool {
            false
        }
        pub fn cleared(&mut self, _: &Game, _: &Profile) {}
        pub fn presence(&mut self, _: bool, _: &Game) {}
    }
}
