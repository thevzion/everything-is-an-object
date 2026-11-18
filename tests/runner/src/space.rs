//! A fixture space, built the same way every time, from the convention as it stands in this
//! repository: its `AGENTS.md` is generated from the norm texts of `convention/roles.md` and the
//! doctrine of `convention/doctrine.md`, and the questions come from `models/`.

use crate::util::*;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// What the convention says, read from this repository at run time.
#[allow(dead_code)]
pub struct Convention {
    pub root: PathBuf,
    pub version: String,
    /// (model@version, type) -> question, from `models/*/model.md`.
    pub questions: BTreeMap<(String, String), String>,
    pub doctrine: String,
    pub norms: BTreeMap<String, String>,
}

impl Convention {
    pub fn load(root: &Path) -> Res<Convention> {
        let doctrine_md = read(&root.join("convention/doctrine.md"));
        let doctrine = code_block(&doctrine_md).ok_or("no doctrine block in convention/doctrine.md")?;
        let roles = read(&root.join("convention/roles.md"));
        let mut norms = BTreeMap::new();
        for key in [
            "The frame sentence, after the title",
            "The last sentence of section 2",
            "Section 3. Make it explicit",
            "Section 4. Answer and propose",
            "Section 5. Delegate",
            "Section 6. Build on the objects",
            "Section 7. Keep every change",
            "The last sentence of section 9",
        ] {
            let text = section(&roles, &format!("#### {key}")).ok_or(format!("no norm text '{key}' in roles.md"))?;
            norms.insert(key.to_string(), text);
        }
        let readme = read(&root.join("convention/README.md"));
        let version = readme
            .lines()
            .find_map(|l| l.strip_prefix("Convention ").map(|r| r.split_whitespace().next().unwrap_or("").to_string()))
            .unwrap_or_else(|| "unknown".into());
        let mut questions = BTreeMap::new();
        for id in ["purpose", "goals", "work", "loop", "knowledge", "agreements", "capital"] {
            let text = read(&root.join(format!("models/{id}/model.md")));
            let (h, body) = parse_header(&text).ok_or(format!("no header in models/{id}/model.md"))?;
            let model = format!("{}@{}", first(&h, "model"), first(&h, "version"));
            let mut in_types = false;
            for line in body.lines() {
                if line.starts_with("## ") {
                    in_types = line.starts_with("## Types");
                    continue;
                }
                if !in_types || !line.starts_with("| `") {
                    continue;
                }
                let cells: Vec<&str> = line.split('|').map(|c| c.trim()).collect();
                if cells.len() < 3 {
                    continue;
                }
                let ty = cells[1].trim_matches('`').split('`').next().unwrap_or("").to_string();
                let q = cells[2].to_string();
                if q.ends_with('?') {
                    questions.insert((model.clone(), ty), q);
                }
            }
        }
        Ok(Convention { root: root.to_path_buf(), version, questions, doctrine, norms })
    }

    pub fn question_of(&self, model: &str, ty: &str) -> Option<&String> {
        self.questions.get(&(model.to_string(), ty.to_string()))
    }

    pub fn all_questions(&self) -> Vec<(String, String, String)> {
        self.questions.iter().map(|((m, t), q)| (m.clone(), t.clone(), q.clone())).collect()
    }
}

fn code_block(md: &str) -> Option<String> {
    let start = md.find("```\n")? + 4;
    let end = md[start..].find("\n```")? + start;
    Some(md[start..end].to_string())
}

/// The text under a heading, up to the next heading of the same or a higher level.
fn section(md: &str, heading: &str) -> Option<String> {
    let level = heading.chars().take_while(|c| *c == '#').count();
    let mut out = Vec::new();
    let mut inside = false;
    let mut in_code = false;
    for line in md.lines() {
        if inside {
            if line.starts_with("```") {
                in_code = !in_code;
            }
            if !in_code && line.starts_with('#') {
                let l = line.chars().take_while(|c| *c == '#').count();
                if l <= level {
                    break;
                }
            }
            out.push(line);
        } else if line.trim_end() == heading {
            inside = true;
        }
    }
    if !inside {
        return None;
    }
    Some(out.join("\n").trim().to_string())
}

