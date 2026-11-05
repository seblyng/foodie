Want to make a website for easy picking of dinner

- Add you recipes with picture
- Connect with friends in a house holding
- Head v Head for x amount of rounds to decide what you want most
- Categories for food: Junk, Healthy etc...

Will probably never finish it, but fun to play around with Leptos and Rust in general

## Deployment

Production runs on the k3s/Argo CD cluster at
`https://foodie.lyngwinther.com`. See [deployment and releases](docs/deployment.md)
and the [Compose-to-Kubernetes migration runbook](docs/migration.md).

Kubernetes configuration is in `deploy/kubernetes`. The former `./deploy`
Compose script is now `scripts/deploy-compose` and must be run from the repo root.
