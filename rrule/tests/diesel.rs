#![cfg(feature = "diesel")]
#[macro_use]
extern crate diesel;

use rrule::{RRule, RRuleProperties};

table! {
    use diesel::sql_types::*;
    test {
        id -> Int2,
        properties -> Text,
        rrule -> Text,
    }
}

#[test]
fn rrule_and_rrule_properties_to_diesel_sql() {
    // There is no assertion here
    // Just to see if the code compiles or not, and that's the test!
    #[allow(dead_code)]
    #[derive(Queryable)]
    struct Test {
        properties: RRuleProperties,
        rrule: RRule,
    }

    #[derive(Insertable)]
    #[table_name = "test"]
    struct NewTest {
        properties: RRuleProperties,
        rrule: RRule,
    }

    #[derive(AsChangeset)]
    #[table_name = "test"]
    struct PatchTest {
        properties: Option<RRuleProperties>,
        rrule: Option<RRule>,
    }
}
