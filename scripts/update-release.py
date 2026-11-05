#!/usr/bin/env python3
"""Set the three verified image digests and the release-specific migration Job."""
import argparse
import io
import pathlib
import re

from ruamel.yaml import YAML
from ruamel.yaml.scalarstring import LiteralScalarString


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
    yaml = YAML()
    yaml.preserve_quotes = True
    yaml.indent(mapping=2, sequence=4, offset=2)
    yaml.width = 4096  # Keep immutable image references on a single line.
    config = yaml.load(path.read_text())
    images = {image["name"]: image for image in config["images"]}
    for component in ("backend", "frontend", "migration"):
        reference = (pathlib.Path(artifact_dir) / f"{component}.txt").read_text().strip()
        match = re.fullmatch(
            rf"(ghcr\.io/[a-z0-9_.-]+/foodie-{component})@(sha256:[0-9a-f]{{64}})", reference
        )
        if not match:
            raise SystemExit(f"Invalid immutable image reference for {component}")
        image = images[f"foodie-{component}"]
        image["newName"] = match[1]
        image.pop("newTag", None)
        image["digest"] = match[2]
    config["patches"][0]["patch"] = LiteralScalarString(
        "- op: replace\n  path: /metadata/name\n  value: migrate-" + release[:12]
    )
    stream = io.StringIO()
    yaml.dump(config, stream)
    output = stream.getvalue()
    if args.dry_run:
        print(output, end="")
    else:
        path.write_text(output)


if __name__ == "__main__":
    main()
