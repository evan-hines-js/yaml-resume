use crate::schema::Resume;
use crate::style::CSS;

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn render(resume: &Resume) -> String {
    let mut html = String::with_capacity(8192);

    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"utf-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str(&format!("<title>{}</title>\n", escape(&resume.name)));
    html.push_str(&format!("<style>{CSS}</style>\n"));
    html.push_str("</head>\n<body>\n");

    // Header
    html.push_str("<header>\n");
    html.push_str(&format!("<h1>{}</h1>\n", escape(&resume.name)));
    if let Some(title) = &resume.title {
        html.push_str(&format!("<div class=\"title\">{}</div>\n", escape(title)));
    }
    render_contact(&mut html, resume);
    html.push_str("</header>\n");

    // Summary
    if let Some(summary) = &resume.summary {
        html.push_str("<section class=\"summary\">\n<h2>Summary</h2>\n");
        html.push_str(&format!("<p>{}</p>\n", escape(summary.trim())));
        html.push_str("</section>\n");
    }

    // Experience
    if let Some(experience) = &resume.experience {
        html.push_str("<section>\n<h2>Experience</h2>\n");
        for exp in experience {
            html.push_str("<div class=\"entry\">\n<div class=\"entry-header\">\n");
            html.push_str(&format!(
                "<span class=\"primary\">{}</span>\n",
                escape(&exp.company)
            ));
            if let Some(dates) = &exp.dates {
                html.push_str(&format!(
                    "<span class=\"dates\">{}</span>\n",
                    escape(dates)
                ));
            }
            html.push_str("</div>\n");
            let mut sub_parts = Vec::new();
            if let Some(role) = &exp.role {
                sub_parts.push(escape(role));
            }
            if let Some(location) = &exp.location {
                sub_parts.push(escape(location));
            }
            if !sub_parts.is_empty() {
                html.push_str(&format!(
                    "<div class=\"entry-sub\">{}</div>\n",
                    sub_parts.join(" · ")
                ));
            }
            if let Some(highlights) = &exp.highlights {
                html.push_str("<ul>\n");
                for h in highlights {
                    html.push_str(&format!("<li>{}</li>\n", escape(h)));
                }
                html.push_str("</ul>\n");
            }
            html.push_str("</div>\n");
        }
        html.push_str("</section>\n");
    }

    // Education
    if let Some(education) = &resume.education {
        html.push_str("<section>\n<h2>Education</h2>\n");
        for edu in education {
            html.push_str("<div class=\"entry\">\n<div class=\"entry-header\">\n");
            html.push_str(&format!(
                "<span class=\"primary\">{}</span>\n",
                escape(&edu.school)
            ));
            if let Some(dates) = &edu.dates {
                html.push_str(&format!(
                    "<span class=\"dates\">{}</span>\n",
                    escape(dates)
                ));
            }
            html.push_str("</div>\n");
            let mut sub_parts = Vec::new();
            if let Some(degree) = &edu.degree {
                sub_parts.push(escape(degree));
            }
            if let Some(gpa) = &edu.gpa {
                sub_parts.push(format!("GPA: {}", escape(gpa)));
            }
            if !sub_parts.is_empty() {
                html.push_str(&format!(
                    "<div class=\"entry-sub\">{}</div>\n",
                    sub_parts.join(" · ")
                ));
            }
            html.push_str("</div>\n");
        }
        html.push_str("</section>\n");
    }

    // Skills
    if let Some(skills) = &resume.skills {
        html.push_str("<section>\n<h2>Skills</h2>\n");
        for group in skills {
            html.push_str("<div class=\"skills-group\">\n");
            html.push_str(&format!(
                "<span class=\"category\">{}:</span>\n",
                escape(&group.category)
            ));
            for item in &group.items {
                html.push_str(&format!("<span class=\"tag\">{}</span>\n", escape(item)));
            }
            html.push_str("</div>\n");
        }
        html.push_str("</section>\n");
    }

    // Projects
    if let Some(projects) = &resume.projects {
        html.push_str("<section>\n<h2>Projects</h2>\n");
        for proj in projects {
            html.push_str("<div class=\"entry\">\n<div class=\"entry-header\">\n");
            if let Some(url) = &proj.url {
                html.push_str(&format!(
                    "<span class=\"primary\"><a href=\"{}\">{}</a></span>\n",
                    escape(url),
                    escape(&proj.name)
                ));
            } else {
                html.push_str(&format!(
                    "<span class=\"primary\">{}</span>\n",
                    escape(&proj.name)
                ));
            }
            html.push_str("</div>\n");
            if let Some(desc) = &proj.description {
                html.push_str(&format!(
                    "<div class=\"entry-sub\">{}</div>\n",
                    escape(desc)
                ));
            }
            if let Some(highlights) = &proj.highlights {
                html.push_str("<ul>\n");
                for h in highlights {
                    html.push_str(&format!("<li>{}</li>\n", escape(h)));
                }
                html.push_str("</ul>\n");
            }
            html.push_str("</div>\n");
        }
        html.push_str("</section>\n");
    }

    // Certifications
    if let Some(certs) = &resume.certifications {
        html.push_str("<section>\n<h2>Certifications</h2>\n");
        for cert in certs {
            html.push_str("<div class=\"cert\">\n");
            html.push_str(&format!(
                "<span class=\"cert-name\">{}</span>\n",
                escape(&cert.name)
            ));
            if let Some(date) = &cert.date {
                html.push_str(&format!(
                    "<span class=\"cert-date\">{}</span>\n",
                    escape(date)
                ));
            }
            html.push_str("</div>\n");
        }
        html.push_str("</section>\n");
    }

    // Awards
    if let Some(awards) = &resume.awards {
        html.push_str("<section>\n<h2>Honors &amp; Awards</h2>\n");
        for award in awards {
            html.push_str("<div class=\"cert\">\n");
            html.push_str(&format!(
                "<span class=\"cert-name\">{}</span>\n",
                escape(&award.name)
            ));
            if let Some(desc) = &award.description {
                html.push_str(&format!(
                    "<span class=\"cert-date\">{}</span>\n",
                    escape(desc)
                ));
            }
            html.push_str("</div>\n");
        }
        html.push_str("</section>\n");
    }

    html.push_str("</body>\n</html>\n");
    html
}

fn render_contact(html: &mut String, resume: &Resume) {
    let contact = match &resume.contact {
        Some(c) => c,
        None => return,
    };

    let mut items: Vec<String> = Vec::new();

    if let Some(email) = &contact.email {
        items.push(format!(
            "<a href=\"mailto:{}\">{}</a>",
            escape(email),
            escape(email)
        ));
    }
    if let Some(phone) = &contact.phone {
        items.push(escape(phone));
    }
    if let Some(location) = &contact.location {
        items.push(escape(location));
    }
    if let Some(linkedin) = &contact.linkedin {
        items.push(format!(
            "<a href=\"https://{}\">{}</a>",
            escape(linkedin),
            escape(linkedin)
        ));
    }
    if let Some(github) = &contact.github {
        items.push(format!(
            "<a href=\"https://{}\">{}</a>",
            escape(github),
            escape(github)
        ));
    }
    if let Some(website) = &contact.website {
        items.push(format!(
            "<a href=\"https://{}\">{}</a>",
            escape(website),
            escape(website)
        ));
    }

    if !items.is_empty() {
        html.push_str("<ul class=\"contact-list\">\n");
        for item in &items {
            html.push_str(&format!("<li>{item}</li>\n"));
        }
        html.push_str("</ul>\n");
    }
}
