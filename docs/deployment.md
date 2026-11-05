# Foodie deployment

## Architecture and ownership

Foodie runs in namespace `foodie` on the existing single-node k3s cluster.
Traefik, cert-manager and ESO CRDs/webhook are shared infrastructure. Foodie has
its own namespace-scoped ESO controller, controller class `foodie`, and a
1Password service account limited to read access to `foodie-production`.

| Repository | Owns |
| --- | --- |
| `seblyng/foodie` | App images, CI, manifests, retained local PVs/PVCs, SecretStore/ExternalSecret |
| `seblyng/plex-docker` | Foodie AppProject/Applications, namespace, certificate, scoped ESO controller |

Traefik preserves the incoming Host and paths:

- `/api` → backend port 42069, including `/api/ws` WebSockets.
- `/aws` → MinIO port 9000, bucket `aws`; **no path stripping or host rewriting**.
- `/` → frontend port 8080, Caddy static files with SPA fallback.

The S3 SDK signs browser requests against `https://foodie.lyngwinther.com`, not
an internal service name. Backend object deletion uses that endpoint too, so
Pod-to-public-domain connectivity must work (LAN DNS/hairpin NAT as appropriate).
HTTP redirects to HTTPS through the existing Traefik entrypoint configuration.
PostgreSQL, Redis and the MinIO console have no public route or host port.

The backend remains one replica because WebSocket fanout is process-local.
It uses Recreate updates to avoid simultaneously running independent connection
maps. Brief application downtime during a release is expected. SIGTERM initiates
graceful shutdown; Kubernetes bounds shutdown, including open sockets, to 30s.
The existing `/api/health-check` checks HTTP responsiveness, not downstream
database/Redis availability; functional checks are still needed after releases.
The session store uses Redis's reconnecting connection manager, so a Redis Pod
restart does not require restarting the backend. Connection/serialization errors
are returned through the session-store interface rather than panicking. Command
and connection timeouts, TCP keepalive, and Linux TCP_USER_TIMEOUT bound failures
when a replaced Pod leaves an old Service connection unresponsive.

## First deployment

Follow [migration.md](migration.md) before syncing production. The checked-in
Argo CD Application starts with `overlays/data-only` and automatic sync disabled.
Production image references deliberately start as `unreleased`: the first image
publication/release PR must replace them before production is synced.
After that PR is merged, render production and run the manifest checker with
`--require-release` to reject any remaining unpublished image references.

## Images and releases

CI tests the backend against disposable PostgreSQL and builds the actual release
WASM frontend image. Backend/migration use Rust 1.94.0, the frontend app uses its
existing `nightly-2025-08-01`, and Trunk is pinned. Cargo uses committed lockfiles.
Container build context excludes data, environment files, `.aws`, Git and targets.

The legacy MinIO release disappeared from its public registries during migration.
`Mirror legacy MinIO` bootstraps a GHCR copy of the exact cached image using the
`minio-source-2025-09-07` draft release asset. The workflow verifies both the
archive checksum and original image ID before publishing; it contains no runtime
credentials or application data. Keep that recovery asset available. Subsequent
deployments pull the digest-pinned GHCR image directly.

After successful CI on `master`, `Publish release` builds three `linux/amd64`
images using the tested source SHA:

```
ghcr.io/seblyng/foodie-backend
ghcr.io/seblyng/foodie-frontend
ghcr.io/seblyng/foodie-migration
```

The release job opens/updates `release/production`, setting immutable digests and
a migration Job name `migrate-<12-character-source-sha>`. It rejects builds whose
application/build sources have since changed on master. CI ignores deploy/docs-only
pushes, so merging the release PR does not start another image publication.
The release job itself validates rendered manifests: GitHub suppresses new CI
events for PRs created with `GITHUB_TOKEN`.

Repository setup, when you are ready to publish:

1. Allow GitHub Actions to create pull requests in repository Actions settings.
2. Allow `GITHUB_TOKEN` the declared package/contents/PR permissions.
3. Make the three GHCR packages public after first publication, or add a private
   registry pull Secret and `imagePullSecrets` to **all three** app Pod templates
   (frontend, backend, migration). Private credentials should be supplied out of
   band, not committed. Public images are the default configuration.
4. Connect `https://github.com/seblyng/foodie.git` in Argo CD. The repository is
   public; private repositories require separate read-only credentials.
5. Review migrations and merge the release PR.

After initial cutover, enable automated sync with `prune: false, selfHeal: true`
in `~/plex-docker/bootstrap/applications/foodie.yaml`, then apply the bootstrap
Application definition. Bootstrap definitions are manually managed in this
cluster. Keep auto-sync disabled until the migration runbook is complete.

Sync ordering within the Foodie Application:

| Wave | Resources |
| --- | --- |
| -20 | Retained PVs/PVCs |
| -10 | SecretStore and ExternalSecret |
| 0 | PostgreSQL, Redis, MinIO, services, generated ConfigMap |
| 10 | Release-specific migration Job (`migration up`) |
| 20 | Backend and frontend |
| 30 | Public ingress |

