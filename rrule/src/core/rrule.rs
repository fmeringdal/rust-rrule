use super::properties::*;
use crate::{validator::validate_properties, DateFilter, RRuleError, RRuleIter};
use std::str::FromStr;

/// A validated Recurrence Rule that can be used to create an iterator.
#[derive(Clone, Debug)]
pub struct RRule {
    /// The properties specified by this rule.
    properties: RRuleProperties,
}

impl RRule {
    /// Create and validate the given properties and make sure they are valid before
    /// creating an RRule struct.
    /// If the properties are not valid it will return an error.
    pub fn new(properties: RRuleProperties) -> Result<Self, RRuleError> {
        let datetime = properties.dt_start;
        let properties = crate::parser::finalize_parsed_properties(properties, &datetime)?;
        let validated_properties = validate_properties(properties)?;
        Ok(Self {
            properties: validated_properties,
        })
    }

    /// Get the parameters set by the RRule.
    pub fn get_properties(&self) -> &RRuleProperties {
        &self.properties
    }
}

impl FromStr for RRule {
    type Err = RRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let properties = crate::parser::parse_rrule_string_to_properties(s)?;
        RRule::new(properties)
    }
}

impl<'a> DateFilter<'a, RRuleIter<'a>> for RRule {}
