//! Shared wrapper option builders.

use std::collections::BTreeMap;
use std::time::Duration;

use crate::client::DataRequestOptions;
use crate::executor::ExecutionOptions;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// Sort direction for endpoints supporting `sort`.
pub enum SortOrder {
    /// Ascending.
    Asc,
    /// Descending.
    Desc,
}

impl SortOrder {
    fn as_api_value(self) -> &'static str {
        match self {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        }
    }
}

#[derive(Debug, Clone, Default)]
/// Common wrapper options mapped into [`crate::DataRequestOptions`].
pub struct BaseOptions {
    /// Generic/direct identifier.
    pub id: Option<String>,
    /// Generic pagination/limit parameter.
    pub limit: Option<u32>,
    /// Range lower bound.
    pub from: Option<u64>,
    /// Range upper bound.
    pub to: Option<u64>,
    /// Pagination offset.
    pub offset: Option<u32>,
    /// Generic category filter.
    pub cat: Option<String>,
    /// Generic stat filter.
    pub stat: Option<String>,
    /// Generic filters string.
    pub filters: Option<String>,
    /// Sort order.
    pub sort: Option<SortOrder>,
    /// Whether to strip tags in supported endpoints.
    pub striptags: Option<bool>,
    /// Generic `target` filter.
    pub target: Option<String>,
    /// Generic `name` filter.
    pub name: Option<String>,
    /// Generic `bonus` filter.
    pub bonus: Option<String>,
    /// Generic `ids` filter.
    pub ids: Option<String>,
    /// Generic `log` filter.
    pub log: Option<String>,
    /// Generic `comment` filter.
    pub comment: Option<String>,
    /// Generic `timestamp` filter.
    pub timestamp: Option<String>,
    /// Execution overrides.
    pub execution_options: ExecutionOptions,
    /// Legacy selections passed via `legacy`.
    pub legacy_selections: Vec<String>,
    /// Additional query filters.
    pub extra_filters: BTreeMap<String, String>,
    /// Additional path arguments.
    pub extra_path_args: BTreeMap<String, String>,
}

impl BaseOptions {
    /// Sets `id`.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets `limit`.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets `from`.
    pub fn with_from(mut self, from: u64) -> Self {
        self.from = Some(from);
        self
    }

    /// Sets `to`.
    pub fn with_to(mut self, to: u64) -> Self {
        self.to = Some(to);
        self
    }

    /// Sets `offset`.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Sets `cat`.
    pub fn with_cat(mut self, cat: impl Into<String>) -> Self {
        self.cat = Some(cat.into());
        self
    }

    /// Sets `stat`.
    pub fn with_stat(mut self, stat: impl Into<String>) -> Self {
        self.stat = Some(stat.into());
        self
    }

    /// Sets `filters`.
    pub fn with_filters(mut self, filters: impl Into<String>) -> Self {
        self.filters = Some(filters.into());
        self
    }

    /// Sets `sort`.
    pub fn with_sort(mut self, sort: SortOrder) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Sets `striptags`.
    pub fn with_striptags(mut self, striptags: bool) -> Self {
        self.striptags = Some(striptags);
        self
    }

    /// Sets `target`.
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Sets `name`.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets `bonus`.
    pub fn with_bonus(mut self, bonus: impl Into<String>) -> Self {
        self.bonus = Some(bonus.into());
        self
    }

    /// Sets `ids`.
    pub fn with_ids(mut self, ids: impl Into<String>) -> Self {
        self.ids = Some(ids.into());
        self
    }

    /// Sets `log`.
    pub fn with_log(mut self, log: impl Into<String>) -> Self {
        self.log = Some(log.into());
        self
    }

    /// Sets `comment`.
    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// Sets `timestamp`.
    pub fn with_timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    /// Replaces execution options for this wrapper request.
    pub fn with_execution_options(mut self, execution_options: ExecutionOptions) -> Self {
        self.execution_options = execution_options;
        self
    }

    /// Overrides max attempts for this wrapper request.
    pub fn with_max_attempts(mut self, max_attempts: u32) -> Self {
        self.execution_options.max_attempts = Some(max_attempts);
        self
    }

    /// Overrides per-request timeout for this wrapper request.
    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.execution_options.request_timeout = Some(timeout);
        self
    }

    /// Adds one legacy selection that should be passed via `legacy`.
    pub fn with_legacy_selection(mut self, selection: impl Into<String>) -> Self {
        self.legacy_selections.push(selection.into());
        self
    }

    /// Adds one extra query filter not covered by a dedicated helper.
    pub fn with_extra_filter(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra_filters.insert(name.into(), value.into());
        self
    }

    /// Adds one extra direct-endpoint path argument not covered by a dedicated helper.
    pub fn with_extra_path_arg(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.extra_path_args.insert(name.into(), value.into());
        self
    }

    /// Converts wrapper options into low-level data request options.
    pub fn into_data_request_options(self) -> DataRequestOptions {
        let mut options = DataRequestOptions::default();
        if let Some(id) = self.id {
            options = options.with_id(id);
        }
        if let Some(limit) = self.limit {
            options = options.with_filter("limit", limit.to_string());
        }
        if let Some(from) = self.from {
            options = options.with_filter("from", from.to_string());
        }
        if let Some(to) = self.to {
            options = options.with_filter("to", to.to_string());
        }
        if let Some(offset) = self.offset {
            options = options.with_filter("offset", offset.to_string());
        }
        if let Some(cat) = self.cat {
            options = options.with_filter("cat", cat);
        }
        if let Some(stat) = self.stat {
            options = options.with_filter("stat", stat);
        }
        if let Some(filters) = self.filters {
            options = options.with_filter("filters", filters);
        }
        if let Some(sort) = self.sort {
            options = options.with_filter("sort", sort.as_api_value());
        }
        if let Some(striptags) = self.striptags {
            options = options.with_filter("striptags", striptags.to_string());
        }
        if let Some(target) = self.target {
            options = options.with_filter("target", target);
        }
        if let Some(name) = self.name {
            options = options.with_filter("name", name);
        }
        if let Some(bonus) = self.bonus {
            options = options.with_filter("bonus", bonus);
        }
        if let Some(ids) = self.ids {
            options = options.with_filter("ids", ids);
        }
        if let Some(log) = self.log {
            options = options.with_filter("log", log);
        }
        if let Some(comment) = self.comment {
            options = options.with_filter("comment", comment);
        }
        if let Some(timestamp) = self.timestamp {
            options = options.with_filter("timestamp", timestamp);
        }
        for selection in self.legacy_selections {
            options = options.with_legacy_selection(selection);
        }
        for (name, value) in self.extra_filters {
            options = options.with_filter(name, value);
        }
        for (name, value) in self.extra_path_args {
            options = options.with_path_arg(name, value);
        }

        options.with_execution_options(self.execution_options)
    }
}
