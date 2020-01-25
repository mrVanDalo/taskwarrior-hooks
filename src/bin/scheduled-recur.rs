extern crate chrono;

use chrono::Duration;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::Offset;
use core::ops::Sub;
use serde_json::to_string;
use std::io;
use std::str::FromStr;

use regex::Match;
use regex::Regex;

use task_hookrs::date::Date;
use task_hookrs::import::import_task;
use task_hookrs::status::TaskStatus::*;
use task_hookrs::uda::UDAValue;

fn main() {
    let mut original = String::new();
    io::stdin().read_line(&mut original).unwrap();

    let mut modified = String::new();
    io::stdin().read_line(&mut modified).unwrap();

    let output = parse_and_render(&original, &modified);
    println!("{}", output);
}

/* return the start of today in utc, but of your local timezone */
fn today_in_utc() -> NaiveDateTime {
    Local::today()
        .naive_local()
        .and_hms(0, 0, 0)
        .sub(Local::today().offset().fix())
}

fn parse_and_render(original: &str, modified: &str) -> String {
    let mut modified_task = import_task(modified).expect(modified);
    if !modified_task.uda().contains_key("scheduled_recur") {
        return String::from(modified);
    }
    if modified_task.scheduled().is_none() {
        return String::from(modified);
    }
    if *modified_task.status() != Completed {
        return String::from(modified);
    }

    let original_task = import_task(original).unwrap();
    if *original_task.status() == Completed {
        return String::from(modified);
    }

    let offset = match modified_task.uda().get("scheduled_recur").unwrap() {
        UDAValue::Str(offset) => parse_duration(offset.as_ref()),
        _ => panic!("couldn't parse the scheduled_recur UDA"),
    };

    let new_scheduled = today_in_utc();
    modified_task.set_scheduled(new_scheduled.checked_add_signed(offset));

    modified_task
        .status_mut()
        .clone_from(original_task.status());

    let end: Option<Date> = None;
    modified_task.set_end(end);

    println!("don't completing task, just rescheduling");

    return to_string(&modified_task).unwrap();
}

fn parse_duration(offset: &str) -> Duration {
    let iso_8601 =
        Regex::new(r"P(?:(\d+)*W)*(?:(\d+)*D)*T*(?:(\d+)*H)*(?:(\d+)*M)*(?:(\d+)*S)*").unwrap();

    if !iso_8601.is_match(offset) {
        return i64::from_str(offset)
            .map(|seconds| Duration::seconds(seconds))
            .expect("could not parse scheduled as seconds nor as ISO_8601");
    }

    for caps in iso_8601.captures_iter(offset) {
        // Note that all of the unwraps are actually OK for this regex
        // because the only way for the regex to match is if all of the
        // capture groups match. This is not true in general though!
        return Duration::weeks(cap_parser(caps.get(1)))
            .checked_add(&Duration::days(cap_parser(caps.get(2))))
            .unwrap()
            .checked_add(&Duration::hours(cap_parser(caps.get(3))))
            .unwrap()
            .checked_add(&Duration::minutes(cap_parser(caps.get(4))))
            .unwrap()
            .checked_add(&Duration::seconds(cap_parser(caps.get(5))))
            .unwrap();
    }

    Duration::weeks(0)
}

