# yaml-resume

A small Rust CLI that turns a YAML file into a clean, print-ready HTML resume.

I got tired of fighting with Word/Google Docs every time I needed to update my resume. Now I just edit a YAML file and run one command. The output looks great in a browser and prints to PDF perfectly.

## Usage

```
cargo run -- resume.yaml
```

This writes `resume.html` by default. You can specify a different output path:

```
cargo run -- resume.yaml -o output.html
```

Open the HTML in your browser, hit Ctrl+P (or Cmd+P), and save as PDF. Done.

## YAML format

Take a look at `resume.yaml` for a full working example. The structure is straightforward -- here's the gist:

```yaml
name: Your Name
title: Your Title

contact:
  email: you@example.com
  phone: "555-123-4567"
  location: Somewhere, US
  github: github.com/you

summary: >
  A sentence or two about yourself.

experience:
  - company: Some Company
    role: Your Role
    dates: Jan 2020 - Present
    highlights:
      - Did a thing
      - Did another thing

education:
  - school: Some University
    degree: B.S. Something
    gpa: "3.9"

skills:
  - category: Languages
    items: [Rust, Go, Python]

certifications:
  - name: Some Certification
    date: "2023"

awards:
  - name: Some Award
```

Every section except `name` is optional -- just include what you need.

## Building

You'll need Rust installed. Then:

```
cargo build --release
```

The binary ends up in `target/release/yaml-resume`.
