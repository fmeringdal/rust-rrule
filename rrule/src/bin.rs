use rrule::{RRule, RRuleSet};

fn main() {
    // Collect args and skip the program name
    let mut args: Vec<_> = std::env::args().skip(1).collect();
    if args.is_empty() || args.len() > 1 {
        println!("Invalid input");
        return;
    }
    let rrule_str = args.remove(0).replace("\\n", "\n");
    if rrule_str.contains("EXRULE") || rrule_str.contains("RDATE") || rrule_str.contains("EXDATE") {
        let rrule_set: RRuleSet = rrule_str
            .parse()
            .expect("Input string to be valid rrule set string");
        for occurence in &rrule_set {
            println!("{}", occurence);
        }
    } else {
        let rrule: RRule = rrule_str
            .parse()
            .expect("Input string to be valid rrule string");
        for occurence in &rrule {
            println!("{}", occurence);
        }
    }
}
