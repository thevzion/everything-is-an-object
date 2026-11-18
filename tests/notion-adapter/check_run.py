"""Check recorded trial evidence offline. This does not rerun Notion or certify it."""

import argparse
import difflib
import hashlib
import json
import re
import subprocess
from pathlib import Path


def normalize(text):
    return "\n".join(line.rstrip() for line in text.replace("\r\n", "\n").replace("\r", "\n").split("\n")).strip()


def parse_object(text):
    header, body = text.split("---", 2)[1:]
    fields = {}
    for line in header.strip().splitlines():
        key, value = line.split(":", 1)
        fields[key] = json.loads(value.strip())
    return fields, body.strip()


def git(space, *args):
    return subprocess.check_output(["git", "-C", str(space), *args], encoding="utf-8").strip()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("evidence", nargs="?", type=Path, default=Path(__file__).with_name("2026-11-15.json"))
    parser.add_argument("--space", type=Path, help="also compare the actual local Git fixture")
    args = parser.parse_args()
    record = json.loads(args.evidence.read_text(encoding="utf-8"))
    repo = Path(__file__).resolve().parents[2]
    types = {}
    for model in ("purpose", "goals", "work", "loop"):
        text = (repo / "models" / model / "model.md").read_text(encoding="utf-8")
        for type_id, question in re.findall(r"\|\s*`([a-z-]+)`(?: \(optional\))?\s*\|\s*([^|]+)\|", text):
            types[type_id] = (model + "@0.1", question.strip())
    errors, drifts = [], []
    objects = {f["path"]: parse_object(f["content"])[0] for f in record["final_files"] if f["path"].startswith("objects/")}
    for path, fields in objects.items():
        required = {"type", "model", "question", "source", "lifecycle", "evidence", "date"}
        if not required <= fields.keys():
            errors.append(path + ": missing header fields")
        if types.get(fields["type"]) != (fields["model"], fields["question"]):
            errors.append(path + ": model or question mismatch")
        if fields["source"] not in ("from-me", "inferred"):
            errors.append(path + ": source")
        if fields["lifecycle"] not in ("proposed", "active", "review", "done", "dropped"):
            errors.append(path + ": lifecycle")
        if fields["evidence"] not in ("by-construction", "sourced", "observed", "hypothesis", "open"):
            errors.append(path + ": evidence")
        if not re.fullmatch(r"\d{4}-\d{2}-\d{2}", fields["date"]):
            errors.append(path + ": date")
        if fields["type"] not in ("problem", "why"):
            if fields.get("serves") not in objects or fields.get("serves-source") != "inferred" or not fields.get("serves-model"):
                errors.append(path + ": serves or link provenance")
    for item in record["baseline"]:
        fields, body = parse_object(item["content"])
        props = item["notion_properties"]
        for key in ("type", "model", "question", "source", "lifecycle", "evidence"):
            if fields[key] != props[key.title()]:
                errors.append(item["path"] + ": projected " + key)
        if props["date:Date:start"] != fields["date"]:
            errors.append(item["path"] + ": projected date")
        if props["Serves"] != ([fields["serves"]] if fields.get("serves") else []):
            errors.append(item["path"] + ": projected serves")
        local, remote = normalize(body), normalize(item["notion_body"])
        if local != remote:
            drifts.append({"path": item["path"], "local_sha256": hashlib.sha256(local.encode()).hexdigest(),
                           "notion_sha256": hashlib.sha256(remote.encode()).hexdigest(),
                           "diff": list(difflib.unified_diff(local.splitlines(), remote.splitlines(), fromfile="local", tofile="notion", lineterm=""))})
    commits = {c["hash"]: c for c in record["commits"]}
    rows = {c["hash"]: c for c in record["projected_changes"]}
    if len(commits) != len(record["commits"]) or len(rows) != len(record["projected_changes"]) or commits.keys() != rows.keys():
        errors.append("commit fingerprints missing or duplicated")
    for key, commit in commits.items():
        row = rows.get(key, {})
        for field in ("message", "why", "source", "author", "status"):
            if not commit[field] or row.get(field) != commit[field]:
                errors.append(key + ": " + field)
        expected = {f for f in commit["files"] if f.startswith("objects/")}
        if expected != set(row.get("objects", [])):
            errors.append(key + ": changed object relations")
        if not re.match(r"\w+\((add|revise|merge)\): ", commit["message"]):
            errors.append(key + ": subject")
    if not all(c["propertiesMatch"] and c["servesMatches"] and c["uriMatches"] for c in record["property_checks"]):
        errors.append("final property comparison")
    if args.space:
        for f in record["final_files"]:
            if normalize(git(args.space, "show", "main:" + f["path"])) != normalize(f["content"]):
                errors.append(f["path"] + ": recorded source differs from Git")
        if set(git(args.space, "rev-list", "--all").splitlines()) != commits.keys():
            errors.append("actual Git commit set differs")
        for branch, key in (("main", "local"), ("notion/main", "remote")):
            if git(args.space, "rev-parse", branch) != record["conflict"][key]:
                errors.append(branch + ": conflict history differs")
        if git(args.space, "status", "--porcelain"):
            errors.append("local fixture is dirty")
        if git(args.space, "ls-files", ".local"):
            errors.append("machine state was committed")
    result = {"evidence_consistency": "held" if not errors else "failed", "errors": errors,
              "objects_checked": len(objects), "serves_relations": sum("serves" in f for f in objects.values()),
              "baseline_bodies": len(record["baseline"]), "normalized_equal": len(record["baseline"]) - len(drifts),
              "body_drifts": drifts, "commits_and_rows": len(commits),
              "empty_read_fields": sum(not row["read"] for row in rows.values()),
              "conflict": record["conflict"], "body_drift_detected": record["body_probe"]["base"] != record["body_probe"]["remote"]}
    print(json.dumps(result, indent=2))
    raise SystemExit(bool(errors))


if __name__ == "__main__":
    main()