Migrations must be backward compatible: the previous backend can continue to
serve while migrations run. Use expand/contract changes for destructive schema
changes. A failed Job blocks the new backend rollout. Investigate its logs before
deleting the failed Job and resyncing. No Job TTL is set, so completed Jobs are not
continuously recreated by Argo CD. Pruning is off; remove old Jobs and generated
ConfigMaps manually once no running or rollback workload references them.
Historical migration Jobs use `IgnoreExtraneous` so retaining completed rollback
Jobs does not leave the Application permanently OutOfSync.

## Secrets

`ExternalSecret/foodie` reads Login items from `foodie-production`, using the
`password` field on each item. Existing OAuth, S3 and allowlist items are reused.
Create/update the following without putting values in Git:

| Item | Required value |
| --- | --- |
| `DATABASE_URL` | `postgres://<url-encoded-user>:<url-encoded-password>@postgres.foodie.svc.cluster.local:5432/<database>` |
| `POSTGRES_DB` | Existing application's database name |
| `POSTGRES_USER`, `POSTGRES_PASSWORD` | Target database owner credentials, matching DATABASE_URL |
| `REDIS_PASSWORD` | Strong random password; URL-encode it in REDIS_URL |
| `REDIS_URL` | `redis://:<same-password>@redis.foodie.svc.cluster.local:6379/0` |
| `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET` | Existing Google OAuth credentials |
| `AWS_USER`, `AWS_PASSWORD` | Existing MinIO credentials, preserved during migration |
| `ALLOWED_MAILS` | Existing application's allowlist format |

BASE_URL, FRONTEND_URL and AWS_URL are ordinary ConfigMap values pointing at the
new public domain. Do not keep the old Compose database/service hostnames in the
target URLs. Keep source credentials recoverable before updating shared vault
items; rollback to Compose may need them.

From `~/plex-docker`, install the dedicated token interactively after the
namespace exists:

```sh
python3 bootstrap/configure-onepassword.py --namespace foodie
```

The helper does not print the token or save it to a file. Keep a recovery copy in
1Password. Rotation of application Secrets requires restarting affected Pods
after ESO refresh; environment variables do not hot-reload. Changing
POSTGRES_PASSWORD does not change an already-initialized database role password:
rotate that role in PostgreSQL as well and update DATABASE_URL together.

## Local validation

Requires Docker, kubectl and Python with PyYAML (CI uses 6.0.3).
The release updater also uses ruamel.yaml 0.18.16 to preserve YAML comments,
indentation and block scalars, keeping release PRs limited to version changes.

```sh
kubectl kustomize deploy/kubernetes/overlays/data-only > /tmp/foodie-data.yaml
kubectl kustomize deploy/kubernetes/overlays/production > /tmp/foodie-production.yaml
python3 scripts/validate-manifests.py /tmp/foodie-data.yaml /tmp/foodie-production.yaml
docker build -f backend/.docker/Dockerfile.prod -t foodie-backend:local .
docker build -f backend/.docker/Dockerfile.migrations.prod -t foodie-migration:local .
docker build -f frontend/.docker/Dockerfile.prod \
  --build-arg CADDYFILE=frontend/.docker/Caddyfile.kubernetes -t foodie-frontend:local .
```

The manifest checker verifies local safety/ordering invariants; it is not a
substitute for Kubernetes schema validation or the first runtime verification.
CI also runs a pinned kubeconform image for standard Kubernetes schemas. Custom
resources require the installed ESO/Argo CD/cert-manager schemas during rollout;
missing custom schemas are skipped by that offline CI check.

CI also verifies session recovery after disconnecting Redis clients. To run that
regression locally, use a dedicated disposable Redis server, then run from
`backend`:

```sh
REDIS_TEST_URL=redis://127.0.0.1:16379/0 cargo test --locked --test redis_store -- --ignored
```

This test intentionally disconnects the selected server's other clients; it must
not target production Redis.

## Operations, backups and rollback

```sh
kubectl -n argocd get application foodie foodie-infrastructure foodie-external-secrets
kubectl -n foodie get pods,pvc,secretstore,externalsecret,certificate,jobs
kubectl -n foodie logs deployment/backend
```

PVs use `Retain` and Argo CD `Prune=false,Delete=false`, under
`/var/lib/foodie/{postgres,minio,redis}` on node `sebastian`. These are local disk
directories: capacity declarations do not enforce quotas and do not survive loss
of the host. Do not use the default `local-path` class, whose reclaim policy is
Delete, for these data volumes.

Back up PostgreSQL with `pg_dump -Fc`, MinIO data **including `.minio.sys`**, and
Redis persistence. Keep encrypted copies off the machine and test restoring
PostgreSQL and MinIO together. The migration runbook includes dump/restore
commands. For a consistent filesystem backup of MinIO/Redis, pause application
writes and stop those workloads before copying; never tar live PostgreSQL data
as a logical backup replacement. Schedule off-host backups with the backup
destination/credentials you choose before considering migration complete.

For an application rollback, revert the release manifest commit and sync Argo
CD. The old migration Job should already exist, so it will not rerun. Do not
remove a current or rollback migration Job casually. A rollback does not undo
schema changes or data writes: incompatible database changes require an explicit
restore and a decision about writes made after the backup.

Recovery needs both repositories, the 1Password bootstrap identity, Argo CD
repository access, and off-host data backups. Restore cluster infrastructure and
namespace first, bootstrap secrets, restore data using `data-only`, then sync the
chosen production release. Certificate private keys can be reissued.