/// One object of a fixture: path, header fields in order, body.
pub struct Obj<'a> {
    pub path: &'a str,
    pub ty: &'a str,
    pub model: &'a str,
    pub serves: &'a [&'a str],
    pub source: &'a str,
    pub lifecycle: &'a str,
    pub evidence: &'a str,
    pub body: &'a str,
}

impl Convention {
    pub fn render(&self, o: &Obj) -> String {
        let q = if o.model == "local" {
            String::new()
        } else {
            self.question_of(o.model, o.ty).cloned().unwrap_or_else(|| panic!("no question for {} {}", o.model, o.ty))
        };
        let mut s = format!("---\ntype: {}\nmodel: {}\nquestion: {}\n", o.ty, o.model, q);
        match o.serves.len() {
            0 => {}
            1 => s.push_str(&format!("serves: {}\n", o.serves[0])),
            _ => {
                s.push_str("serves:\n");
                for x in o.serves {
                    s.push_str(&format!("  - {x}\n"));
                }
            }
        }
        s.push_str(&format!(
            "source: {}\nlifecycle: {}\nevidence: {}\ndate: 2026-11-01\n---\n\n{}\n",
            o.source, o.lifecycle, o.evidence, o.body.trim()
        ));
        s
    }
}

/// What a space is about, for its `space.md` and `AGENTS.md`.
pub struct SpaceInfo<'a> {
    pub name: &'a str,
    pub uri: &'a str,
    pub title: &'a str,
    pub models: &'a [&'a str],
    pub members: &'a str,
    pub deciders: &'a str,
    pub proposers: &'a str,
    pub why_line: &'a str,
    /// (label, text, path) for section 1.
    pub why: Vec<(&'a str, &'a str, &'a str)>,
    /// (text, path) for section 2.
    pub goals: Vec<(&'a str, &'a str)>,
    pub sites: Vec<(&'a str, &'a str)>,
}

impl Convention {
    pub fn space_md(&self, s: &SpaceInfo) -> String {
        let mut m = format!("---\nspace: {}\nuri: {}\nconvention: {}\nmodels:\n", s.name, s.uri, self.version);
        for x in s.models {
            m.push_str(&format!("  - {x}\n"));
        }
        m.push_str("adapters:\n  - folder+git\n---\n\n");
        m.push_str(&format!("# {}\n\n## Truth\n\n| Where | Role |\n|---|---|\n| this folder | Source |\n| AGENTS.md | Projection |\n\n", s.title));
        m.push_str(&format!(
            "## Access\n\n| Verb | Who |\n|---|---|\n| read | {} |\n| propose | {} |\n| decide | {} |\n",
            s.members, s.proposers, s.deciders
        ));
        m
    }

