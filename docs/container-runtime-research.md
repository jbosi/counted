# Container Runtime Research: Docker Alternatives (2026)

> Research done in March 2026 in the context of evaluating a potential migration away from Docker for the Counted project (single VPS, docker compose, 3 services).

---

## Landscape Overview

| Runtime | Architecture | Rootless default | Compose | Status |
|---------|-------------|-----------------|---------|--------|
| Docker Engine | Client + root daemon | No (opt-in) | `docker compose` (excellent) | Dominant standard |
| Podman | Daemonless, fork/exec | Yes | `podman-compose` (good) + Quadlet (native) | Production-ready since 2020 |
| nerdctl + containerd | Client + containerd daemon | No | `nerdctl compose` (experimental) | Best for K8s, poor VPS DX |
| Rancher Desktop | Wraps containerd or dockerd | Depends | Via docker compat | Desktop-only |

---

## Docker: Actual Problems

- **Daemon runs as root** — a compromised daemon means root on the host. The entire container lifecycle flows through a single privileged process. Docker's rootless mode exists but is not default and requires extra setup.
- **No native systemd integration** — `docker compose up -d` is not a system service. Getting it to start on boot requires wrapping it in a hand-written systemd unit, which is clumsy.
- **Daemon SPOF** — if dockerd crashes, all running containers go with it.

---

## Podman: Key Properties

**Architecture**: Daemonless. Each `podman run` forks directly from the CLI process. No shared daemon, no root requirement.

**Rootless by default**: Containers run as the invoking user. If a container is compromised, the attacker gets user-level access, not root.

**OCI-compliant**: All standard Dockerfiles work without modification. Same image format, same registry protocol.

**Quadlet**: The modern way to run containers as proper systemd services. You write a `.container` unit file (INI format) and drop it in `/etc/containers/systemd/`. systemd manages start-on-boot, restart, logging (journald), and health monitoring natively. `podman auto-update` + a systemd timer gives you automatic rolling updates from a registry.

**CLI compat**: `alias docker=podman` covers ~95% of daily usage. `podman build`, `podman push`, `podman pull` all work identically.

---

## podman-compose vs docker compose

podman-compose is a Python script that translates Compose YAML into podman commands. It is NOT a rewrite of docker compose — it's a compatibility layer.

**Works**: `depends_on`, `service_healthy` condition, `restart`, named volumes, bridge networks, environment files.

**Does NOT work**: `cache_from` (build layer caching shortcut). This is the only gap that affects this project — 4 lines that can be removed without losing functionality (podman's own layer cache handles incremental builds).

**Alternative**: Docker Compose v2 itself (`docker compose`) can be used with Podman via the Podman socket (`DOCKER_HOST=unix:///run/user/1000/podman/podman.sock`). This gives full docker compose feature compatibility with Podman as the backend — useful for devcontainer compat.

---

## Quadlet: Production Orchestration

Quadlet (merged into Podman 4.4, stable in 5.x) generates systemd units from `.container` declarative files. It replaces `docker compose up -d` for production.

**Why it's better than compose for a VPS**:
- Containers ARE systemd services — `systemctl start/stop/status/restart` works
- Logs go to journald — `journalctl -u my-service -f` with full systemd log retention
- Dependencies via `After=` / `Requires=` — cleaner than compose's `depends_on`
- Auto-update via `AutoUpdate=registry` + `podman-auto-update.timer` — pull new images and restart on schedule
- Survives reboots natively — no wrapper service needed
- `systemctl daemon-reload` picks up file changes automatically

**Example** `.container` file (INI format):
```ini
[Unit]
Description=My service
After=network-online.target

[Container]
Image=docker.io/myuser/myapp:latest
PublishPort=8080:8080
Network=myapp.network
AutoUpdate=registry

[Service]
Restart=always
EnvironmentFile=/etc/myapp/env

[Install]
WantedBy=multi-user.target
```

Drop into `/etc/containers/systemd/`, run `systemctl daemon-reload`, then `systemctl enable --now myapp`.

---

## nerdctl: Why Not

nerdctl is a Docker-compatible CLI for containerd. containerd is the runtime Kubernetes uses internally.

**Good**: Excellent performance, K8s parity, Docker CLI familiarity.
**Bad for this use case**:
- containerd was designed as a component, not a standalone tool — the surface area for a VPS admin is rough
- `nerdctl compose` is experimental and lags far behind
- More complex initial setup than docker or podman
- No Quadlet equivalent for systemd integration
- No rootless story comparable to Podman

Verdict: **use if you're running Kubernetes or building K8s tooling. Skip for a VPS.**

---

## Performance (2026 benchmarks)

| Metric | Docker | Podman |
|--------|--------|--------|
| Container start | ~1.2s | ~0.8s |
| Memory (daemon overhead) | ~100MB | ~85MB (no daemon) |
| Build speed | Equivalent | Equivalent |

Performance differences are marginal. Both are production-grade. Not a decision factor.

---

## CI/CD: GitHub Actions

The current pipeline uses `docker/build-push-action@v5` + `docker/setup-buildx-action@v3`. These work by invoking Docker, which is pre-installed on `ubuntu-latest` runners.

**Option A (zero change)**: Keep using docker/build-push-action. Images are OCI-compliant. Podman on the VPS pulls from Docker Hub identically. No CI changes needed.

**Option B (full Podman CI)**: Podman is also pre-installed on `ubuntu-latest` runners (since ~2023). Replace the actions with:
```yaml
- run: |
    podman build -t docker.io/jbosi/backend:latest --layers .
    podman push docker.io/jbosi/backend:latest
```
Simpler YAML, same result. Tradeoff: loses GitHub Actions layer cache (`type=gha`). Podman's own `--layers` flag handles incremental builds at the layer level.

---

## Windows Dev Environment

Podman Desktop (free, no license restrictions) is the replacement for Docker Desktop on Windows. It uses a WSL2 VM as the backend (same as Docker Desktop).

VS Code Dev Containers support Podman via:
```json
"dockerPath": "podman"
```
in `devcontainer.json`.

Docker Desktop for Windows remains valid — the Docker daemon license is still free for personal use. Switching on the dev machine is optional.

---

## Summary: Worth Switching?

**Yes, for production (VPS)**. Quadlet is genuinely better than wrapping `docker compose up -d` in a systemd unit. Rootless Podman is genuinely safer than a root Docker daemon. The Dockerfiles don't change. The cost is writing 3 Quadlet unit files and learning ~15 minutes of Quadlet syntax.

**Optional for local dev**. podman-compose works for a 3-service stack. Docker Desktop works fine too.

**No change needed in CI/CD** (Option A). Images are OCI, Podman on VPS pulls them fine.

---

## References

- [Podman vs Docker 2026 (Last9)](https://last9.io/blog/podman-vs-docker/)
- [Podman Quadlet: Replace docker-compose for servers](https://matduggan.com/replace-compose-with-quadlet/)
- [Make systemd better for Podman with Quadlet (Red Hat)](https://www.redhat.com/en/blog/quadlet-podman)
- [podman-compose GitHub](https://github.com/containers/podman-compose)
- [nerdctl: Docker-compatible CLI for containerd](https://dev.to/lovestaco/nerdctl-a-docker-compatible-cli-for-containerd-4i2l)
- [Docker Security 2026: Hardening Containers](https://zeonedge.com/blog/docker-security-best-practices-2026-hardening-containers-build-runtime)
- [Podman Desktop — Docker Desktop alternative](https://podman-desktop.io/docs/migrating-from-docker/managing-docker-compatibility)
