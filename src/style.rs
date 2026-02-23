pub const CSS: &str = r#"
*,
*::before,
*::after {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

:root {
    --accent: #2d5986;
    --text: #2c2c2c;
    --muted: #5a5a5a;
    --border: #d0d7de;
    --bg: #ffffff;
    --tag-bg: #eef2f7;
}

html {
    font-size: 15px;
}

body {
    font-family: "Charter", "Bitstream Charter", "Sitka Text", Cambria, serif;
    color: var(--text);
    background: var(--bg);
    line-height: 1.5;
    max-width: 800px;
    margin: 0 auto;
    padding: 48px 40px;
}

a {
    color: var(--accent);
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}

/* Header */
header {
    text-align: center;
    margin-bottom: 28px;
}

header h1 {
    font-size: 2rem;
    font-weight: 700;
    letter-spacing: 0.02em;
    color: var(--text);
    margin-bottom: 2px;
}

header .title {
    font-size: 1.1rem;
    color: var(--muted);
    margin-bottom: 10px;
}

.contact-list {
    list-style: none;
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 6px 0;
    font-size: 0.9rem;
    color: var(--muted);
}

.contact-list li + li::before {
    content: "·";
    margin: 0 8px;
    color: var(--border);
}

/* Sections */
section {
    margin-bottom: 24px;
}

section h2 {
    font-size: 0.85rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: var(--accent);
    border-bottom: 1.5px solid var(--border);
    padding-bottom: 4px;
    margin-bottom: 14px;
}

/* Summary */
.summary p {
    color: var(--muted);
    font-size: 0.95rem;
}

/* Entry (experience, education, projects) */
.entry {
    margin-bottom: 16px;
}

.entry:last-child {
    margin-bottom: 0;
}

.entry-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    flex-wrap: wrap;
    gap: 4px 16px;
}

.entry-header .primary {
    font-weight: 700;
    font-size: 1rem;
}

.entry-header .secondary {
    color: var(--muted);
    font-size: 0.92rem;
}

.entry-header .dates {
    font-size: 0.88rem;
    color: var(--muted);
    white-space: nowrap;
}

.entry-sub {
    font-size: 0.9rem;
    color: var(--muted);
    margin-top: 1px;
}

.entry ul {
    margin-top: 6px;
    margin-left: 18px;
    font-size: 0.93rem;
}

.entry ul li {
    margin-bottom: 3px;
}

/* Skills */
.skills-group {
    margin-bottom: 10px;
}

.skills-group:last-child {
    margin-bottom: 0;
}

.skills-group .category {
    font-weight: 700;
    font-size: 0.9rem;
    margin-right: 8px;
}

.tag {
    display: inline-block;
    background: var(--tag-bg);
    color: var(--text);
    font-size: 0.82rem;
    padding: 2px 10px;
    border-radius: 3px;
    margin: 2px 4px 2px 0;
}

/* Certifications */
.cert {
    display: flex;
    justify-content: space-between;
    font-size: 0.93rem;
    margin-bottom: 4px;
}

.cert .cert-name {
    font-weight: 600;
}

.cert .cert-date {
    color: var(--muted);
}

/* Print */
@media print {
    html {
        font-size: 13px;
    }

    body {
        padding: 0;
        max-width: none;
    }

    a {
        color: var(--text);
    }

    section {
        break-inside: avoid;
    }

    .entry {
        break-inside: avoid;
    }
}
"#;