    /// The ten sections, as `convention/roles.md` defines them, with its norm texts word for word.
    pub fn agents_md(&self, s: &SpaceInfo) -> String {
        let n = |k: &str| self.norms.get(k).cloned().unwrap_or_default();
        let mut a = format!(
            "<!-- Projection · convention {} · {} · {} · 2026-11-01 -->\n# {}\n\n{}\n\n{}\n\n",
            self.version,
            s.models.join(", "),
            s.uri,
            s.title,
            s.why_line,
            n("The frame sentence, after the title")
        );
        a.push_str("## 1. Why we are here\n<!-- Why does this space exist? · objects/ -->\n\n");
        for (label, text, path) in &s.why {
            a.push_str(&format!("- {label}: {text} (`{path}`)\n"));
        }
        a.push_str("\n## 2. What we want now\n<!-- What do we want now, and how will we know? · objects/goals/ -->\n\n");
        for (i, (text, path)) in s.goals.iter().enumerate() {
            a.push_str(&format!("{}. {text} (`{path}`)\n", i + 1));
        }
        a.push_str(&format!("\n{}\n\n", n("The last sentence of section 2")));
        for (n, heading, question) in [
            (3, "Make it explicit", "How does what is said, decided and made become objects?"),
            (4, "Answer and propose", "How do we answer, propose, and wait for a yes?"),
            (5, "Delegate", "Who does what, and how is work handed over?"),
            (6, "Build on the objects", "How does what is built here stay on the source?"),
            (7, "Keep every change", "How is a change recorded, and corrected?"),
        ] {
            a.push_str(&format!("## {n}. {heading}\n<!-- {question} · convention -->\n\n{}\n\n", self.norms.get(&format!("Section {n}. {heading}")).cloned().unwrap_or_default()));
        }
        a.push_str(&format!(
            "## 8. Who may do what\n<!-- Who may read, propose, decide, and act for whom? · space.md -->\n\n| Verb | Who |\n|---|---|\n| read | {} |\n| propose | {} |\n| decide | {} |\n\n",
            s.members, s.proposers, s.deciders
        ));
        a.push_str("## 9. Where things are\n<!-- Where is the source, and what is a copy? · space.md -->\n\n- Source: this folder, `space.md` and `objects/`.\n- Projection: this file.\n");
        for (name, path) in &s.sites {
            a.push_str(&format!("- Site: {name} (`{path}`).\n"));
        }
        a.push_str(&format!("\n{}\n\n", n("The last sentence of section 9")));
        a.push_str("## 10. This space's own rules\n<!-- What does this space add? · objects/decisions/, objects/principles/, objects/rules/ -->\n\nNone yet.\n");
        a
    }

    /// Writes `space.md`, `AGENTS.md`, `CLAUDE.md` and `.gitignore` into `dir`.
    /// The template every space's `AGENTS.md` follows: the norm texts in place, and each part that
    /// comes from the space shown in angle brackets, with its source. Written to
    /// `setup/AGENTS.template.md` by `convention-trials template`; a self-test keeps it in sync.
    pub fn agents_template(&self) -> String {
        let s = SpaceInfo {
            name: "{handle}",
            uri: "{the space's URI}",
            title: "{The space's name}",
            models: &["{the models it installs, with their versions}"],
            members: "{who may read}",
            deciders: "{who may decide}",
            proposers: "{who may propose}",
            why_line: "{Why the space exists, in one line, from objects/why.md}",
            why: vec![
                ("Why", "{why the space exists}", "objects/why.md"),
                ("Problem", "{the problem it answers}", "objects/problem.md"),
                ("Vision", "{what the world looks like when it is solved}", "objects/vision.md"),
            ],
            goals: vec![("{a goal, and how we will know}", "objects/goals/{name}.md")],
            sites: vec![("{another space it points to}", "objects/sites/{name}.md")],
        };
        let a = self.agents_md(&s);
        let a = a.replacen(" · 2026-11-01 -->", " · {date} -->", 1);
        let a = a.replacen(
            "objects/rules/ -->\n\nNone yet.\n",
            "objects/rules/ -->\n\n- {a rule of this space} (`objects/rules/{name}.md`)\n",
            1,
        );
        format!(
            "<!-- The template of every AGENTS.md, generated from convention/roles.md by `convention-trials template`; never edited by hand. What is in braces comes from the space's own objects. -->\n{a}"
        )
    }

    pub fn write_space_files(&self, dir: &Path, s: &SpaceInfo) -> Res<()> {
        write(&dir.join("space.md"), &self.space_md(s))?;
        write(&dir.join("AGENTS.md"), &self.agents_md(s))?;
        write(&dir.join("CLAUDE.md"), "@AGENTS.md\n")?;
        write(&dir.join(".gitignore"), ".local/\n")?;
        Ok(())
    }
}

/// An object found in a space after a run.
#[derive(Debug, Clone)]
pub struct Found {
    pub path: String,
    pub header: BTreeMap<String, Vec<String>>,
    pub body: String,
    pub raw: String,
}

