use crate::rrule::*;
use chrono::prelude::*;

struct RRuleSet {
    rrule: Vec<RRule>,
    rdate: Vec<DateTime<Utc>>,
    exrule: Vec<RRule>,
    exdate: Vec<DateTime<Utc>>,
    dtstart: Option<DateTime<Utc>>,
}

impl RRuleSet {
    pub fn new() -> Self {
        Self {
            rrule: vec![],
            rdate: vec![],
            exrule: vec![],
            exdate: vec![],
            dtstart: None,
        }
    }

    pub fn rrule(&mut self, rrule: RRule) {
        self.rrule.push(rrule);
    }

    pub fn exrule(&mut self, rrule: RRule) {
        self.exrule.push(rrule);
    }

    pub fn rdate(&mut self, rdate: DateTime<Utc>) {
        self.rdate.push(rdate);
    }

    pub fn exdate(&mut self, exdate: DateTime<Utc>) {
        self.exdate.push(exdate);
    }

    pub fn value_of(&mut self) -> Vec<String> {
        let mut result = vec![];

        if !self.rrule.is_empty() && self.dtstart.is_some() {
            //result = result.concat(optionsToString({ dtstart: this._dtstart }))
            result.push(String::from("yeah"));
        }

        for rrule in &self.rrule {
            // result = result.concat(rrule.toString().split('\n'))
            result.push(String::from("test"));
        }

        for exrule in &self.exrule {
            //result = result.concat(
            //exrule.toString().split('\n')
            //.map(line => line.replace(/^RRULE:/, 'EXRULE:'))
            //.filter(line => !/^DTSTART/.test(line))
            //)
            result.push(String::from("hi"));
        }

        if !self.rdate.is_empty() {
            //result.push(
            //rdatesToString('RDATE', this._rdate, this.tzid())
            //)
        }

        if !self.exdate.is_empty() {
            //result.push(
            //rdatesToString('EXDATE', this._exdate, this.tzid())
            //)
        }

        result
    }
}
