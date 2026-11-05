#!/usr/bin/env python3
"""Check deployment invariants on rendered manifests, without cluster access."""
import argparse
import pathlib
import re

import yaml


def validate(path, require_release=False):
    resources = list(yaml.safe_load_all(pathlib.Path(path).read_text()))
    identities = set()
    claims = set()
    for resource in resources:
        kind = resource["kind"]
        meta = resource["metadata"]
        identity = (kind, meta.get("namespace"), meta["name"])
        assert identity not in identities, f"Duplicate resource: {identity}"
        identities.add(identity)
        if kind != "PersistentVolume":
            assert meta.get("namespace") == "foodie", identity
        if kind in ("PersistentVolume", "PersistentVolumeClaim"):
            assert meta["annotations"]["argocd.argoproj.io/sync-options"] == "Prune=false,Delete=false"
        if kind == "PersistentVolume":
            assert resource["spec"]["persistentVolumeReclaimPolicy"] == "Retain"
            assert resource["spec"]["local"]["path"].startswith("/var/lib/foodie/")
        if kind == "PersistentVolumeClaim":
            claims.add(meta["name"])
        if kind in ("Deployment", "StatefulSet", "Job"):
            pod = resource["spec"]["template"]["spec"]
            assert pod["automountServiceAccountToken"] is False
            assert "hostNetwork" not in pod
            for container in pod["containers"]:
                assert "resources" in container
                image = container["image"]
                assert re.search(r"@sha256:[0-9a-f]{64}$", image) or image.endswith(":unreleased"), image
                if require_release:
                    assert not image.endswith(":unreleased"), f"Unpublished production image: {image}"
    assert claims == {"postgres", "redis", "minio"}, claims
    jobs = [r for r in resources if r["kind"] == "Job"]
    if jobs:
        assert len(jobs) == 1
        if require_release:
            assert re.fullmatch(r"migrate-[0-9a-f]{12}", jobs[0]["metadata"]["name"]), "Missing release-specific migration Job name"
        assert jobs[0]["metadata"]["annotations"]["argocd.argoproj.io/sync-wave"] == "10"
        assert jobs[0]["spec"]["template"]["spec"]["containers"][0]["args"] == ["up"]
        ingress = next(r for r in resources if r["kind"] == "Ingress")
        paths = ingress["spec"]["rules"][0]["http"]["paths"]
        assert {p["path"]: p["backend"]["service"]["name"] for p in paths} == {
            "/api": "backend", "/aws": "minio", "/": "frontend"
        }
    print(f"Validated {len(resources)} resources: {path}")


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--require-release", action="store_true")
parser.add_argument("filenames", nargs="+")
args = parser.parse_args()
for filename in args.filenames:
    validate(filename, args.require_release)
