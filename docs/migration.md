# Compose → k3s migration runbook

## Deployment status

The migration completed on **2026-11-05**. Production is at
`https://foodie.lyngwinther.com`, managed by Argo CD's production overlay with
automatic sync and self-healing. Pruning is disabled.

The original Compose data and containers are retained for rollback. The Foodie
containers are stopped with restart policies disabled. Cutover backups are in
`~/foodie-migration-backup-20261105-cutover/`; earlier verified backups are in
`~/foodie-migration-backup-20261105/`. These are local backups; encrypted off-host
backup scheduling still needs a chosen destination. A pre-existing missing
recipe image reference is recorded in the private earlier backup inventory.

The procedure below is retained for recovery/reference, not for rerunning against
live production. Before bootstrapping a fresh recovery, disable automatic sync
on the Foodie Application and set its path to `overlays/data-only`; the checked-in
normal configuration now starts production automatically. Restore commands use
`--clean` and must only target the intended fresh/stopped database.

## 1. Inventory and preserve the source

Source containers: `foodie_postgres`, `foodie_minio`, `foodie_redis`,
`foodie_backend`, `foodie_frontend`. At preparation time backend was restarting,
Redis and MinIO were stopped, and PostgreSQL/frontend were running. Establish
what is usable rather than assuming the old deployment is healthy.

Confirm the real application database name and data sizes without printing
credentials:

```sh
docker exec foodie_postgres psql -U postgres -Atc \
  "SELECT datname FROM pg_database WHERE NOT datistemplate;"
sudo du -sh .db/foodie_db .db/aws .db/foodie_redis .db/filestorage
docker inspect foodie_postgres foodie_minio foodie_redis \
  --format '{{.Name}} {{range .Mounts}}{{.Source}}:{{.Destination}} {{end}}'
```

Inventory recipe/account counts and the `aws` bucket's objects; record counts and
checksums in a private migration record. Inspect `.db/filestorage` and `.aws` for
legacy content/configuration and include any unique files in the backup. The
current backend uses MinIO, but do not discard legacy files without checking.

Save source credential references in 1Password before changing shared vault
items. The source Redis config was mounted but not passed to redis-server; do
not assume its password configuration or session persistence was active.

Use a private directory on a disk with sufficient space:

```sh
umask 077
mkdir -p "$HOME/foodie-migration-backup"
export SOURCE_DB='REPLACE_WITH_VERIFIED_DATABASE_NAME'
docker exec foodie_postgres pg_dump -U postgres -Fc --dbname="$SOURCE_DB" \
  > "$HOME/foodie-migration-backup/database.dump"
docker exec foodie_postgres pg_dumpall -U postgres --globals-only \
  > "$HOME/foodie-migration-backup/globals.sql"
```

Globals contain password hashes: protect them. Inspect globals for extra roles
needed by the application; the normal restore below remaps ownership to the new
POSTGRES_USER and deliberately does not restore source passwords. Archive MinIO,
Redis and legacy files only with their writers stopped. Retain an unmodified
source backup and an encrypted off-machine copy. Verify a disposable database
restore before proceeding; a successful dump command alone is insufficient.

## 2. Publish prerequisites when ready

1. Publish reviewed changes to both repositories (this preparation does not
   push them). The old SSH deployment workflow must be removed when publishing
   these changes.
2. Let CI publish images, set GHCR package access, then merge the first release
   PR. Check all three production references have real `sha256` digests.
3. Configure Argo CD's access to `https://github.com/seblyng/foodie.git`.
4. Configure `foodie-production` items per [deployment.md](deployment.md), with
   new in-cluster DATABASE_URL/REDIS_URL and preserved MinIO credentials.
5. Create a dedicated read-only 1Password service account for this vault. Keep
   its token recoverable outside the cluster.
6. Add Google authorized JavaScript origin `https://foodie.lyngwinther.com` and
   redirect URI `https://foodie.lyngwinther.com/api/oauth/google/callback`.
7. Prepare DNS to point `foodie.lyngwinther.com` at the same public ingress as
   the existing cluster. Confirm Pod access to this hostname works as well.

## 3. Prepare new storage and infrastructure

On node `sebastian`, create **new**, empty target directories. Do not point PVs at
`.db` and never run Docker and Kubernetes against the same database directory.

```sh
sudo install -d -m 0700 -o 70 -g 70 /var/lib/foodie/postgres
sudo install -d -m 0700 -o 1000 -g 1000 /var/lib/foodie/minio
sudo install -d -m 0700 -o 999 -g 999 /var/lib/foodie/redis
```

Check existing target paths before these commands; stop if they contain data.
The PostgreSQL image uses UID 70; Redis and MinIO use the configured UIDs 999 and
1000. Kubernetes fsGroup supplements volume access but is not a backup/migration.

From `~/plex-docker` after the commits are published:

```sh
kubectl apply -k bootstrap/applications
argocd app sync foodie-infrastructure
argocd app wait foodie-infrastructure --health --timeout 300
argocd app sync foodie-external-secrets
argocd app wait foodie-external-secrets --health --timeout 300
python3 bootstrap/configure-onepassword.py --namespace foodie
```

