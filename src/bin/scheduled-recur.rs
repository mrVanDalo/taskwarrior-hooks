extern crate chrono;

use chrono::Duration;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use serde_json::to_string;
use std::io;
use std::str::FromStr;

use regex::Match;
use regex::Regex;

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

fn parse_and_render(original: &str, modified: &str) -> String {
    let mut modified_task = import_task(modified).unwrap();
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
        UDAValue::Str(offset) => parse_duration(offset.as_ref()).unwrap(),
        _ => panic!("penis"),
    };

    let now = Local::today();
    let new_scheduled = (now).naive_utc();
    modified_task.set_scheduled(
        NaiveDateTime::new(new_scheduled, NaiveTime::from_hms(0, 0, 0)).checked_add_signed(offset),
    );

    return to_string(&modified_task).unwrap();
}

fn parse_duration(offset: &str) -> Result<Duration, ()> {
    let re =
        Regex::new(r"P(?:(\d+)*W)*(?:(\d+)*D)*T*(?:(\d+)*H)*(?:(\d+)*M)*(?:(\d+)*S)*").unwrap();

    for caps in re.captures_iter(offset) {
        // Note that all of the unwraps are actually OK for this regex
        // because the only way for the regex to match is if all of the
        // capture groups match. This is not true in general though!
        return Ok(Duration::weeks(cap_parser(caps.get(1)))
            .checked_add(&Duration::days(cap_parser(caps.get(2))))
            .unwrap()
            .checked_add(&Duration::hours(cap_parser(caps.get(3))))
            .unwrap()
            .checked_add(&Duration::minutes(cap_parser(caps.get(4))))
            .unwrap()
            .checked_add(&Duration::seconds(cap_parser(caps.get(5))))
            .unwrap());
    }

    Ok(Duration::weeks(0))
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
        assert_eq!(parse_duration("P1W").unwrap(), Duration::weeks(1));
        assert_eq!(parse_duration("P2W").unwrap(), Duration::weeks(2));
        assert_eq!(parse_duration("P2W1D").unwrap(), Duration::days(15));
        assert_eq!(parse_duration("P1DT1H").unwrap(), Duration::hours(25));
        assert_eq!(parse_duration("PT1H").unwrap(), Duration::hours(1));
        assert_eq!(parse_duration("PT1H30M").unwrap(), Duration::minutes(90));
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

        let now = Local::today();
        let new_scheduled = (now + chrono::Duration::days(1)).naive_utc();
        expect_task.set_scheduled(Some(NaiveDateTime::new(
            new_scheduled,
            NaiveTime::from_hms(0, 0, 0),
        )));

        let result_task = import_task(parse_and_render(&original, &modified).as_ref()).unwrap();

        assert_eq!(expect_task.scheduled(), result_task.scheduled());
    }

    #[test]
    fn test_reschedule2() {
        let original = r#"
         {
           "id": 1,
           "description": "test",
           "entry": "20190404T212544Z",
           "scheduled": "20191219T110000Z",
           "status": "pending",
           "uuid": "03f6ff63-26e3-41ba-bd90-a5bdd1be2ea7",
           "scheduled_recur": "P1DT8H"
         }"#;

        let modified = r#"
         {
           "id": 1,
           "description": "test",
           "entry": "20190404T212544Z",
           "scheduled": "20191219T110000Z",
           "status": "completed",
           "uuid": "03f6ff63-26e3-41ba-bd90-a5bdd1be2ea7",
           "scheduled_recur": "P1DT8H"
         }"#;

        let mut expect_task = import_task(modified).unwrap();

        let now = Local::today();
        let new_scheduled = (now + chrono::Duration::days(1)).naive_utc();
        expect_task.set_scheduled(Some(NaiveDateTime::new(
            new_scheduled,
            NaiveTime::from_hms(8, 0, 0),
        )));

        let result_task = import_task(parse_and_render(&original, &modified).as_ref()).unwrap();

        assert_eq!(expect_task.scheduled(), result_task.scheduled());
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

}
