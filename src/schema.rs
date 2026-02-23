use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Resume {
    pub name: String,
    pub title: Option<String>,
    pub contact: Option<Contact>,
    pub summary: Option<String>,
    pub experience: Option<Vec<Experience>>,
    pub education: Option<Vec<Education>>,
    pub skills: Option<Vec<Skill>>,
    pub projects: Option<Vec<Project>>,
    pub certifications: Option<Vec<Certification>>,
    pub awards: Option<Vec<Award>>,
}

#[derive(Debug, Deserialize)]
pub struct Contact {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Experience {
    pub company: String,
    pub role: Option<String>,
    pub dates: Option<String>,
    pub location: Option<String>,
    pub highlights: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Education {
    pub school: String,
    pub degree: Option<String>,
    pub dates: Option<String>,
    pub gpa: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Skill {
    pub category: String,
    pub items: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    pub url: Option<String>,
    pub description: Option<String>,
    pub highlights: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Certification {
    pub name: String,
    pub date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Award {
    pub name: String,
    pub description: Option<String>,
}
