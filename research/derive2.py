"""
Derive vertical VOLUME shares from Forter's published data.

Forter's public API exposes two things that constrain each other:
  (1) widgets.agentMix  -> each agent's share of ALL agentic orders (90d)
  (2) widgets.whereAgentsShop.chartData.byVertical -> for each vertical, the
      agent MIX WITHIN that vertical (each vertical's rows sum to ~100%)

So:  within[a][V] * vol[V] = agentVolume[a] = agentShare[a] * T
Unknowns: vol[V]/T for the 4 verticals.  6 agent equations + 1 sum equation = 7
equations, 4 unknowns -> over-determined, so the fit is a real check, not a
tautology. We report the residual per equation and test sensitivity.

NOTE ON PROVENANCE: this is a DERIVATION from Forter's published numbers, not a
number Forter publishes. Forter publishes AOV per vertical but NOT volume per
vertical. We flag it as derived everywhere.
"""
import json, os, sys

BASE = r"C:\Users\joell\agentic_research"
AGENTS = ["MUSE", "OPENAI", "GOOGLE", "INSTINCT", "GROK_BOT", "CHATGPT_AGENT"]
VERTICALS = ["digital", "food_delivery", "physical", "travel"]


def load(fname):
    d = json.load(open(os.path.join(BASE, fname), encoding="utf-8"))
    w = d["widgets"]
    bv = {}
    for row in w["whereAgentsShop"]["chartData"]["byVertical"]:
        bv.setdefault(row["vertical"], {})[row["segment"]] = row["pctOfVerticalAgenticOrders"]
    am = {}
    for row in w["agentMix"]["chartData"].get("byAgent", []) if isinstance(w["agentMix"]["chartData"].get("byAgent"), list) else []:
        am[row.get("segment")] = row
    aov = {r["vertical"]: r["avgOrderValueUsd"] for r in w["orderValue"]["chartData"]["byVertical"]}
    return d, bv, am, aov


def nnls_1d(rows, rhs, n, iters=120000, lr=0.03):
    """projected gradient, simplex-constrained (non-negative, sums to 1)"""
    w = [1.0 / n] * n
    for _ in range(iters):
        grad = [0.0] * n
        for r, b in zip(rows, rhs):
            err = sum(r[k] * w[k] for k in range(n)) - b
            for k in range(n):
                grad[k] += 2 * err * r[k]
        w = [max(0.0, w[k] - lr * grad[k]) for k in range(n)]
        s = sum(w)
        if s <= 0:
            return None
        w = [x / s for x in w]
    return w


def solve(bv, agent_share, aov, label):
    rows, rhs = [], []
    for a in AGENTS:
        r = [bv.get(V, {}).get(a, 0.0) / 100.0 for V in VERTICALS]
        if sum(r) == 0:
            continue
        rows.append(r)
        rhs.append(agent_share[a])
    rows.append([1.0] * len(VERTICALS)); rhs.append(1.0)

    w = nnls_1d(rows, rhs, len(VERTICALS))
    if w is None:
        print(f"[{label}] no solution"); return None

    print(f"\n=== DERIVED vertical volume shares  [{label}] ===")
    print(f"{'vertical':<15}{'vol %':>9}{'AOV $':>10}{'spend %':>10}")
    spend = {V: w[i] * aov[V] for i, V in enumerate(VERTICALS)}
    tot = sum(spend.values())
    for V in sorted(VERTICALS, key=lambda v: -w[VERTICALS.index(v)]):
        print(f"{V:<15}{w[VERTICALS.index(V)]*100:>8.1f}%{aov[V]:>10.2f}{spend[V]/tot*100:>9.1f}%")
    print("  residuals (predicted vs target):")
    worst = 0
    for r, b in zip(rows[:-1], rhs[:-1]):
        p = sum(r[k] * w[k] for k in range(len(VERTICALS)))
        worst = max(worst, abs(p - b))
        print(f"    pred={p:.4f} target={b:.4f}  diff={p-b:+.4f}")
    print(f"  max |residual| = {worst:.4f}")
    return dict(zip(VERTICALS, w)), worst


def agent_share_from_api(d):
    """agentMix widget: figure out its structure"""
    w = d["widgets"]["agentMix"]
    return json.dumps(w)[:1500]


for fname, label in [("forter_api.json", "API 90d fetch 2026-11-22"),
                     ("forter_30.json", "API 30d"),
                     ("forter_7.json", "API 7d")]:
    d, bv, am, aov = load(fname)
    print("=" * 78)
    print(label, "| window:", d["window"])
    print("agentMix raw:", agent_share_from_api(d)[:600])
    print("byVertical rows:", json.dumps(bv)[:400])
