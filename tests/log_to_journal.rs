// Copyright Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Test that we actually log to the systemd journal,
//! and that systemd can pick all those messages up.log_

#![deny(warnings, clippy::all)]

use log::kv::Value;
use log::{Level, Log, Record};
use pretty_assertions::assert_eq;

use systemd_journal_logger::JournalLog;

mod journal;

#[test]
fn simple_log_entry() {
    let target = journal::random_target("systemd_journal_logger/simple_log_entry");

    JournalLog::new().unwrap().log(
        &Record::builder()
            .level(Level::Warn)
            .target(&target)
            .module_path(Some(module_path!()))
            .file(Some(file!()))
            .line(Some(92749))
            .args(format_args!("systemd_journal_logger test: {}", 42))
            .build(),
    );

    let entries = journal::read_current_process(module_path!(), &target);
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];

    assert_eq!(entry["TARGET"], target);
    assert_eq!(entry["PRIORITY"], "4");
    assert_eq!(entry["MESSAGE"], "systemd_journal_logger test: 42");
    assert_eq!(entry["CODE_FILE"], file!());
    assert_eq!(entry["CODE_LINE"], "92749");
    assert_eq!(entry["CODE_MODULE"], module_path!());

    assert!(entry["SYSLOG_IDENTIFIER"]
        .as_text()
        .contains("log_to_journal"));
    assert_eq!(
        entry["SYSLOG_IDENTIFIER"],
        std::env::current_exe()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    );

    assert_eq!(entry["SYSLOG_PID"], std::process::id().to_string());
    // // The PID we logged is equal to the PID systemd determined as source for our process
    assert_eq!(entry["SYSLOG_PID"], entry["_PID"]);
}

#[test]
fn internal_null_byte_in_message() {
    let target = journal::random_target("systemd_journal_logger/internal_null_byte_in_message");

    JournalLog::new().unwrap().log(
        &Record::builder()
            .level(Level::Warn)
            .target(&target)
            .module_path(Some(module_path!()))
            .args(format_args!("systemd_journal_logger with \x00 byte"))
            .build(),
    );

    let entries = journal::read_current_process(module_path!(), &target);
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];

    assert_eq!(entry["TARGET"], target);
    assert_eq!(entry["PRIORITY"], "4");
    assert_eq!(
        entry["MESSAGE"].as_text(),
        "systemd_journal_logger with \x00 byte"
    );
}

#[test]
fn multiline_message() {
    let target = journal::random_target("systemd_journal_logger/multiline_message");

    JournalLog::new().unwrap().log(
        &Record::builder()
            .level(Level::Error)
            .target(&target)
            .module_path(Some(module_path!()))
            .args(format_args!(
                "systemd_journal_logger test\nwith\nline {}",
                "breaks"
            ))
            .build(),
    );

    let entries = journal::read_current_process(module_path!(), &target);
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];

    assert_eq!(entry["TARGET"], target);
    assert_eq!(entry["PRIORITY"], "3");
    assert_eq!(
        entry["MESSAGE"],
        "systemd_journal_logger test\nwith\nline breaks"
    );
}

#[test]
fn trailing_newline_message() {
    let target = journal::random_target("systemd_journal_logger/trailing_newline_message");

    JournalLog::new().unwrap().log(
        &Record::builder()
            .level(Level::Trace)
            .target(&target)
            .module_path(Some(module_path!()))
            .args(format_args!("trailing newline\n"))
            .build(),
    );

    let entries = journal::read_current_process(module_path!(), &target);
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];

    assert_eq!(entry["TARGET"], target);
    assert_eq!(entry["PRIORITY"], "7");
    assert_eq!(entry["MESSAGE"], "trailing newline\n");
}

#[test]
fn very_large_message() {
    let target = journal::random_target("systemd_journal_logger/very_large_message");

    let very_large_string = "b".repeat(512_000);
    JournalLog::new().unwrap().log(
        &Record::builder()
            .level(Level::Trace)
            .target(&target)
            .module_path(Some(module_path!()))
            .args(format_args!("{}", very_large_string))
            .build(),
    );

    let entries = journal::read_current_process(module_path!(), &target);
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];

    assert_eq!(entry["TARGET"], target);
    assert_eq!(entry["PRIORITY"], "7");
    assert_eq!(entry["MESSAGE"].as_text(), very_large_string);
}

#[test]
fn extra_fields() {
    let target = journal::random_target("systemd_journal_logger/extra_fields");

    JournalLog::new()
        .unwrap()
        .with_extra_fields(vec![("FOO", "BAR")])
        .log(
            &Record::builder()
                .level(Level::Debug)
                .target(&target)
                .module_path(Some(module_path!()))
                .args(format_args!("with an extra field"))
                .build(),
        );

    let entries = journal::read_current_process(module_path!(), &target);
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];

    assert_eq!(entry["TARGET"], target);
    assert_eq!(entry["PRIORITY"], "6");
    assert_eq!(entry["MESSAGE"], "with an extra field");
    assert_eq!(entry["FOO"], "BAR")
}

#[test]
fn escaped_extra_fields() {
    let target = journal::random_target("systemd_journal_logger/escaped_extra_fields");

    JournalLog::new()
        .unwrap()
        .with_extra_fields(vec![
            ("Hallöchen", "Welt"),
            ("123_FOO", "BAR"),
            ("_spam", "EGGS"),
        ])
        .log(
            &Record::builder()
                .level(Level::Debug)
                .target(&target)
                .module_path(Some(module_path!()))
                .args(format_args!("with an escaped extra field"))
                .build(),
        );

    let entries = journal::read_current_process(module_path!(), &target);
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];

    assert_eq!(entry["TARGET"], target);
    assert_eq!(entry["PRIORITY"], "6");
    assert_eq!(entry["MESSAGE"], "with an escaped extra field");
    assert_eq!(entry["HALL_CHEN"], "Welt");
    assert_eq!(entry["ESCAPED_123_FOO"], "BAR");
    assert_eq!(entry["ESCAPED__SPAM"], "EGGS");
}

#[test]
fn extra_record_fields() {
    let target = journal::random_target("systemd_journal_logger/extra_record_fields");

    let kvs: &[(&str, Value)] = &[
        ("_foo", Value::from("foo")),
        ("spam_with_eggs", Value::from(false)),
    ];

    JournalLog::new()
        .unwrap()
        .with_extra_fields(vec![("EXTRA_FIELD", "foo")])
        .log(
            &Record::builder()
                .level(Level::Error)
                .target(&target)
                .module_path(Some(module_path!()))
                .args(format_args!("Hello world"))
                .key_values(&kvs)
                .build(),
        );

    let entries = journal::read_current_process(module_path!(), &target);
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];

    assert_eq!(entry["TARGET"], target);
    assert_eq!(entry["PRIORITY"], "3");
    assert_eq!(entry["MESSAGE"], "Hello world");
    assert_eq!(entry["EXTRA_FIELD"], "foo");
    assert_eq!(entry["ESCAPED__FOO"], "foo");
    assert_eq!(entry["SPAM_WITH_EGGS"], "false");
}
