use rrule::RRuleSet;
use std::str::FromStr;

pub fn from_crash_file(id: u32, data: &[u8]) {
    println!("--------- Test file {} -----------", id);

    let data_string = std::str::from_utf8(data).unwrap();
    println!("Test {} String: \n\n{:?}\n", id, data_string);
    let result = std::panic::catch_unwind(|| {
        parse_rrule_from_string(data_string);
    });
    println!("Test {} status: {:?}", id, result);
    println!("--------- Done test {} -----------", id);
}

pub fn parse_rrule_from_string(rrule: &str) {
    match RRuleSet::from_str(rrule) {
        Ok(rrule) => {
            println!("RRule data: {:#?}", rrule);
            let (list, limited) = rrule.all(50);
            crate::print_all_datetimes(&list);
            if limited {
                println!("RRule was limited");
            }
        }
        Err(err) => println!("RRule could not be parsed: {}", err),
    };
}
