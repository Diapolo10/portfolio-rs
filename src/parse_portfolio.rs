use std::fs;
use std::path::PathBuf;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Portfolio {
    basics: Basics,
    work: Option<Vec<Work>>,
    volunteer: Option<Vec<Volunteer>>,
    education: Option<Vec<Education>>,
    awards: Option<Vec<Award>>,
    certificates: Option<Vec<Certificate>>,
    publications: Option<Vec<Publication>>,
    skills: Option<Vec<Skill>>,
    languages: Option<Vec<Language>>,
    interests: Option<Vec<Interest>>,
    references: Option<Vec<Reference>>,
    projects: Option<Vec<Project>>,
}

#[derive(Deserialize, Serialize)]
struct Basics {
    name: String,
    label: String,
    image_path: PathBuf,
    email: String,
    phone: String,
    url: String,
    summary: String,
    location: Location,
    profiles: Option<Vec<Profile>>,
}

#[derive(Deserialize, Serialize)]
struct Location {
    address: String,
    postal_code: String,
    city: String,
    country_code: String,
    region: String,
}

#[derive(Deserialize, Serialize)]
struct Profile {
    network: String,
    username: String,
    url: String,
}

#[derive(Deserialize, Serialize)]
struct Work {
    name: String,
    position: String,
    url: String,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    summary: String,
    highlights: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
struct Volunteer {
    organization: String,
    position: String,
    url: Option<String>,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    summary: String,
    highlights: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
struct Education {
    institution: String,
    url: Option<String>,
    area: String,
    study_type: String,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    score: String,
    courses: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
struct Award {
    title: String,
    date: NaiveDate,
    awarder: String,
    summary: String,
}

#[derive(Deserialize, Serialize)]
struct Certificate {
    name: String,
    date: NaiveDate,
    issuer: String,
    url: String,
}

#[derive(Deserialize, Serialize)]
struct Publication {
    name: String,
    publisher: String,
    release_date: NaiveDate,
    url: String,
    summary: String,
}

#[derive(Deserialize, Serialize)]
struct Skill {
    name: String,
    level: String,
    keywords: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
struct Language {
    language: String,
    fluency: String,
}

#[derive(Deserialize, Serialize)]
struct Interest {
    name: String,
    keywords: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
struct Reference {
    name: String,
    reference: String,
}

#[derive(Deserialize, Serialize)]
struct Project {
    name: String,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    description: String,
    highlights: Option<Vec<String>>,
    url: String,
}

pub fn parse(file_path: PathBuf) -> Portfolio {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let portfolio: Portfolio = toml::from_str(contents.as_str()).expect("Format invalid.");
    portfolio
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data(fname: &str) -> PathBuf {
        [env!("CARGO_MANIFEST_DIR"), "resources", "test", fname]
            .iter()
            .collect()
    }

    #[test]
    fn it_works() {
        let result = parse(test_data("portfolio.toml"));
        assert_eq!(result.basics.name, "John Doe".to_owned());
    }
}
