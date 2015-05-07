#![cfg(test)]

use regex::Regex;
use users;
use users::FullName;
use users::Email;
use users::ObfuscatedEmail;

#[test]
fn name_returns_the_users_full_name() {
    let subject = users::User {
        first_name: "David".to_string(),
        last_name: "Begin".to_string(),
        email: "david@example.com".to_string()
    };
    assert_eq!(subject.name(), "David Begin".to_string());
}

#[test]
fn email_returns_the_users_email() {
    let subject = users::User {
        first_name: "David".to_string(),
        last_name: "Begin".to_string(),
        email: "david@example.com".to_string()
    };
    assert_eq!(subject.email(), "david@example.com".to_string());
}

#[test]
fn obfuscated_email_returns_a_gibberish_email() {
    let subject = users::User {
        first_name: "David".to_string(),
        last_name: "Begin".to_string(),
        email: "david@example.com".to_string()
    };
    assert_eq!(subject.obfuscated_email(), "xxx@example.com".to_string());
}
