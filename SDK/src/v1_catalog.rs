#[derive(Debug, Clone, Copy)]
/// Static v1 resource specification used for fallback routing.
pub struct V1ResourceSpec {
    /// Canonical resource name.
    pub name: &'static str,
    /// Path segment for generic v1 calls.
    pub path_segment: &'static str,
    /// Known v1 selection names for this resource.
    pub selections: &'static [&'static str],
}

impl V1ResourceSpec {
    /// Returns whether the resource supports the provided selection.
    pub fn has_selection(&self, selection: &str) -> bool {
        self.selections.contains(&selection)
    }
}

#[derive(Debug, Clone, Copy, Default)]
/// Lookup catalog for known Torn API v1 resources and selections.
pub struct V1Catalog;

impl V1Catalog {
    /// Returns the static v1 resource specification by resource name.
    pub fn resource(&self, resource: &str) -> Option<&'static V1ResourceSpec> {
        match resource {
            "user" => Some(&USER),
            "property" => Some(&PROPERTY),
            "faction" => Some(&FACTION),
            "company" => Some(&COMPANY),
            "market" => Some(&MARKET),
            "torn" => Some(&TORN),
            _ => None,
        }
    }

    /// Returns whether the resource supports the provided v1 selection.
    pub fn has_selection(&self, resource: &str, selection: &str) -> bool {
        self.resource(resource)
            .map(|spec| spec.has_selection(selection))
            .unwrap_or(false)
    }

    /// Builds a generic v1 request path, optionally including `id`.
    pub fn build_generic_path(&self, resource: &str, id: Option<&str>) -> Option<String> {
        let spec = self.resource(resource)?;
        let base = format!("/{}", spec.path_segment);
        let id = id.map(str::trim).filter(|value| !value.is_empty());
        Some(match id {
            Some(id) => format!("{base}/{id}"),
            None => base,
        })
    }
}

pub(crate) const V1_ALLOWED_FILTERS: &[&str] = &[
    "cat",
    "filters",
    "from",
    "limit",
    "offset",
    "sort",
    "stat",
    "striptags",
    "timestamp",
    "to",
];

pub(crate) fn is_allowed_v1_filter(name: &str) -> bool {
    V1_ALLOWED_FILTERS.contains(&name)
}

const USER_SELECTIONS: &[&str] = &[
    "ammo",
    "attacks",
    "attacksfull",
    "bars",
    "basic",
    "battlestats",
    "bazaar",
    "bounties",
    "calendar",
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
    "networth",
    "newevents",
    "newmessages",
    "notifications",
    "organizedcrime",
    "organizedcrimes",
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
    "travel",
    "virus",
    "weaponexp",
    "workstats",
];

const PROPERTY_SELECTIONS: &[&str] = &["lookup", "property", "timestamp"];

const FACTION_SELECTIONS: &[&str] = &[
    "applications",
    "armor",
    "armorynews",
    "attacknews",
    "attacks",
    "attacksfull",
    "balance",
    "basic",
    "boosters",
    "caches",
    "cesium",
    "chain",
    "chainreport",
    "chains",
    "contributors",
    "crime",
    "crimeexp",
    "crimenews",
    "crimes",
    "currency",
    "donations",
    "drugs",
    "fundsnews",
    "hof",
    "lookup",
    "mainnews",
    "medical",
    "members",
    "membershipnews",
    "news",
    "positions",
    "rackets",
    "raidreport",
    "raids",
    "rankedwarreport",
    "rankedwars",
    "reports",
    "revives",
    "revivesfull",
    "search",
    "stats",
    "temporary",
    "territory",
    "territorynews",
    "territoryownership",
    "territorywarreport",
    "territorywars",
    "timestamp",
    "upgrades",
    "utilities",
    "warfare",
    "wars",
    "weapons",
];

const COMPANY_SELECTIONS: &[&str] = &[
    "applications",
    "companies",
    "detailed",
    "employees",
    "lookup",
    "news",
    "profile",
    "stock",
    "timestamp",
];

const MARKET_SELECTIONS: &[&str] = &[
    "auctionhouse",
    "auctionhouselisting",
    "bazaar",
    "itemmarket",
    "lookup",
    "pointsmarket",
    "properties",
    "rentals",
    "timestamp",
];

const TORN_SELECTIONS: &[&str] = &[
    "attacklog",
    "bank",
    "bounties",
    "calendar",
    "cards",
    "chainreport",
    "cityshops",
    "companies",
    "competition",
    "crimes",
    "dirtybombs",
    "education",
    "elimination",
    "eliminationteam",
    "factionhof",
    "factiontree",
    "gyms",
    "hof",
    "honors",
    "itemammo",
    "itemdetails",
    "itemmods",
    "items",
    "itemstats",
    "logcategories",
    "logtypes",
    "lookup",
    "medals",
    "merits",
    "organisedcrimes",
    "organizedcrimes",
    "pawnshop",
    "pokertables",
    "properties",
    "rackets",
    "raidreport",
    "raids",
    "rankedwarreport",
    "rankedwars",
    "rockpaperscissors",
    "searchforcash",
    "shoplifting",
    "stats",
    "stocks",
    "subcrimes",
    "territory",
    "territorynames",
    "territorywarreport",
    "territorywars",
    "timestamp",
];

const USER: V1ResourceSpec = V1ResourceSpec {
    name: "user",
    path_segment: "user",
    selections: USER_SELECTIONS,
};

const PROPERTY: V1ResourceSpec = V1ResourceSpec {
    name: "property",
    path_segment: "property",
    selections: PROPERTY_SELECTIONS,
};

const FACTION: V1ResourceSpec = V1ResourceSpec {
    name: "faction",
    path_segment: "faction",
    selections: FACTION_SELECTIONS,
};

const COMPANY: V1ResourceSpec = V1ResourceSpec {
    name: "company",
    path_segment: "company",
    selections: COMPANY_SELECTIONS,
};

const MARKET: V1ResourceSpec = V1ResourceSpec {
    name: "market",
    path_segment: "market",
    selections: MARKET_SELECTIONS,
};

const TORN: V1ResourceSpec = V1ResourceSpec {
    name: "torn",
    path_segment: "torn",
    selections: TORN_SELECTIONS,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_company_v1_only_resource() {
        let catalog = V1Catalog;
        assert!(catalog.resource("company").is_some());
        assert!(catalog.has_selection("company", "profile"));
        assert_eq!(
            catalog.build_generic_path("company", Some("123")),
            Some("/company/123".to_string())
        );
    }

    #[test]
    fn supports_user_selection_lists_and_path_building() {
        let catalog = V1Catalog;
        assert!(catalog.has_selection("user", "profile"));
        assert!(catalog.has_selection("user", "bazaar"));
        assert_eq!(
            catalog.build_generic_path("user", Some("3637232")),
            Some("/user/3637232".to_string())
        );
        assert_eq!(
            catalog.build_generic_path("user", None),
            Some("/user".to_string())
        );
    }
}
