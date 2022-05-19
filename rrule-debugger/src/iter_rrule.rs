pub fn from_crash_file(id: u32, data: &[u8]) {
    println!("--------- Test file {} -----------", id);

    let result = std::panic::catch_unwind(|| {
        rrule_from_bin(data);
    });
    println!("Test {} status: {:?}", id, result);
    println!("--------- Done test {} -----------", id);
}

pub fn rrule_from_bin(data: &[u8]) {
    match rrule_afl_fuzz::take_rrule::take_rrule_from_data(data) {
        Some(rule) => {
            println!("RRule data: {:#?}", rule);
            let (list, err) = rule.all_with_error(50);
            crate::print_all_datetimes(list);
            if let Some(err) = err {
                println!("RRule ended with error: {}", err);
            }
        }
        None => println!("Not enough data"),
    };
}
