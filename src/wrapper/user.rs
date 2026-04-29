//! User resource wrapper APIs.

use crate::client::{DataRequestOptions, TornClient};
use crate::models::generated::RawSelectionBundle;
use crate::models::manual::user::{
    UserAmmoBundle, UserAttacksBundle, UserAttacksFullBundle, UserBarsBundle, UserBasicBundle,
    UserBattleStatsBundle, UserBazaarBundle, UserBountiesBundle, UserCalendarBundle,
    UserCasinoBundle, UserCompetitionBundle, UserCooldownsBundle, UserCrimesBundle,
    UserCriminalRecordBundle, UserDiscordBundle, UserDisplayBundle, UserEducationBundle,
    UserEnlistedCarsBundle, UserEquipmentBundle, UserEventsBundle, UserFactionBalanceBundle,
    UserFactionBundle, UserForumFeedBundle, UserForumFriendsBundle, UserForumPostsBundle,
    UserForumSubscribedThreadsBundle, UserForumThreadsBundle, UserGymBundle, UserHofBundle,
    UserHonorsBundle, UserIconsBundle, UserInventoryBundle, UserItemMarketBundle, UserJobBundle,
    UserJobPointsBundle, UserJobRanksBundle, UserListBundle, UserLogBundle, UserLookupBundle,
    UserMedalsBundle, UserMeritsBundle, UserMessagesBundle, UserMissionsBundle, UserMoneyBundle,
    UserNetworthBundle, UserNewEventsBundle, UserNewMessagesBundle, UserNotificationsBundle,
    UserOrganizedCrimeBundle, UserPerksBundle, UserPersonalStatsBundle, UserProfileBundle,
    UserPropertiesBundle, UserPropertyBundle, UserRacesBundle, UserRacingRecordsBundle,
    UserRefillsBundle, UserReportsBundle, UserRevivesBundle, UserRevivesFullBundle,
    UserSkillsBundle, UserStocksBundle, UserTimestampBundle, UserTradeBundle, UserTradesBundle,
    UserTravelBundle, UserWeaponExpBundle, UserWorkStatsBundle,
};
use crate::transport::HttpTransport;
use crate::wrapper::error::SdkError;
use crate::wrapper::internal::{
    execute_raw, execute_typed, validate_range, validate_required_path_arg,
    validate_required_query_arg, validate_selection, validate_selections,
};
use crate::wrapper::options::BaseOptions;

macro_rules! raw_selection_methods {
    ($options:ty; $($method:ident => $selection:literal),* $(,)?) => {
        $(
            #[doc = concat!("Convenience helper for raw `user.", $selection, "`.")]
            pub async fn $method(&self, options: $options) -> Result<RawSelectionBundle, SdkError> {
                self.raw_selection($selection, options).await
            }
        )*
    };
}

#[derive(Debug, Clone, Default)]
/// Options for `user` wrapper requests.
pub struct UserOptions {
    /// Shared base options.
    pub base: BaseOptions,
    /// Optional crime identifier for `crimes` endpoint variants.
    pub crime_id: Option<String>,
    /// Optional trade identifier for `trade` endpoint variants.
    pub trade_id: Option<String>,
}

impl UserOptions {
    /// Replaces the shared base options.
    pub fn with_base(mut self, base: BaseOptions) -> Self {
        self.base = base;
        self
    }

    /// Sets the generic/direct `id` value.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.base = self.base.with_id(id);
        self
    }

    /// Sets `crimeId` for selections that require it.
    pub fn with_crime_id(mut self, value: impl Into<String>) -> Self {
        self.crime_id = Some(value.into());
        self
    }

    /// Sets `tradeId` for selections that require it.
    pub fn with_trade_id(mut self, value: impl Into<String>) -> Self {
        self.trade_id = Some(value.into());
        self
    }

    pub(crate) fn into_data_request_options(self) -> DataRequestOptions {
        let mut options = self.base.into_data_request_options();
        if let Some(value) = self.crime_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("crimeId", value);
        }
        if let Some(value) = self.trade_id {
            if options.id.is_none() {
                options = options.with_id(value.clone());
            }
            options = options.with_path_arg("tradeId", value);
        }
        options
    }
}

