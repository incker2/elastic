#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

pub mod mapping;
pub mod formats;

extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate elastic_types;

use chrono::format::{ Parsed, Item };
use chrono::offset::TimeZone;
use elastic_types::date::prelude::*;

#[derive(Default, Serialize, Deserialize)]
struct MyType {
	pub date: DateTime
}

#[derive(Default, Serialize, Deserialize)]
struct MyTypeFmtd {
	pub date: DateTime<TestDateFormat1>
}

const MYTYPE_DATE_FMT_1: &'static str = "%Y/%m/%d %H:%M:%S";
const MYTYPE_DATE_FMT_2: &'static str = "%d/%m/%Y %H:%M:%S";

//A date format based on a chrono format string
#[allow(non_camel_case_types)]
#[derive(Default)]
pub struct TestDateFormat1;
impl DateFormat for TestDateFormat1 {
	fn fmt<'a>() -> Vec<Item<'a>> {
		date_fmt!("%Y/%m/%d %H:%M:%S")
			.iter()
			.cloned()
			.collect()
	}
	fn name() -> &'static str {
		"test_date_1"
	}
}

//A date format based on an elasticsearch formart string
#[allow(non_camel_case_types)]
#[derive(Default)]
pub struct TestDateFormat2;
impl DateFormat for TestDateFormat2 {
	fn fmt<'a>() -> Vec<Item<'a>> {
		date_fmt!("yyyyMMdd")
			.iter()
			.cloned()
			.collect()
	}
	fn name() -> &'static str {
		"test_date_2"
	}
}

#[test]
fn dates_should_use_chrono_format() {
	let _dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
	let expected = _dt.format(MYTYPE_DATE_FMT_1).to_string();

	let dt = DateTime::<TestDateFormat1>::new(_dt.clone());
	let actual = dt.format();

	assert_eq!(expected, actual);
}

#[test]
fn dates_should_use_es_format() {
	let _dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
	let expected = "20150513".to_string();

	let dt = DateTime::<TestDateFormat2>::new(_dt.clone());
	let actual = dt.format();

	assert_eq!(expected, actual);
}

#[test]
fn serialise_date() {
	let date = DateTime::<BasicDateTime>::new(
		chrono::UTC.datetime_from_str(
			"13/05/2015 00:00:00", MYTYPE_DATE_FMT_2
		).unwrap()
	);

	let ser = serde_json::to_string(&date).unwrap();

	assert_eq!(r#""20150513T000000.000Z""#, ser);
}

#[test]
fn deserialise_date() {
	let date: DateTime<BasicDateTime> = serde_json::from_str(r#""20150513T000000.000Z""#).unwrap();

	assert_eq!((2015, 5, 13), (date.year(), date.month(), date.day()));
}