/// Every object file under `objects/`, except sites and local type definitions.
pub fn objects(space: &Path) -> Vec<Found> {
    let mut out = Vec::new();
    for rel in walk(space, &space.join("objects")) {
        if !rel.ends_with(".md") || rel.starts_with("objects/sites/") || rel.starts_with("objects/types/") {
            continue;
        }
        let raw = read(&space.join(&rel));
        let (header, body) = parse_header(&raw).unwrap_or_default();
        out.push(Found { path: rel, header, body, raw });
    }
    out
}

pub fn local_types(space: &Path) -> Vec<Found> {
    let mut out = Vec::new();
    for rel in walk(space, &space.join("objects/types")) {
        if !rel.ends_with(".md") {
            continue;
        }
        let raw = read(&space.join(&rel));
        let (header, body) = parse_header(&raw).unwrap_or_default();
        out.push(Found { path: rel, header, body, raw });
    }
    out
}

/// The question a local type file declares: its `question:` header, or a `question:` line.
pub fn local_question(t: &Found) -> String {
    let q = first(&t.header, "question");
    if !q.is_empty() {
        return q;
    }
    for line in t.raw.lines() {
        let l = line.trim().trim_start_matches(['-', '*', ' ']).trim();
        let lower = l.to_lowercase();
        if let Some(rest) = lower.strip_prefix("question:") {
            let start = l.len() - rest.len();
            return l[start..].trim().trim_matches(['*', '"', '`']).trim().to_string();
        }
    }
    String::new()
}

/// Does this object serve nothing yet: no `serves`, and not a root? Allowed since pass 2, and named
/// in each header check's detail, so the first version of the criteria can still be read.
pub fn serves_nothing(o: &Found) -> bool {
    let ty = first(&o.header, "type");
    !o.header.is_empty() && !["problem", "why", "philosophy"].contains(&ty.as_str()) && o.header.get("serves").map_or(true, |v| v.is_empty())
}

/// Is this object's header what the convention asks? Returns the faults found, empty when none.
pub fn header_faults(c: &Convention, space: &Path, o: &Found) -> Vec<String> {
    let mut f = Vec::new();
    if o.header.is_empty() {
        return vec!["no header".into()];
    }
    for k in ["type", "model", "question", "source", "lifecycle", "evidence", "date"] {
        if first(&o.header, k).is_empty() {
            f.push(format!("no {k}"));
        }
    }
    let ty = first(&o.header, "type");
    let model = first(&o.header, "model");
    let q = first(&o.header, "question");
    let source = first(&o.header, "source");
    if !source.is_empty() && source != "from-me" && source != "inferred" {
        f.push(format!("source '{source}'"));
    }
    let lc = first(&o.header, "lifecycle");
    if !lc.is_empty() && !["proposed", "active", "review", "done", "dropped"].contains(&lc.as_str()) {
        f.push(format!("lifecycle '{lc}'"));
    }
    let ev = first(&o.header, "evidence");
    let ev0 = ev.split_whitespace().next().unwrap_or("");
    if !ev.is_empty()
        && !["by-construction", "sourced", "observed", "hypothesis", "open", "common-ground", "contested"].contains(&ev0)
    {
        f.push(format!("evidence '{ev}'"));
    }
    // Since pass 2, an object may serve nothing yet: a missing `serves` is no longer a fault
    // (version 2 of the header criteria). `serves_nothing` names those objects, for the record.
    if model == "local" {
        let tf = space.join(format!("objects/types/{ty}.md"));
        if !tf.exists() {
            f.push(format!("local type objects/types/{ty}.md missing"));
        } else {
            let raw = read(&tf);
            let (h, b) = parse_header(&raw).unwrap_or_default();
            let lq = local_question(&Found { path: String::new(), header: h, body: b, raw });
            if lq.trim() != q.trim() {
                f.push(format!("question differs from its local type's ('{lq}')"));
            }
        }
    } else if !model.is_empty() {
        match c.question_of(&model, &ty) {
            Some(expected) if expected.trim() == q.trim() => {}
            Some(expected) => f.push(format!("question not word for word: expected '{expected}'")),
            None => f.push(format!("no type '{ty}' in {model}")),
        }
    }
    f
}