#[inline]
fn cap_parser(x: Option<Match>) -> i64 {
    match x.map(|a| a.as_str()).map(|a| i64::from_str(a).ok()) {
        Some(Some(a)) => a,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("P1W"), Duration::weeks(1));
        assert_eq!(parse_duration("P2W"), Duration::weeks(2));
        assert_eq!(parse_duration("P2W1D"), Duration::days(15));
        assert_eq!(parse_duration("P1DT1H"), Duration::hours(25));
        assert_eq!(parse_duration("P14D"), Duration::days(14));
        assert_eq!(parse_duration("PT1H"), Duration::hours(1));
        assert_eq!(parse_duration("PT1H30M"), Duration::minutes(90));
        // seconds
        assert_eq!(parse_duration("1209600"), Duration::days(14));
    }

    #[test]
    fn test_reschedule() {
        let original = r#"
         {
           "id": 1,
           "description": "test",
           "entry": "20190404T212544Z",
           "scheduled": "20191219T110000Z",
           "status": "pending",
           "uuid": "03f6ff63-26e3-41ba-bd90-a5bdd1be2ea7",
           "scheduled_recur": "P1D"
         }"#;

        let modified = r#"
         {
           "id": 1,
           "description": "test",
           "entry": "20190404T212544Z",
           "scheduled": "20191219T110000Z",
           "status": "completed",
           "uuid": "03f6ff63-26e3-41ba-bd90-a5bdd1be2ea7",
           "scheduled_recur": "P1D"
         }"#;

        let mut expect_task = import_task(modified).unwrap();
        let new_scheduled = today_in_utc() + chrono::Duration::days(1);
        expect_task.set_scheduled(Some(new_scheduled));

        let result_task = import_task(parse_and_render(&original, &modified).as_ref()).unwrap();
        assert_eq!(expect_task.scheduled(), result_task.scheduled());
        assert_eq!(Pending, *result_task.status());
    }

    #[test]
    fn test_reschedule2() {
        let original = r#"
         {
           "id": 1,
           "description": "this : is a & description",
           "entry": "20190404T212544Z",
           "scheduled": "20191219T110000Z",
           "status": "pending",
           "uuid": "03f6ff63-26e3-41ba-bd90-a5bdd1be2ea7",
           "scheduled_recur": "P1DT8H"
         }"#;

        let modified = r#"
         {
           "id": 1,
           "description": "this : is a & description",
           "entry": "20190404T212544Z",
           "scheduled": "20191219T110000Z",
           "status": "completed",
           "uuid": "03f6ff63-26e3-41ba-bd90-a5bdd1be2ea7",
           "scheduled_recur": "P1DT8H"
         }"#;

        let mut expect_task = import_task(modified).unwrap();

        let new_scheduled = today_in_utc() + chrono::Duration::hours(24 + 8);
        expect_task.set_scheduled(Some(new_scheduled));

        let result_task = import_task(parse_and_render(&original, &modified).as_ref()).unwrap();

        assert_eq!(expect_task.scheduled(), result_task.scheduled());
        assert_eq!(Pending, *result_task.status());
    }

    #[test]
    fn test_reschedule3() {
        let original = r#"
         {
           "id": 1,
           "description": "this : is a & description",
           "entry": "20190404T212544Z",
           "scheduled": "20191219T110000Z",
           "status": "pending",
           "uuid": "03f6ff63-26e3-41ba-bd90-a5bdd1be2ea7",
           "scheduled_recur": "P14D"
         }"#;

        let modified = r#"
         {
           "id": 1,
           "description": "this : is a & description",
           "entry": "20190404T212544Z",
           "scheduled": "20191219T110000Z",
           "status": "completed",
           "uuid": "03f6ff63-26e3-41ba-bd90-a5bdd1be2ea7",
           "scheduled_recur": "P14D"
         }"#;

        let mut expect_task = import_task(modified).unwrap();

        let new_scheduled = today_in_utc() + chrono::Duration::days(14);
        expect_task.set_scheduled(Some(new_scheduled));

        let result_task = import_task(parse_and_render(&original, &modified).as_ref()).unwrap();

        assert_eq!(expect_task.scheduled(), result_task.scheduled());
        assert_eq!(Pending, *result_task.status());
    }

    #[test]
    fn test_unstouch() {
        let original = r#"
         {
           "id": 1,
           "description": "test",
           "entry": "20190404T212544Z",
           "scheduled": "20191219T110000Z",
           "status": "pending",
           "uuid": "03f6ff63-26e3-41ba-bd90-a5bdd1be2ea7"
         }"#;

        let modified = r#"
         {
           "id": 1,
           "description": "test",
           "entry": "20190404T212544Z",
           "scheduled": "20191219T110000Z",
           "status": "completed",
           "uuid": "03f6ff63-26e3-41ba-bd90-a5bdd1be2ea7"
         }"#;

        assert_eq!(parse_and_render(&original, &modified), modified);
    }

    #[test]
    #[ignore]
    fn test_timezone_test() {
        // $> date --utc --date=$(date +"%Y-%m-%dT00:00:00%z") +"%Y-%m-%dT%H:%M:%SZ"
        assert_eq!(today_in_utc().to_string(), "2020-01-24 11:00:00");
    }

}
