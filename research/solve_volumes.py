"""
Solve Forter's published agent-mix tables for vertical VOLUME shares.

Forter's public API (GET https://intelligence.forter.com/api/v2/ai-agent-commerce)
publishes, for the same 90-day window:
  (a) widgets.agentMix.chartData.daily  -> each day's agent mix (pctOfAgenticOrders)
  (b) widgets.whereAgentsShop.chartData.byVertical -> agent mix WITHIN each vertical
      (each vertical's rows sum to ~100%, so it is a within-vertical mix)
  (c) widgets.orderValue.chartData.byVertical -> AOV per vertical

We want vol[V] = share of ALL agentic orders that are in vertical V.
Forter does not publish that. But it is identifiable:

    within[a][V] * vol[V] = agentVolume[a]
    agentVolume[a]        = sum over days of (daily share[a][d] * orders[d])
    sum_V vol[V]          = 1

We do NOT know orders[d], so instead we use the 90-day AGGREGATE agent mix
(which Forter publishes on the page: "Share of orders and spend, by agent")
as the agent's total volume share, then solve the over-determined linear system

    sum_V within[a][V]*vol[V] = agentShare[a]     for each of 6 agents
    sum_V vol[V] = 1                              (normalisation)

7 equations, 4 unknowns -> over-determined; the residual is a real test of
whether Forter's own tables are mutually consistent.

Caveat to carry into the write-up: this is a DERIVATION, not a Forter-published
number. Its accuracy depends on the two tables sharing one window and one
rounding regime.
"""
import json, os
import numpy as np

BASE = r"C:\Users\joell\agentic_research"
AGENTS = ["MUSE", "OPENAI", "GOOGLE", "INSTINCT", "GROK_BOT", "CHATGPT_AGENT"]
VERTICALS = ["digital", "food_delivery", "physical", "travel"]

d = json.load(open(os.path.join(BASE, "forter_api.json"), encoding="utf-8"))
w = d["widgets"]

bv = {}
for row in w["whereAgentsShop"]["chartData"]["byVertical"]:
    bv.setdefault(row["vertical"], {})[row["segment"]] = row["pctOfVerticalAgenticOrders"]
aov = {r["vertical"]: r["avgOrderValueUsd"] for r in w["orderValue"]["chartData"]["byVertical"]}

print("=== CHECK: does each vertical's agent-mix sum to 100%? ===")
for V, mix in bv.items():
    print(f"  {V:<15} sum={sum(mix.values()):6.2f}  (agents present: {len(mix)})")

# --- 90-day aggregate agent mix from Forter's published page (Aug 24 - Nov 21, 2026) ---
# Source: https://ai.forter.com/agentic-commerce-statistics.md  -> "Share of orders and spend, by agent"
AGENT_SHARE = {"MUSE": 0.47, "OPENAI": 0.25, "GOOGLE": 0.03,
               "INSTINCT": 0.07, "GROK_BOT": 0.15, "CHATGPT_AGENT": 0.04}
print("\nagent share (90d, orders):", AGENT_SHARE, "sum =", sum(AGENT_SHARE.values()))

A = np.array([[bv.get(V, {}).get(a, 0.0) / 100.0 for V in VERTICALS] for a in AGENTS])
b = np.array([AGENT_SHARE[a] for a in AGENTS])
A = np.vstack([A, np.ones((1, 4))])
b = np.append(b, 1.0)

# Least squares
sol, res, rank, sv = np.linalg.lstsq(A, b, rcond=None)
print("\n=== RAW LEAST-SQUARES SOLUTION ===")
for V, x in zip(VERTICALS, sol):
    print(f"  {V:<15}{x*100:7.1f}%")
pred = A @ sol
print("  residuals:", np.round(pred - b, 4))
print("  max|resid| =", round(float(np.max(np.abs(pred - b))), 4), " | rank =", rank)

# Non-negative constrained solve via active-set enumeration (4 unknowns -> brute force is exact)
import itertools
best = None
def feasible_nonneg_least_squares(A, b, n):
    best = None
    # enumerate all supports (subsets of columns), solve least squares on that support,
    # keep only solutions with all supported values > 0
    for r in range(1, n + 1):
        for cols in itertools.combinations(range(n), r):
            As = A[:, cols]
            try:
                x, *_ = np.linalg.lstsq(As, b, rcond=None)
            except Exception:
                continue
            if np.any(x < -1e-9):
                continue
            resid = float(np.sum((As @ x - b) ** 2))
            if best is None or resid < best[0]:
                full = np.zeros(n)
                full[list(cols)] = x
                best = (resid, full, cols)
    return best

resid, nn, cols = feasible_nonneg_least_squares(A, b, 4)
print("\n=== NON-NEGATIVE CONSTRAINED SOLUTION (exact active-set enumeration) ===")
for V, x in zip(VERTICALS, nn):
    print(f"  {V:<15}{x*100:7.1f}%")
print("  sum:", round(float(nn.sum()), 6))
pred = A @ nn
print("  residuals:", np.round(pred - b, 4))
print("  max|resid| =", round(float(np.max(np.abs(pred - b))), 4))
print("  SSE =", round(resid, 6))

print("\n=== DERIVED TABLE: vertical volume, AOV, and spend share ===")
spend = {V: nn[i] * aov[V] for i, V in enumerate(VERTICALS)}
tot_spend = sum(spend.values())
print(f"{'vertical':<15}{'VOLUME %':>11}{'AOV $':>10}{'SPEND %':>10}{'spend/vol':>11}")
for V in sorted(VERTICALS, key=lambda v: -nn[VERTICALS.index(v)]):
    i = VERTICALS.index(V)
    print(f"{V:<15}{nn[i]*100:>10.1f}%{aov[V]:>10.2f}{spend[V]/tot_spend*100:>9.1f}%{spend[V]/nn[i]:>11.2f}")

print("\n=== SANITY: recompute each agent's total share from the solution ===")
for i, a in enumerate(AGENTS):
    print(f"  {a:<16} derived {A[i] @ nn:.4f}   published {b[i]:.4f}   diff {A[i] @ nn - b[i]:+.4f}")

# sensitivity: perturb published agent shares by +/-2pp (rounding tolerance) and re-solve
print("\n=== SENSITIVITY: +/-2pp on each published agent share ===")
import random
random.seed(3)
for trial in range(6):
    bp = b[:6].copy() + np.array([random.uniform(-0.02, 0.02) for _ in range(6)])
    Ap = A[:6]; Bp = np.append(bp, 1.0); Afull = np.vstack([Ap, np.ones((1,4))])
    r2, nn2, _ = feasible_nonneg_least_squares(Afull, Bp, 4)
    print("   ", {V: f"{nn2[VERTICALS.index(V)]*100:.1f}%" for V in VERTICALS},
          " max|resid|=", round(float(np.max(np.abs(Afull @ nn2 - Bp))), 4))
