// [#protodoc-title: Regex matcher]

/// A regex matcher designed for safety when used with untrusted input.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegexMatcher {
    /// The regex match string. The string must be supported by the configured engine.
    #[prost(string, tag = "2")]
    pub regex: std::string::String,
    #[prost(oneof = "regex_matcher::EngineType", tags = "1")]
    pub engine_type: ::std::option::Option<regex_matcher::EngineType>,
}
pub mod regex_matcher {
    /// Google's `RE2 <https://github.com/google/re2>`_ regex engine. The regex string must adhere to
    /// the documented `syntax <https://github.com/google/re2/wiki/Syntax>`_. The engine is designed
    /// to complete execution in linear time as well as limit the amount of memory used.
    ///
    /// Envoy supports program size checking via runtime. The runtime keys `re2.max_program_size.error_level`
    /// and `re2.max_program_size.warn_level` can be set to integers as the maximum program size or
    /// complexity that a compiled regex can have before an exception is thrown or a warning is
    /// logged, respectively. `re2.max_program_size.error_level` defaults to 100, and
    /// `re2.max_program_size.warn_level` has no default if unset (will not check/log a warning).
    ///
    /// Envoy emits two stats for tracking the program size of regexes: the histogram `re2.program_size`,
    /// which records the program size, and the counter `re2.exceeded_warn_level`, which is incremented
    /// each time the program size exceeds the warn level threshold.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GoogleRe2 {
        /// This field controls the RE2 "program size" which is a rough estimate of how complex a
        /// compiled regex is to evaluate. A regex that has a program size greater than the configured
        /// value will fail to compile. In this case, the configured max program size can be increased
        /// or the regex can be simplified. If not specified, the default is 100.
        ///
        /// This field is deprecated; regexp validation should be performed on the management server
        /// instead of being done by each individual client.
        #[prost(message, optional, tag = "1")]
        pub max_program_size: ::std::option::Option<u32>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EngineType {
        /// Google's RE2 regex engine.
        #[prost(message, tag = "1")]
        GoogleRe2(GoogleRe2),
    }
}
/// Describes how to match a string and then produce a new string using a regular
/// expression and a substitution string.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegexMatchAndSubstitute {
    /// The regular expression used to find portions of a string (hereafter called
    /// the "subject string") that should be replaced. When a new string is
    /// produced during the substitution operation, the new string is initially
    /// the same as the subject string, but then all matches in the subject string
    /// are replaced by the substitution string. If replacing all matches isn't
    /// desired, regular expression anchors can be used to ensure a single match,
    /// so as to replace just one occurrence of a pattern. Capture groups can be
    /// used in the pattern to extract portions of the subject string, and then
    /// referenced in the substitution string.
    #[prost(message, optional, tag = "1")]
    pub pattern: ::std::option::Option<RegexMatcher>,
    /// The string that should be substituted into matching portions of the
    /// subject string during a substitution operation to produce a new string.
    /// Capture groups in the pattern can be referenced in the substitution
    /// string. Note, however, that the syntax for referring to capture groups is
    /// defined by the chosen regular expression engine. Google's `RE2
    /// <https://github.com/google/re2>`_ regular expression engine uses a
    /// backslash followed by the capture group number to denote a numbered
    /// capture group. E.g., ``\1`` refers to capture group 1, and ``\2`` refers
    /// to capture group 2.
    #[prost(string, tag = "2")]
    pub substitution: std::string::String,
}
// [#protodoc-title: String matcher]

/// Specifies the way to match a string.
/// [#next-free-field: 7]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringMatcher {
    /// If true, indicates the exact/prefix/suffix matching should be case insensitive. This has no
    /// effect for the safe_regex match.
    /// For example, the matcher *data* will match both input string *Data* and *data* if set to true.
    #[prost(bool, tag = "6")]
    pub ignore_case: bool,
    #[prost(oneof = "string_matcher::MatchPattern", tags = "1, 2, 3, 5")]
    pub match_pattern: ::std::option::Option<string_matcher::MatchPattern>,
}
pub mod string_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchPattern {
        /// The input string must match exactly the string specified here.
        ///
        /// Examples:
        ///
        /// * *abc* only matches the value *abc*.
        #[prost(string, tag = "1")]
        Exact(std::string::String),
        /// The input string must have the prefix specified here.
        /// Note: empty prefix is not allowed, please use regex instead.
        ///
        /// Examples:
        ///
        /// * *abc* matches the value *abc.xyz*
        #[prost(string, tag = "2")]
        Prefix(std::string::String),
        /// The input string must have the suffix specified here.
        /// Note: empty prefix is not allowed, please use regex instead.
        ///
        /// Examples:
        ///
        /// * *abc* matches the value *xyz.abc*
        #[prost(string, tag = "3")]
        Suffix(std::string::String),
        /// The input string must match the regular expression specified here.
        #[prost(message, tag = "5")]
        SafeRegex(super::RegexMatcher),
    }
}
/// Specifies a list of ways to match a string.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStringMatcher {
    #[prost(message, repeated, tag = "1")]
    pub patterns: ::std::vec::Vec<StringMatcher>,
}