#[derive(Debug, Clone, Copy)]
/// User resource API entrypoint.
pub struct UserApi<'a, T: HttpTransport> {
    pub(crate) client: &'a TornClient<T>,
}

impl<T: HttpTransport> UserApi<'_, T> {
    /// Supported `user` selections validated by wrapper helpers.
    pub const SUPPORTED_SELECTIONS: &'static [&'static str] = &[
        "ammo",
        "attacks",
        "attacksfull",
        "bars",
        "basic",
        "battlestats",
        "bazaar",
        "bounties",
        "calendar",
        "casino",
        "competition",
        "cooldowns",
        "crimes",
        "criminalrecord",
        "discord",
        "display",
        "education",
        "enlistedcars",
        "equipment",
        "events",
        "faction",
        "factionbalance",
        "forumfeed",
        "forumfriends",
        "forumposts",
        "forumsubscribedthreads",
        "forumthreads",
        "gym",
        "hof",
        "honors",
        "icons",
        "inventory",
        "itemmarket",
        "job",
        "jobpoints",
        "jobranks",
        "list",
        "log",
        "lookup",
        "medals",
        "merits",
        "messages",
        "missions",
        "money",
        "newevents",
        "networth",
        "newmessages",
        "notifications",
        "organizedcrime",
        "perks",
        "personalstats",
        "profile",
        "properties",
        "property",
        "races",
        "racingrecords",
        "refills",
        "reports",
        "revives",
        "revivesfull",
        "skills",
        "stocks",
        "timestamp",
        "trade",
        "trades",
        "travel",
        "weaponexp",
        "workstats",
    ];

    /// Returns the underlying low-level client.
    pub fn raw_client(&self) -> &TornClient<T> {
        self.client
    }

    /// Typed helper for `user.profile`.
    pub async fn profile(&self, user_id: impl Into<String>) -> Result<UserProfileBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["profile".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.basic`.
    pub async fn basic(&self, user_id: impl Into<String>) -> Result<UserBasicBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["basic".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.discord`.
    pub async fn discord(&self, user_id: impl Into<String>) -> Result<UserDiscordBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["discord".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.casino`.
    pub async fn casino(&self) -> Result<UserCasinoBundle, SdkError> {
        execute_typed(
            self.client,
            "user",
            vec!["casino".to_string()],
            UserOptions::default().into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.faction`.
    pub async fn faction(&self, user_id: impl Into<String>) -> Result<UserFactionBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["faction".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.money`.
    pub async fn money(&self, options: UserOptions) -> Result<UserMoneyBundle, SdkError> {
        validate_options_for_selection("money", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["money".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.bars`.
    pub async fn bars(&self, options: UserOptions) -> Result<UserBarsBundle, SdkError> {
        validate_options_for_selection("bars", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["bars".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.cooldowns`.
    pub async fn cooldowns(&self, options: UserOptions) -> Result<UserCooldownsBundle, SdkError> {
        validate_options_for_selection("cooldowns", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["cooldowns".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.ammo`.
    pub async fn ammo(&self, options: UserOptions) -> Result<UserAmmoBundle, SdkError> {
        validate_options_for_selection("ammo", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["ammo".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.battlestats`.
    pub async fn battlestats(
        &self,
        options: UserOptions,
    ) -> Result<UserBattleStatsBundle, SdkError> {
        validate_options_for_selection("battlestats", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["battlestats".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.equipment`.
    pub async fn equipment(&self, options: UserOptions) -> Result<UserEquipmentBundle, SdkError> {
        validate_options_for_selection("equipment", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["equipment".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.missions`.
    pub async fn missions(&self, options: UserOptions) -> Result<UserMissionsBundle, SdkError> {
        validate_options_for_selection("missions", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["missions".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.timestamp`.
    pub async fn timestamp(&self, options: UserOptions) -> Result<UserTimestampBundle, SdkError> {
        validate_options_for_selection("timestamp", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["timestamp".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.notifications`.
    pub async fn notifications(
        &self,
        options: UserOptions,
    ) -> Result<UserNotificationsBundle, SdkError> {
        validate_options_for_selection("notifications", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["notifications".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.icons`.
    pub async fn icons(&self, options: UserOptions) -> Result<UserIconsBundle, SdkError> {
        validate_options_for_selection("icons", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["icons".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.enlistedcars`.
    pub async fn enlistedcars(
        &self,
        options: UserOptions,
    ) -> Result<UserEnlistedCarsBundle, SdkError> {
        validate_options_for_selection("enlistedcars", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["enlistedcars".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.job`.
    pub async fn job(&self, options: UserOptions) -> Result<UserJobBundle, SdkError> {
        validate_options_for_selection("job", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["job".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.jobpoints`.
    pub async fn jobpoints(&self, options: UserOptions) -> Result<UserJobPointsBundle, SdkError> {
        validate_options_for_selection("jobpoints", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["jobpoints".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.jobranks`.
    pub async fn jobranks(&self, options: UserOptions) -> Result<UserJobRanksBundle, SdkError> {
        validate_options_for_selection("jobranks", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["jobranks".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.refills`.
    pub async fn refills(&self, options: UserOptions) -> Result<UserRefillsBundle, SdkError> {
        validate_options_for_selection("refills", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["refills".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.skills`.
    pub async fn skills(&self, options: UserOptions) -> Result<UserSkillsBundle, SdkError> {
        validate_options_for_selection("skills", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["skills".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.stocks`.
    pub async fn stocks(&self, options: UserOptions) -> Result<UserStocksBundle, SdkError> {
        validate_options_for_selection("stocks", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["stocks".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.calendar`.
    pub async fn calendar(&self, options: UserOptions) -> Result<UserCalendarBundle, SdkError> {
        validate_options_for_selection("calendar", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["calendar".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.lookup`.
    pub async fn lookup(&self, options: UserOptions) -> Result<UserLookupBundle, SdkError> {
        validate_options_for_selection("lookup", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["lookup".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.events`.
    pub async fn events(&self, options: UserOptions) -> Result<UserEventsBundle, SdkError> {
        validate_options_for_selection("events", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["events".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.newevents`.
    pub async fn newevents(&self, options: UserOptions) -> Result<UserNewEventsBundle, SdkError> {
        validate_options_for_selection("newevents", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["newevents".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.messages`.
    pub async fn messages(&self, options: UserOptions) -> Result<UserMessagesBundle, SdkError> {
        validate_options_for_selection("messages", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["messages".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.newmessages`.
    pub async fn newmessages(
        &self,
        options: UserOptions,
    ) -> Result<UserNewMessagesBundle, SdkError> {
        validate_options_for_selection("newmessages", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["newmessages".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.races`.
    pub async fn races(&self, options: UserOptions) -> Result<UserRacesBundle, SdkError> {
        validate_options_for_selection("races", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["races".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.attacks`.
    pub async fn attacks(&self, options: UserOptions) -> Result<UserAttacksBundle, SdkError> {
        validate_options_for_selection("attacks", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["attacks".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.attacksfull`.
    pub async fn attacksfull(
        &self,
        options: UserOptions,
    ) -> Result<UserAttacksFullBundle, SdkError> {
        validate_options_for_selection("attacksfull", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["attacksfull".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.revives`.
    pub async fn revives(&self, options: UserOptions) -> Result<UserRevivesBundle, SdkError> {
        validate_options_for_selection("revives", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["revives".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.revivesfull`.
    pub async fn revivesfull(
        &self,
        options: UserOptions,
    ) -> Result<UserRevivesFullBundle, SdkError> {
        validate_options_for_selection("revivesfull", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["revivesfull".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.hof`.
    pub async fn hof(&self, user_id: impl Into<String>) -> Result<UserHofBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["hof".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.bounties`.
    pub async fn bounties(
        &self,
        user_id: impl Into<String>,
    ) -> Result<UserBountiesBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["bounties".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.properties`.
    pub async fn properties(
        &self,
        user_id: impl Into<String>,
    ) -> Result<UserPropertiesBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["properties".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.property`.
    pub async fn property(
        &self,
        user_id: impl Into<String>,
    ) -> Result<UserPropertyBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["property".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.forumthreads`.
    pub async fn forumthreads(
        &self,
        user_id: impl Into<String>,
    ) -> Result<UserForumThreadsBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["forumthreads".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.forumposts`.
    pub async fn forumposts(
        &self,
        user_id: impl Into<String>,
    ) -> Result<UserForumPostsBundle, SdkError> {
        let options = UserOptions::default().with_id(user_id.into());
        execute_typed(
            self.client,
            "user",
            vec!["forumposts".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.forumsubscribedthreads`.
    pub async fn forumsubscribedthreads(
        &self,
        options: UserOptions,
    ) -> Result<UserForumSubscribedThreadsBundle, SdkError> {
        validate_options_for_selection("forumsubscribedthreads", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["forumsubscribedthreads".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.forumfriends`.
    pub async fn forumfriends(
        &self,
        options: UserOptions,
    ) -> Result<UserForumFriendsBundle, SdkError> {
        validate_options_for_selection("forumfriends", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["forumfriends".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.forumfeed`.
    pub async fn forumfeed(&self, options: UserOptions) -> Result<UserForumFeedBundle, SdkError> {
        validate_options_for_selection("forumfeed", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["forumfeed".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.workstats`.
    pub async fn workstats(&self, options: UserOptions) -> Result<UserWorkStatsBundle, SdkError> {
        validate_options_for_selection("workstats", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["workstats".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.merits`.
    pub async fn merits(&self, options: UserOptions) -> Result<UserMeritsBundle, SdkError> {
        validate_options_for_selection("merits", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["merits".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.itemmarket`.
    pub async fn itemmarket(&self, options: UserOptions) -> Result<UserItemMarketBundle, SdkError> {
        validate_options_for_selection("itemmarket", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["itemmarket".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.organizedcrime`.
    pub async fn organizedcrime(
        &self,
        options: UserOptions,
    ) -> Result<UserOrganizedCrimeBundle, SdkError> {
        validate_options_for_selection("organizedcrime", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["organizedcrime".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.factionbalance`.
    pub async fn factionbalance(
        &self,
        options: UserOptions,
    ) -> Result<UserFactionBalanceBundle, SdkError> {
        validate_options_for_selection("factionbalance", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["factionbalance".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.education`.
    pub async fn education(&self, options: UserOptions) -> Result<UserEducationBundle, SdkError> {
        validate_options_for_selection("education", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["education".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.competition`.
    pub async fn competition(
        &self,
        options: UserOptions,
    ) -> Result<UserCompetitionBundle, SdkError> {
        validate_options_for_selection("competition", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["competition".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.weaponexp`.
    pub async fn weaponexp(&self, options: UserOptions) -> Result<UserWeaponExpBundle, SdkError> {
        validate_options_for_selection("weaponexp", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["weaponexp".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.racingrecords`.
    pub async fn racingrecords(
        &self,
        options: UserOptions,
    ) -> Result<UserRacingRecordsBundle, SdkError> {
        validate_options_for_selection("racingrecords", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["racingrecords".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.honors`.
    pub async fn honors(&self, options: UserOptions) -> Result<UserHonorsBundle, SdkError> {
        validate_options_for_selection("honors", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["honors".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.medals`.
    pub async fn medals(&self, options: UserOptions) -> Result<UserMedalsBundle, SdkError> {
        validate_options_for_selection("medals", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["medals".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.inventory`.
    pub async fn inventory(&self, options: UserOptions) -> Result<UserInventoryBundle, SdkError> {
        validate_options_for_selection("inventory", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["inventory".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.trades`.
    pub async fn trades(&self, options: UserOptions) -> Result<UserTradesBundle, SdkError> {
        validate_options_for_selection("trades", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["trades".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.trade`.
    pub async fn trade(&self, options: UserOptions) -> Result<UserTradeBundle, SdkError> {
        validate_options_for_selection("trade", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["trade".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.travel`.
    pub async fn travel(&self, options: UserOptions) -> Result<UserTravelBundle, SdkError> {
        validate_options_for_selection("travel", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["travel".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `user.display`.
    pub async fn display(&self, options: UserOptions) -> Result<UserDisplayBundle, SdkError> {
        validate_options_for_selection("display", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["display".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `user.gym`.
    pub async fn gym(&self, options: UserOptions) -> Result<UserGymBundle, SdkError> {
        validate_options_for_selection("gym", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["gym".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `user.perks`.
    pub async fn perks(&self, options: UserOptions) -> Result<UserPerksBundle, SdkError> {
        validate_options_for_selection("perks", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["perks".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `user.criminalrecord`.
    pub async fn criminalrecord(
        &self,
        options: UserOptions,
    ) -> Result<UserCriminalRecordBundle, SdkError> {
        validate_options_for_selection("criminalrecord", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["criminalrecord".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `user.bazaar`.
    pub async fn bazaar(&self, options: UserOptions) -> Result<UserBazaarBundle, SdkError> {
        validate_options_for_selection("bazaar", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["bazaar".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.reports`.
    pub async fn reports(&self, options: UserOptions) -> Result<UserReportsBundle, SdkError> {
        validate_options_for_selection("reports", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["reports".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.crimes`.
    pub async fn crimes(&self, options: UserOptions) -> Result<UserCrimesBundle, SdkError> {
        validate_options_for_selection("crimes", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["crimes".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.list`.
    pub async fn list(&self, options: UserOptions) -> Result<UserListBundle, SdkError> {
        validate_options_for_selection("list", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["list".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.log`.
    pub async fn log(&self, options: UserOptions) -> Result<UserLogBundle, SdkError> {
        validate_options_for_selection("log", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["log".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for fallback `user.networth`.
    pub async fn networth(&self, options: UserOptions) -> Result<UserNetworthBundle, SdkError> {
        validate_options_for_selection("networth", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["networth".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Typed helper for `user.personalstats`.
    pub async fn personalstats(
        &self,
        options: UserOptions,
    ) -> Result<UserPersonalStatsBundle, SdkError> {
        validate_options_for_selection("personalstats", &options)?;
        execute_typed(
            self.client,
            "user",
            vec!["personalstats".to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes a single validated selection and returns a raw JSON bundle.
    pub async fn raw_selection(
        &self,
        selection: &str,
        options: UserOptions,
    ) -> Result<RawSelectionBundle, SdkError> {
        validate_selection("user", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_raw(
            self.client,
            "user",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes multiple validated selections and returns a raw JSON bundle.
    pub async fn raw_selections<S, I>(
        &self,
        selections: I,
        options: UserOptions,
    ) -> Result<RawSelectionBundle, SdkError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let selection_values = selections.into_iter().map(Into::into).collect::<Vec<_>>();
        validate_selections("user", &selection_values, Self::SUPPORTED_SELECTIONS)?;
        for selection in &selection_values {
            validate_options_for_selection(selection, &options)?;
        }
        execute_raw(
            self.client,
            "user",
            selection_values,
            options.into_data_request_options(),
        )
        .await
    }

    /// Executes one validated selection and deserializes into a caller type.
    pub async fn typed_selection<R>(
        &self,
        selection: &str,
        options: UserOptions,
    ) -> Result<R, SdkError>
    where
        R: serde::de::DeserializeOwned,
    {
        validate_selection("user", selection, Self::SUPPORTED_SELECTIONS)?;
        validate_options_for_selection(selection, &options)?;
        execute_typed(
            self.client,
            "user",
            vec![selection.to_string()],
            options.into_data_request_options(),
        )
        .await
    }

    raw_selection_methods!(UserOptions;
        networth_raw => "networth",
        newevents_raw => "newevents",
        missions_raw => "missions",
        money_raw => "money",
        organizedcrime_raw => "organizedcrime",
        perks_raw => "perks",
        newmessages_raw => "newmessages",
        notifications_raw => "notifications",
        list_raw => "list",
        log_raw => "log",
        jobpoints_raw => "jobpoints",
        jobranks_raw => "jobranks",
        merits_raw => "merits",
        messages_raw => "messages",
        lookup_raw => "lookup",
        medals_raw => "medals",
        personalstats_raw => "personalstats",
        stocks_raw => "stocks",
        timestamp_raw => "timestamp",
        trade_raw => "trade",
        trades_raw => "trades",
        revivesfull_raw => "revivesfull",
        skills_raw => "skills",
        workstats_raw => "workstats",
        travel_raw => "travel",
        weaponexp_raw => "weaponexp",
        property_raw => "property",
        races_raw => "races",
        profile_raw => "profile",
        properties_raw => "properties",
        reports_raw => "reports",
        revives_raw => "revives",
        racingrecords_raw => "racingrecords",
        refills_raw => "refills",
        cooldowns_raw => "cooldowns",
        crimes_raw => "crimes",
        calendar_raw => "calendar",
        casino_raw => "casino",
        competition_raw => "competition",
        display_raw => "display",
        education_raw => "education",
        criminalrecord_raw => "criminalrecord",
        discord_raw => "discord",
        attacksfull_raw => "attacksfull",
        bars_raw => "bars",
        ammo_raw => "ammo",
        attacks_raw => "attacks",
        bazaar_raw => "bazaar",
        bounties_raw => "bounties",
        basic_raw => "basic",
        battlestats_raw => "battlestats",
        enlistedcars_raw => "enlistedcars",
        hof_raw => "hof",
        honors_raw => "honors",
        forumthreads_raw => "forumthreads",
        gym_raw => "gym",
        itemmarket_raw => "itemmarket",
        job_raw => "job",
        icons_raw => "icons",
        inventory_raw => "inventory",
        faction_raw => "faction",
        factionbalance_raw => "factionbalance",
        equipment_raw => "equipment",
        events_raw => "events",
        forumposts_raw => "forumposts",
        forumsubscribedthreads_raw => "forumsubscribedthreads",
        forumfeed_raw => "forumfeed",
        forumfriends_raw => "forumfriends",
    );
}

fn validate_options_for_selection(selection: &str, options: &UserOptions) -> Result<(), SdkError> {
    validate_range("user", options.base.from, options.base.to)?;

    if requires_user_id(selection) {
        validate_required_path_arg("user", selection, "id", options.base.id.is_some())?;
    }

    if selection == "crimes" {
        validate_required_path_arg(
            "user",
            selection,
            "crimeId",
            options.crime_id.is_some() || options.base.id.is_some(),
        )?;
    }

    if selection == "list" {
        validate_required_query_arg("user", selection, "cat", options.base.cat.is_some())?;
    }

    if selection == "inventory" {
        validate_required_query_arg("user", selection, "cat", options.base.cat.is_some())?;
    }

    if selection == "personalstats" {
        let has_cat = options.base.cat.is_some();
        let has_stat = options.base.stat.is_some();

        if has_cat == has_stat {
            return Err(SdkError::Validation(
                "resource 'user' selection 'personalstats' requires exactly one of 'cat' or 'stat'"
                    .to_string(),
            ));
        }
    }

    if selection == "trade" {
        validate_required_path_arg(
            "user",
            selection,
            "tradeId",
            options.trade_id.is_some() || options.base.id.is_some(),
        )?;
    }

    Ok(())
}

fn requires_user_id(selection: &str) -> bool {
    matches!(
        selection,
        "basic"
            | "bounties"
            | "discord"
            | "faction"
            | "forumposts"
            | "forumthreads"
            | "hof"
            | "profile"
            | "properties"
            | "property"
    )
}
