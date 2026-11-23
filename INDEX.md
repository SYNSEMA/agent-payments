# INDEX — the project's map

> **If you are picking this up with Hermes, read this file first.** It is self-contained.
> **Repo:** `github.com/SYNSEMA/agent-payments` · **Folder on disk:** `~/Desktop/intent-proof`
> Last updated: 22-nov-2026, after the repo was assembled.

---

## 0. The three names, and why there are three

| | |
|---|---|
| **CSIBEINGS LLC** | the legal entity (US, Wyoming, with EIN). Owns the commercial trust service |
| **SYNSEMA** | the language, and the GitHub org that holds the OSS portfolio. **Never renamed** |
| **agent-payments** | **the repo, and the recipe.** The portable evidence format. Apache-2.0, forever |

The folder on disk is still called `intent-proof` — that is the project's working name and the name
of the internal format version strings (`intent-proof/intent/1`, `intent-proof/proof/1`), which
**cannot change** without invalidating every existing bundle.

---

## 1. What this repo is

A **format**, not a service: a signed bundle that lets a third party verify, offline and with
public keys only, that a purchase made by an AI agent was requested and approved by a human **and
that the purchase meets the request**.

**Three ways to use it**, and the recipe is only the first:

| | How |
|---|---|
| Deploy it | `syn deploy` (the [Synsema platform](https://synsema.com) CLI, Vercel-style) |
| Run it anywhere | `synsema serve web.syn` — no account, no network, no platform |
| Just verify | `node reference/ts/verify.mjs bundle.json` — Node 20+, **zero dependencies** |

**The format does not depend on Synsema.** The spec is plain text; the verifiers are TypeScript and
Rust. Synsema is the reference implementation and the ergonomic way in — not a dependency.

---

## 2. The map

```
spec/SPEC.md                    ← THE NORMATIVE DOCUMENT. Read this first.
spec/bundle.schema.json         the JSON Schema of a bundle
spec/pscp/0000-template.md      how the specification changes

reference/ts/verify.mjs         the TypeScript verifier (ZERO dependencies)
reference/rust/                 the Rust verifier (crates.io dependencies)
reference/implementations.md    the registry of independent implementations

testvectors/valid/              must verify
testvectors/invalid/            must be rejected — one file per attack
scripts/make-vectors.mjs        derives the vectors from the golden file
scripts/verify-vectors.sh       runs the Synsema verifier over the whole set

src/                            the reference implementation in Synsema
  intent.syn                    request → constraints
  conformance.syn               the checker
  proof.syn                     signing, approval, bindings, the nine checks
  verify.syn                    the offline verifier
  proof.test.syn                31 tests
web.syn                         the console (the recipe's entry)
web/index.html                  the console's UI

LICENSE  NOTICE  PATENTS.md  GOVERNANCE.md  CONTRIBUTING.md
CODE_OF_CONDUCT.md  SECURITY.md  CITATION.cff  CHANGELOG.md  REUSE.toml
syn.toml                        the recipe manifest
synsema.json                    the project file (minimum engine: 0.6.66)

research/                       the strategy, the thesis and the raw market research
  THESIS.md  GTM.md  OPEN.md  POSITIONING.md  NAMING.md  ORG-AND-NAME.md
```

---

## 3. Verified state (22-nov-2026)

| | |
|---|---|
| Unit tests | **31/31** |
| Mutation audit | **291/291 rejected, 0 accepted, 0 crashes** |
| TypeScript verifier | **1/1 valid verified, 13/13 invalid rejected** |
| Rust verifier | **4/4 tests ok** (same vector set) |
| Synsema verifier over the vector set | **14/14 as specified** |
| The three implementations agree | ✅ on every vector |
| `./demo.cmd` | exit **0** |
| The console | serves on 8477, `/api/state` answers |
| `reuse lint` (REUSE 3.3) | **86/86 archivos, exit 0** ✅ |
| The Chromium CLA claim | **corregida** — era falsa. Chromium sigue exigiendo CLA |

**Everything above was executed, not assumed.**

---

## 4. 🔒 Decided (do not reopen)

1. **CSIBEINGS LLC** (entity) · **SYNSEMA** (language/org) · **agent-payments** (recipe/repo).
2. **The licence never changes.** Apache-2.0 + explicit patent grant + DCO. The language's own
   source is already Apache-2.0, so this is continuity.
3. **No separate neutral org now.** The trigger is countable: **3 independent implementers** outside
   this org → then `spec/` moves to a neutral steward.
4. **The format is free forever**; the money is in trust (custody, rotation, availability, evidence
   an auditor accepts).
5. **`agent-payments` is the portable one** — no enclave, no host keys. The existing
   `vela-policy-engine` recipe is its host-bound twin.
6. **No access to anyone's conversations.** It is a library the agent's own runtime calls at the
   point of authorisation.
7. **No money spent** until someone buys or commits to buy.
8. **Where each layer runs** (`GOVERNANCE.md` §11): the **format is hosted nowhere** (it is
   downloaded and it runs) · the **recipe** is a platform `kind = "web"` service · background work
   with no ingress is a platform `kind = "worker"` · the **trust service** is a **dedicated runner
   in our own cloud** (or a plain VPS). The platform is the distribution channel, not the place
   where the money or the keys live.
9. **This is not a competing standard.** AEP (A-Comm Technologies) is the evidence *chain*; this is
   the intent-and-conformance layer. They compose. The repo says so in `spec/SPEC.md` §1.2, in the
   README and in the prior-art table.
10. **The recipe is NOT an entry in `SYNSEMA/recipes`.** That catalogue was last pushed on
    **6-oct-2026** and every entry in it is a `vela-*` recipe bound to Horizen's Vela (the devnet,
    the `VELA_*` secrets, the enclave). It is a Vela-era artefact: it will either be rewritten
   around Vela's retirement or go with it, and a portable, non-Vela recipe filed there would be
   either lost in that refactor or would make the catalogue incoherent. **The recipe lives as its
   own repository in the organisation** and is distributed as such.
11. **The distribution does not depend on the platform catalogue.** Three channels, in order of how
    portable they are:
    - **The format** — `git clone`, or the package channel: a `synsema.json` index under a stable
      URL so that anyone can write `use "agent-payments"` and run `synsema get`. **This is the
      "usable from everywhere" path, and it touches nothing of the platform.**
    - **The recipe** — `git clone` + `synsema serve web.syn`, or `syn deploy` for the one-click
      path.
    - The platform's recipe catalogue is a **discovery convenience**, not a dependency. Nothing in
      this repository is published there on purpose.

## 5. 🔓 Open (five, and two have a deadline)

| # | Question | Note |
|---|---|---|
| 1 | 🔴 **CSIBEINGS LLC's fiscal status** | Was there a capital contribution? Was the 5472 filed? Deadline **15 April**. See `research/ORG-AND-NAME.md` §2.5 — **the missing ITIN is not a blocker** |
| 2 | The public name of the trust service | Rúbrica / Aval / Sello / other. No hurry |
| 3 | Does the trust service live in the US LLC or in a local entity? | Splitting them is the clean answer |
| 4 | The name of the spec package | It lives in `spec/` inside the repo; a name of its own is optional |
| 5 | 🔴 **How to position against AEP** | AEP is a live Apache-2.0+DCO spec in the same space (`A-Comm-Tech/a-comm-evidence-protocol`). The repo already says "not a competing standard" and the two compose — but **the founder should confirm that framing before publishing**, because a second "agentic-commerce evidence standard" is the one thing that would make auditors and payments people see a duplicate |

---

## 6. What is still missing before the repo is pushed

**Blocking:**

- [ ] The `syn` CLI on this machine is **broken**: `syn whoami` fails with
      *"Capability not granted: file_read(".synsema/syn.json")"*. The installed `syn.syn` is from
      5-oct and lacks the `require file.read(".synsema/syn.json")`. **`syn update`** should fix it
      (the CLI fetches itself from the site). Until then `syn deploy` will not work.
- [ ] `syn setup` — sign in and pick the organisation. `~/.synsema/syn.json` exists (381 bytes,
      26-sep), so the machine is signed in; the local CLI just cannot read it.
- [ ] Create the repo `SYNSEMA/agent-payments` (public) and push. **The org `SYNSEMA` exists**
      (created 23-jul-2026) and the path is **404 → free**. `kitecosmic` is **admin** of the org
      and `members_can_create_repositories` is true, so `gh repo create` works.
- [ ] Tag `v0.1.0` — the recipe's `[source] ref` points at a tag, not a branch.
- [ ] **Publish the package index** so the format is consumable without the platform: a
      `synsema.json` with `id`, `version`, `main` and `files[]` (sha256 each), served under a
      stable URL (GitHub Pages of the organisation, not `raw.githubusercontent`, which can be
      claimed by a fork). The generator is the `make_index.syn` pattern from the Synsema docs.
      That is what makes `use "agent-payments"` work for anyone.
- [ ] ~~Add the recipe to `SYNSEMA/recipes`~~ — **decided: no.** See §4.10.
- [ ] Branch protection on `main`, private vulnerability reporting on, and the Scorecard badge
      (already in the README) lights up.

**Not blocking, but before v0.2.0:**

- [ ] Migrate the demo to the **flight** scenario (six constraints) — specified in
      `research/THESIS.md` §3, not implemented. Requires regenerating the golden file with a real
      WebAuthn ceremony and re-running the mutation audit.
- [ ] The intent extractor (today it is a fixed intent per scenario).
- [ ] Anti-replay: a timestamp or nonce compared against a validity window.

---

## 7. How to resume this with Hermes

**Fastest:** open Hermes in this folder and say

> `Leé ~/Desktop/intent-proof/INDEX.md y seguí con el próximo paso pendiente.`

**With the original conversation:** the history is preserved and searchable —

> `Buscá en mis conversaciones anteriores "agent-payments" y retomá desde el último estado.`

**From scratch:** say

> `Trabajamos en ~/Desktop/intent-proof, que se publica como github.com/SYNSEMA/agent-payments.
> Leé INDEX.md, spec/SPEC.md y research/ORG-AND-NAME.md antes de proponerme nada. La empresa es
> CSIBEINGS LLC, el lenguaje SYNSEMA, la receta agent-payments.`

### What survives a close

| | |
|---|---|
| ✅ | Every file in `~/Desktop/intent-proof/` · Hermes's **memory** (profile `default`) · the **skills** (`open-source-standard-strategy` with its worked example, `startup-gtm-validation`, `synsema`) · the **session history** (searchable) |
| ⚠️ | The local server (`synsema serve web.syn` → 8477) and the browser's virtual authenticator have to be started again |
| ❌ | The *context* of the conversation — which is why everything decided is **written in files** |

**The rule this project followed:** *if it is not in a file, it is not decided.*

---

## 8. Things that are true and must not be repeated wrong

- AP2 v0.2 does **not** keep the "Intent Mandate" in natural language — that was v0.1.
- The **23,1%** is a **derived** number (42,8% × 54%), not a published Visa metric, and the
  benchmark covers only TVs, laptops and appliances.
- Visa Compelling Evidence 3.0 is **optional** and covers only **10.4**. **13.3** does not qualify.
- The bundle is **AEP-shaped, not AEP-complete**, and it is **not Verifiable Intent compatible**.
  The honest table is in `README.md` and `CHANGELOG.md`.
- The signed message is `SHA-256(JCS(proofConfig)) ‖ SHA-256(JCS(doc without proof))` — **not** the
  document. See `spec/SPEC.md` §6.3.

---

## 9. Commands

```bash
cd ~/Desktop/intent-proof

synsema test src/proof.test.syn                                  # 31 tests
synsema run --cap-set "stdout,file.read" src/verify.syn -- proof-bundle.json   # VERIFIED
synsema run --cap-set "stdout,file.read" src/verify.syn -- tampered.json       # REJECTED (exit 1)
./demo.cmd                                                       # the three above, exit 0

node reference/ts/verify.mjs testvectors/valid                   # 1/1 verified
node reference/ts/verify.mjs testvectors/invalid                 # 13/13 rejected
cargo test --manifest-path reference/rust/Cargo.toml             # 4/4
sh scripts/verify-vectors.sh                                     # the Synsema verifier, 14/14
node scripts/make-vectors.mjs                                    # regenerate the vectors

synsema serve web.syn                                            # the console, 8477
```
