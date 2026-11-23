#!/usr/bin/env python3
"""
Validate CITATION.cff against the official Citation File Format schema.

    python scripts/validate-citation.py

The schema is the one from the CFF project, pinned to the 1.2.0 tag - the same
`cff-version` this repository declares. It is fetched over HTTPS so the check
uses the canonical schema rather than a copy that could drift.

Why this exists: the "Cite this repository" widget renders CITATION.cff in APA
and in BibTeX, and it is easy to write a file that looks fine and that every
validator rejects. The CFF 1.2.0 schema permits only two `type` values -
`dataset` and `software` - so a file that says `type: standard` is invalid
even though it reads well.

Exit code 0 = valid, 1 = invalid.
"""

import json
import sys
import urllib.request

import jsonschema
import yaml

SCHEMA_URL = (
    "https://raw.githubusercontent.com/citation-file-format/"
    "citation-file-format/1.2.0/schema.json"
)
CFF = "CITATION.cff"


def main() -> int:
    try:
        with urllib.request.urlopen(SCHEMA_URL, timeout=30) as r:
            schema = json.load(r)
    except Exception as e:  # noqa: BLE001 - the message is the point
        print(f"could not fetch the CFF schema: {e}")
        print("  (a network problem, not a problem with CITATION.cff)")
        return 2

    try:
        with open(CFF, encoding="utf-8") as f:
            doc = yaml.safe_load(f)
    except Exception as e:  # noqa: BLE001
        print(f"{CFF} is not readable YAML: {e}")
        return 1

    validator = jsonschema.validators.validator_for(schema)
    try:
        validator.check_schema(schema)
    except Exception as e:  # noqa: BLE001
        print(f"the fetched schema is not a valid JSON Schema: {e}")
        return 2

    errors = sorted(validator(schema).iter_errors(doc), key=lambda e: list(e.path))
    if not errors:
        print(f"{CFF} is valid against the official CFF 1.2.0 schema.")
        print(f"  type: {doc.get('type')}   title: {doc.get('title')}")
        return 0

    for e in errors:
        where = "/".join(str(p) for p in e.path) or "(root)"
        print(f"  {where}: {e.message}")
    print(f"{CFF} is NOT valid against the official CFF 1.2.0 schema.")
    return 1


if __name__ == "__main__":
    sys.exit(main())
