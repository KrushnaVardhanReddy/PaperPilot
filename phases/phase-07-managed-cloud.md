## Phase 7 — Managed Cloud

**Goal:** Organizations that don't want to self-host can use PaperPilot as a service.

**Exit condition:** A company can sign up, connect their team, and use PaperPilot Cloud without operating any infrastructure.

**[BLOCKED BY Phase 6]**

---

### 7.1 — Cloud Infrastructure `[PARALLEL within group]`

| # | Task | Notes |
|---|---|---|
| 7.1.1 | Multi-tenant architecture | Org isolation at data and compute layer |
| 7.1.2 | Document processing pipeline | Async, scalable job processing |
| 7.1.3 | Cloud MCP gateway | Hosted MCP endpoint per org; evaluate `pmcp` zero-cost SDK for high-throughput multi-tenant streaming |
| 7.1.4 | Monitoring and alerting | Uptime, error rates, job queue depth |
| 7.1.5 | Autoscaling | Scale workers based on job queue |

---

### 7.2 — Cloud-Specific Features `[BLOCKED BY 7.1]`

| # | Task | Notes |
|---|---|---|
| 7.2.1 | Billing and usage tracking | Per-org metering |
| 7.2.2 | Admin dashboard | Org management, usage, billing |
| 7.2.3 | Onboarding flow & API Key Lifecycle | Self-service signup (GitHub/Google OAuth + Magic Links), org setup, developer API key provisioning (`pp_live_...`), and initial quota allocation verified at Cloudflare Edge via KV/D1 |
| 7.2.4 | SLA monitoring | Track and report uptime against commitments |
| 7.2.5 | Enterprise support tooling | Ticket system, escalation paths |
| 7.2.6 | E2E Cloud tenant tests | End-to-end multi-tenant isolation and billing workflow tests (No Mocking) |

---

## Parallel Execution Summary

This is the high-level view of what can run simultaneously across phases:

```text
Phase 1
├── 1.1 (Setup)          ──────────── all parallel
├── 1.2 (Traits)         ──────────── after 1.1
├── 1.3 (Operations)     ──────────── all parallel with each other, after 1.2
├── 1.4 (Tests)          ──────────── parallel with 1.3
└── 1.5 (CLI)            ──────────── each command unblocked when its op is done

Phase 2 ──── starts after Phase 1
├── 2.1 (MCP setup)      ──────────── parallel
├── 2.2 (Tools)          ──────────── all parallel, after 2.1
└── 2.3 (Infrastructure) ──────────── parallel with 2.2

Phase 3 ──── starts after Phase 1 (PARALLEL WITH Phase 2)
├── 3.1 (Setup)          ──────────── parallel
├── 3.2 (Shell UI)       ──────────── parallel within group, after 3.1
├── 3.3 (Preview)        ──────────── parallel with 3.2, after 3.1
├── 3.4 (Operations UI)  ──────────── after 3.2
└── 3.5 (Job System)     ──────────── parallel with 3.2, after 3.1

Phase 4 ──── starts after Phase 3
├── 4.0 (Offline NLP)    ──────────── parallel [FREE TIER, no key needed]
├── 4.1 (LLM infra)      ──────────── parallel [PRO TIER, API key required]
├── 4.2 (Planning)       ──────────── after 4.0 + 4.1
├── 4.3 (NL UI)          ──────────── parallel with 4.4, after 4.2
├── 4.4 (Execution)      ──────────── parallel with 4.3, after 4.2
└── 4.5 (CLI AI)         ──────────── parallel with 4.3, after 4.2

Phase 5 ──── starts after Phase 4 (PARALLEL WITH Phase 6)
├── 5.1 (OCR)            ──────────── parallel with 5.2
└── 5.2 (Understanding)  ──────────── parallel with 5.1

Phase 6 ──── starts after Phase 4 (PARALLEL WITH Phase 5)
├── 6.1 (Identity)       ──────────── parallel within group
├── 6.2 (Security)       ──────────── parallel with 6.1 (RBAC needed first)
├── 6.3 (Audit)          ──────────── parallel with 6.1 and 6.2
└── 6.4 (Deployment)     ──────────── parallel with all of 6.x

Phase 7 ──── starts after Phase 6
├── 7.1 (Infrastructure) ──────────── parallel within group
└── 7.2 (Features)       ──────────── after 7.1
```

---

## Critical Path

The sequence that determines the earliest possible ship date for each milestone:

```text
1.1 Setup
  → 1.2 Traits
    → 1.3 Operations (all parallel)
      → 1.5 CLI
        → Phase 1 complete ✓

        → 2.1 MCP setup
          → 2.2 + 2.3 (parallel)
            → Phase 2 complete ✓

        → 3.1 Desktop setup (starts same time as 2.1)
          → 3.2 + 3.3 + 3.5 (parallel)
            → 3.4 Operations UI
              → Phase 3 complete ✓

              → 4.0 Offline NLP + 4.1 LLM infra (parallel)
                → 4.2 Planning (unified, resolver-agnostic)
                  → 4.3 + 4.4 + 4.5 (parallel)
                    → Phase 4 complete ✓

                    → Phase 5 + Phase 6 (parallel)
                      → Phase 7
```

---

## Recommended Start

If working solo or as a small team, this is the suggested order for the first 4 weeks:

| Week | Focus |
|---|---|
| 1 | 1.1 Setup + 1.2 Traits + PDF library evaluation |
| 2 | 1.3 Group A (Merge, Split, Extract, Delete, Reorder, Rotate) |
| 3 | 1.3 Group B + C (Compress, Repair, Metadata, Encrypt, Extract text/images) |
| 4 | 1.4 Tests + 1.5 CLI — ship a working `paperpilot` binary |

After week 4, you have something real to show. Start Phase 2 (MCP) and Phase 3 (Desktop) in parallel from week 5.

---

