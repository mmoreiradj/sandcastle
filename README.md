# 🏰 Sandcastle

**Sandcastle** is a Kubernetes operator + toolkit for spinning up **ephemeral environments** on demand.
It’s designed to make preview environments as easy as opening a pull request, running a CLI command, or triggering CI/CD.

Think **sandcastles at the beach**: quick to build, fun to use, easy to tear down.

---

## ✨ Features (planned & in-progress)

* **Ephemeral environments**
  Spin up short-lived environments per Pull Request, feature branch, or ad-hoc request.

* **Kubernetes native**
  Built as a Kubernetes Operator in Rust. Manages namespaces, resources, and cleanup.

* **Extensible infra**
  Crossplane-powered integrations (cloud databases, buckets, queues, etc.) planned.

* **Multiple entry points**

  * GitHub/GitLab PRs → auto environment
  * CLI → `sandcastle up my-feature`
  * API → request environments programmatically

* **Fast cleanup**
  Tear down automatically when PR closes, TTL expires, or on demand.

---

## 🗂 Repository Layout

```
sandcastle/
├── apps/
│   ├── operator/     # Kubernetes operator (Rust)
│   ├── api/          # API backend (Rust)
│   ├── cli/          # CLI tool (Rust)
│   └── ui/           # Web UI dashboard (TypeScript/React)
│
├── crates/           # Shared Rust crates
│   ├── core/         # core domain logic
│   └── utils/        # helpers, logging, etc.
│
├── packages/         # Shared TypeScript packages
│   ├── sdk-js/       # JS/TS client SDK
│   └── ui-kit/       # shared React components
│
├── charts/           # Helm charts
├── scripts/          # Dev/CI automation
└── docs/             # Design docs & architecture
```

---

## 🚀 Getting Started

### Prerequisites

* Kubernetes cluster (v1.25+ recommended, kind/minikube for local dev works fine)
* Rust (1.80+), Cargo
* Node.js (18+) + pnpm/yarn
* `kubectl` and optionally `helm`

### Local Development

1. Clone the repo:

   ```bash
   git clone https://github.com/your-org/sandcastle.git
   cd sandcastle
   ```

2. Build operator & CLI (Rust):

   ```bash
   cargo build --workspace
   ```

3. Start UI (TypeScript):

   ```bash
   cd apps/ui
   pnpm install
   pnpm dev
   ```

4. Deploy operator into cluster:

   ```bash
   kubectl apply -f infra/manifests/operator.yaml
   ```

---

## 🎮 Usage

### Spin up an environment (via CLI)

```bash
sandcastle up feature/my-branch
```

### Tear it down

```bash
sandcastle down feature/my-branch
```

### From Pull Requests

* Open a PR → Sandcastle operator detects it → spins up an isolated environment.
* Close/merge PR → environment is destroyed.

---

## 🛣 Roadmap

* [ ] Core operator (namespace + k8s resources)
* [ ] CLI for local/dev usage
* [ ] GitHub/GitLab CI integrations
* [ ] Crossplane integration (databases, queues, cloud infra)
* [ ] UI dashboard for visibility & control
* [ ] SDKs (JS, Rust) for embedding into workflows

---

## 🤝 Contributing

Contributions welcome! Check out [docs/contributing.md](docs/contributing.md) for guidelines.

---

## 📜 License

MIT

---
