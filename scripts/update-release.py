#!/usr/bin/env python3
"""Set the three verified image digests and the release-specific migration Job."""
import argparse
import pathlib
import re

import yaml


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("release")
    parser.add_argument("artifact_dir")
    parser.add_argument("--dry-run", action="store_true", help="Print YAML without changing the overlay")
    args = parser.parse_args()
    release, artifact_dir = args.release, args.artifact_dir
    if not re.fullmatch(r"[0-9a-f]{40}", release):
        raise SystemExit("Release must be a full Git SHA")
    path = pathlib.Path(__file__).resolve().parents[1] / "deploy/kubernetes/overlays/production/kustomization.yaml"
    config = yaml.safe_load(path.read_text())
    images = []
    for component in ("backend", "frontend", "migration"):
        reference = (pathlib.Path(artifact_dir) / f"{component}.txt").read_text().strip()
        match = re.fullmatch(
            rf"(ghcr\.io/[a-z0-9_.-]+/foodie-{component})@(sha256:[0-9a-f]{{64}})", reference
        )
        if not match:
            raise SystemExit(f"Invalid immutable image reference for {component}")
        images.append({"name": f"foodie-{component}", "newName": match[1], "digest": match[2]})
    config["images"] = images
    config["patches"][0]["patch"] = (
        "- op: replace\n  path: /metadata/name\n  value: migrate-" + release[:12] + "\n"
    )
    output = yaml.safe_dump(config, sort_keys=False)
    if args.dry_run:
        print(output, end="")
    else:
        path.write_text(output)


if __name__ == "__main__":
    main()
