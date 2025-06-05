#!/usr/bin/env python3
import json
import sys

def flatten_and_dedupe_accounts(orig_list):
    """
    Given a list of account descriptors, return a flattened list in which:
      1. Any object of the form { "name": SOME_NAME, "accounts": [ ... ] } is replaced by its children.
      2. If multiple entries share the same "name", only the first occurrence is kept.
    """
    flat = []
    seen_names = set()

    def recurse(lst):
        for entry in lst:
            # If this entry is a wrapper (has its own "accounts" array), recurse into that array first
            if "accounts" in entry and isinstance(entry["accounts"], list):
                recurse(entry["accounts"])
            else:
                nm = entry.get("name")
                if nm not in seen_names:
                    seen_names.add(nm)
                    flat.append(entry)
                # If nm already seen, skip this entry entirely

    recurse(orig_list)
    return flat

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 flatten_idl.py idl.json", file=sys.stderr)
        sys.exit(1)

    path = sys.argv[1]
    with open(path, "r") as f:
        idl = json.load(f)

    # Walk through every instruction and replace its "accounts" list:
    for ix in idl.get("instructions", []):
        if "accounts" in ix:
            ix["accounts"] = flatten_and_dedupe_accounts(ix["accounts"])

    # Output the modified IDL
    json.dump(idl, sys.stdout, indent=2)
    print()

if __name__ == "__main__":
    main()