For the Argo CD CLI, use the existing login with `--grpc-web` (or use its UI).
Confirm `foodie-ingress` Certificate is Ready. The wildcard certificate uses the
existing acme-dns registration; keep certificate issuance serialized if another
wildcard challenge is already running.

## 4. Copy stopped object/session stores, then sync data-only

Pause writes: stop the old backend and frontend, then stop MinIO/Redis before
the final object/session copy. Keep PostgreSQL running to take a final dump.

```sh
docker stop foodie_backend foodie_frontend foodie_minio foodie_redis
docker exec foodie_postgres pg_dump -U postgres -Fc --dbname="$SOURCE_DB" \
  > "$HOME/foodie-migration-backup/database-final.dump"
sudo tar -C .db -cpf "$HOME/foodie-migration-backup/minio.tar" aws
sudo tar -C .db -cpf "$HOME/foodie-migration-backup/redis.tar" foodie_redis
sudo rsync -a --chown=1000:1000 .db/aws/ /var/lib/foodie/minio/
sudo rsync -a --chown=999:999 .db/foodie_redis/ /var/lib/foodie/redis/
sudo chmod 0700 /var/lib/foodie/minio /var/lib/foodie/redis
```

The raw MinIO copy is valid here because the pinned GHCR target contains the
**same verified image configuration and layers** as the stopped source. The
mirror workflow verifies the original config digest before publishing. Include
hidden files and metadata. If the source
image has changed since preparation, reassess compatibility or use a verified S3
object migration instead. Redis is also pinned to its source image. Inspect its
files for RDB vs AOF configuration: the target intentionally starts with RDB
persistence as the source default did. Do not silently ignore an AOF-only store.
Users should expect to log in again: cookies from the old domain do not transfer
to `foodie.lyngwinther.com`, even when Redis files are preserved.

The `foodie` Application must still point at `overlays/data-only`:

```sh
argocd app sync foodie
argocd app wait foodie --health --timeout 600
kubectl -n foodie wait externalsecret/foodie --for=condition=Ready --timeout=120s
kubectl -n foodie get pods,pvc
```

If secrets fail, check item names/token permissions and ESO conditions, not
Secret values. All expected source objects must exist in MinIO before proceeding.

## 5. Restore PostgreSQL

The target PostgreSQL initializes an empty POSTGRES_DB using the new credentials.
Restore into it while the Foodie backend is absent:

```sh
kubectl -n foodie exec -i postgres-0 -- sh -ec \
  'pg_restore --exit-on-error --clean --if-exists --no-owner --no-privileges -U "$POSTGRES_USER" -d "$POSTGRES_DB"' \
  < "$HOME/foodie-migration-backup/database-final.dump"
kubectl -n foodie exec postgres-0 -- sh -ec \
  'psql -v ON_ERROR_STOP=1 -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c "ANALYZE;"'
```

Compare table/recipe/account counts with the source. Keep the source PostgreSQL
and its data intact. Never attach its physical data directory to the new Pod.

## 6. Validate application before enabling ingress

Change `foodie` Application's source path in
`~/plex-docker/bootstrap/applications/foodie.yaml` to
`deploy/kubernetes/overlays/production`, publish that change when ready, then
reapply the bootstrap definitions. Keep auto-sync off.

For the first production sync, exclude the Ingress in the Argo CD sync UI. Sync
all other desired resources in a normal sync (not a selective migration-only
sync), then check the migration Job succeeds before the backend rolls out.
Port-forward the frontend/backend for basic HTTP and SPA checks. For full
HTTPS/OAuth/S3 validation before broad public access, sync the Ingress and use a
controlled DNS/hosts override from a test workstation to the ingress IP. Restrict
external access during this window at the edge if DNS is already public.

Verify:

- `/api/health-check`, SPA deep links, normal login and Google callback.
- Existing accounts, friends, recipes and images.
- New recipe/image upload, image download, and recipe/image deletion. Test all
  S3 operations; successful uploads alone do not verify backend deletion access.
- WebSocket interaction between two signed-in browser sessions.
- Restart backend, PostgreSQL, MinIO and Redis one at a time and verify data
  persistence and application recovery.
- MinIO console, PostgreSQL and Redis are not publicly reachable.

Do not write important new data until rollback expectations are agreed.

## 7. Cut over and finish

Enable the production DNS/route, verify HTTPS from outside the LAN, then enable
automatic sync (`prune: false, selfHeal: true`) for the Foodie Application in
the bootstrap manifest and apply it. Infrastructure applications can be enabled
for automatic reconciliation after successful setup as well.

Keep old containers stopped and source data/backups intact through an agreed
retention window. Retire SSH/production-secret GitHub Actions credentials after
confirming no remaining workflow uses them. Configure encrypted off-host backups
and perform a documented restore drill. Upgrade old dependency versions as a
separate, tested change after the migration is stable.

## Rollback

Before new writes: stop Kubernetes frontend/backend, remove/disable its public
route, restore the previous routing/DNS and source credential settings, then
start the old Docker data services/backend/frontend. Confirm the old backend is
healthy first: it was restart-looping at preparation time. Do not assume merely
starting it restores service.

After new writes: first back up Kubernetes PostgreSQL and MinIO, then decide how
to transfer those writes back or accept a restore point. Never start the old
database and silently lose newer data. Application image rollback within k3s is
preferable if the database schema remains backward compatible.
