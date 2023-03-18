pub fn from_crash_file(id: u32, data: &[u8]) {
    println!("--------- Test file {id} -----------");

    let result = std::panic::catch_unwind(|| {
        rrule_from_bin(data);
    });
    println!("Test {id} status: {result:?}");
    println!("--------- Done test {id} -----------");
}

pub fn rrule_from_bin(data: &[u8]) {
    match rrule_afl_fuzz::take_rrule::take_rrule_from_data(data) {
        Some(rule) => {
            println!("RRule data: {rule:#?}");
            let result = rule.all(50);
            crate::print_all_datetimes(&result.dates);
            if result.limited {
                println!("RRule was limited");
            }
        }
        None => println!("Not enough data"),
    };
}
