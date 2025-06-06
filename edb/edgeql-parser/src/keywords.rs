use phf::phf_set;

pub const UNRESERVED_KEYWORDS: phf::Set<&str> = phf_set!(
    "abort",
    "abstract",
    "access",
    "after",
    "alias",
    "allow",
    "all",
    "annotation",
    "applied",
    "as",
    "asc",
    "assignment",
    "before",
    "blobal",
    "branch",
    "cardinality",
    "cast",
    "committed",
    "config",
    "conflict",
    "constraint",
    "cube",
    "current",
    "data",
    "database",
    "ddl",
    "declare",
    "default",
    "deferrable",
    "deferred",
    "delegated",
    "desc",
    "deny",
    "each",
    "empty",
    "expression",
    "extension",
    "final",
    "first",
    "force",
    "from",
    "function",
    "future",
    "implicit",
    "index",
    "infix",
    "inheritable",
    "instance",
    "into",
    "isolation",
    "json",
    "last",
    "link",
    "migration",
    "multi",
    "named",
    "object",
    "of",
    "only",
    "onto",
    "operator",
    "optionality",
    "order",
    "orphan",
    "overloaded",
    "owned",
    "package",
    "policy",
    "populate",
    "postfix",
    "prefix",
    "property",
    "proposed",
    "pseudo",
    "read",
    "reject",
    "release",
    "rename",
    "repeatable",
    "required",
    "reset",
    "restrict",
    "rewrite",
    "role",
    "roles",
    "rollup",
    "savepoint",
    "scalar",
    "schema",
    "sdl",
    "serializable",
    "session",
    "source",
    "superuser",
    "system",
    "target",
    "template",
    "ternary",
    "text",
    "then",
    "to",
    "transaction",
    "trigger",
    "type",
    "unless",
    "using",
    "verbose",
    "version",
    "view",
    "write",
);

pub const PARTIAL_RESERVED_KEYWORDS: phf::Set<&str> = phf_set!("except", "intersect", "union",);

pub const FUTURE_RESERVED_KEYWORDS: phf::Set<&str> = phf_set!(
    "anyarray",
    "begin",
    "case",
    "check",
    "deallocate",
    "discard",
    "end",
    "explain",
    "fetch",
    "get",
    "global",
    "grant",
    "import",
    "listen",
    "load",
    "lock",
    "match",
    "move",
    "notify",
    "on",
    "over",
    "prepare",
    "partition",
    "raise",
    "refresh",
    "revoke",
    "single",
    "when",
    "window",
    "never",
);

pub const CURRENT_RESERVED_KEYWORDS: phf::Set<&str> = phf_set!(
    "__source__",
    "__subject__",
    "__type__",
    "__std__",
    "__edgedbsys__",
    "__edgedbtpl__",
    "__new__",
    "__old__",
    "__specified__",
    "__default__",
    "administer",
    "alter",
    "analyze",
    "and",
    "anytuple",
    "anytype",
    "anyobject",
    "by",
    "commit",
    "configure",
    "create",
    "delete",
    "describe",
    "detached",
    "distinct",
    "do",
    "drop",
    "else",
    "exists",
    "extending",
    "false",
    "filter",
    "for",
    "group",
    "if",
    "ilike",
    "in",
    "insert",
    "introspect",
    "is",
    "like",
    "limit",
    "module",
    "not",
    "offset",
    "optional",
    "or",
    "rollback",
    "select",
    "set",
    "start",
    "true",
    "typeof",
    "update",
    "variadic",
    "with",
);

pub const COMBINED_KEYWORDS: phf::Set<&str> = phf_set!(
    "named only",
    "set annotation",
    "set type",
    "extension package",
    "order by",
);

pub fn lookup(s: &str) -> Option<Keyword> {
    None.or_else(|| PARTIAL_RESERVED_KEYWORDS.get_key(s))
        .or_else(|| FUTURE_RESERVED_KEYWORDS.get_key(s))
        .or_else(|| CURRENT_RESERVED_KEYWORDS.get_key(s))
        .map(|x| Keyword(x))
}

pub fn lookup_all(s: &str) -> Option<Keyword> {
    lookup(s).or_else(|| {
        None.or_else(|| COMBINED_KEYWORDS.get_key(s))
            .or_else(|| UNRESERVED_KEYWORDS.get_key(s))
            .map(|x| Keyword(x))
    })
}

/// This is required for serde deserializer for Token to work correctly.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Keyword(pub &'static str);

impl Keyword {
    pub fn is_reserved(&self) -> bool {
        FUTURE_RESERVED_KEYWORDS.contains(self.0) || CURRENT_RESERVED_KEYWORDS.contains(self.0)
    }
    pub fn is_unreserved(&self) -> bool {
        UNRESERVED_KEYWORDS.contains(self.0) || PARTIAL_RESERVED_KEYWORDS.contains(self.0)
    }
    pub fn is_dunder(&self) -> bool {
        self.0.starts_with("__") && self.0.ends_with("__")
    }
    pub fn is_bool(&self) -> bool {
        self.0 == "true" || self.0 == "false"
    }
}

impl From<Keyword> for &'static str {
    fn from(value: Keyword) -> Self {
        value.0
    }
}
