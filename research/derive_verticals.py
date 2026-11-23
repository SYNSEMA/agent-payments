"""
Derive vertical VOLUME shares from Forter's published agent-mix data.

Forter publishes (a) each agent's share of ALL agentic orders (agentMix),
and (b) each agent's share WITHIN each vertical (whereAgentsShop.byVertical).
If both are true, then:

    agentShareWithinVertical[agent][V] * volume[V] = agentVolume[agent]

Summing over agents for a vertical, and using agentVolume[agent] = agentShare[agent]*T,
gives a linear system for volume[V]/T. We solve it (non-negative least squares)
to get the vertical volume ranking Forter does not publish directly.
"""

import itertools, json

# --- Data from Forter's public API, GET /api/v2/ai-agent-commerce?days=90 ---
# Snapshot fetched 2026-11-22, window 2026-08-24 .. 2026-11-22.
# https://ai.forter.com/agentic-commerce-statistics  +  https://www.forter.com/blog/ai-agent-order-data/

AGENTS = ["MUSE", "OPENAI", "GOOGLE", "INSTINCT", "GROK_BOT", "CHATGPT_AGENT"]

# agentMix: share of ALL agentic orders, 90 days (from API widgets.agentMix)
AGENT_SHARE_OF_ALL = {
    "MUSE": 0.44,
    "OPENAI": 0.25,
    "GOOGLE": 0.03,
    "INSTINCT": 0.07,
    "GROK_BOT": 0.15,
    "CHATGPT_AGENT": 0.04,
}

# whereAgentsShop.byVertical: pctOfVerticalAgenticOrders (share of THAT vertical's orders)
WITHIN_VERTICAL = {
    "digital":      {"MUSE": 52.92, "OPENAI": 0.49, "GOOGLE": 0.02, "INSTINCT": 12.75, "GROK_BOT": 22.41, "CHATGPT_AGENT": 11.55},
    "food_delivery":{"MUSE": 43.36, "OPENAI": 25.48,"GOOGLE": 0.02, "INSTINCT": 6.37, "GROK_BOT": 20.39, "CHATGPT_AGENT": 4.40},
    "physical":     {"MUSE": 41.06, "OPENAI": 37.47,"GOOGLE": 4.11, "INSTINCT": 5.09, "GROK_BOT": 9.73,  "CHATGPT_AGENT": 2.57},
    "travel":       {"MUSE": 60.68, "OPENAI": 0.0,  "GOOGLE": 0.0,  "INSTINCT": 10.11,"GROK_BOT": 23.70, "CHATGPT_AGENT": 5.54},
}
VERTICALS = list(WITHIN_VERTICAL.keys())

# Average agentic order value by vertical (USD), same 90-day window (orderValue.byVertical)
AOV = {"travel": 514.62, "physical": 238.23, "digital": 136.83, "food_delivery": 119.02}


def solve():
    """Non-negative least squares on: sum_V within[a][V]*v_V = agentShare[a], with sum v_V = 1."""
    # Build design matrix: unknowns v_d, v_f, v_p, v_t (fractions of total agentic orders)
    # Equations: for each agent a:  sum_V within[a][V]*v_V  =  agentShare[a]
    #             plus: v_d+v_f+v_p+v_t = 1
    rows, rhs = [], []
    for a in AGENTS:
        rows.append([WITHIN_VERTICAL[V].get(a, 0.0) / 100.0 for V in VERTICALS])
        rhs.append(AGENT_SHARE_OF_ALL[a])
    rows.append([1.0] * len(VERTICALS))
    rhs.append(1.0)

    n = len(VERTICALS)
    # Solve via projected gradient with simplex projection (sum-to-1, v>=0), augmented with Tikhonov term.
    import random
    random.seed(7)
    w = [1.0 / n] * n
    lr = 0.05
    for it in range(200000):
        # gradient of sum (A w - b)^2
        grad = [0.0] * n
        for r, ri in enumerate(rows):
            err = sum(rows[r][k] * w[k] for k in range(n)) - rhs[r]
            for k in range(n):
                grad[k] += 2 * err * rows[r][k]
        for k in range(n):
            grad[k] += 0.0005 * (w[k] - 1.0 / n)   # tiny prior toward uniform
        w = [max(0.0, w[k] - lr * grad[k]) for k in range(n)]
        s = sum(w)
        if s > 0:
            w = [x / s for x in w]

    resid = []
    for r, ri in enumerate(rows):
        pred = sum(rows[r][k] * w[k] for k in range(n))
        resid.append((ri, round(pred, 5)))
    return w, resid


w, resid = solve()
print("=== DERIVED vertical volume shares (Forter network, 90d to 2026-11-22) ===")
print(f"{'vertical':<15}{'vol share':>12}{'AOV':>10}{'spend share':>14}")
rows_out = []
for V, share in zip(VERTICALS, w):
    spend = share * AOV[V]
    rows_out.append((V, share, AOV[V], spend))
total_spend = sum(s[3] for s in rows_out)
for V, share, aov, spend in sorted(rows_out, key=lambda x: -x[1]):
    print(f"{V:<15}{share*100:>11.1f}%{aov:>10.2f}{spend/total_spend*100:>13.1f}%")
print()
print("residual check (predicted vs target per agent + sum):")
for ri, p in resid:
    print(f"  {ri}: {p}")

print()
print("=== derived agent mix per vertical (sanity: should match agentMix) ===")
for a in AGENTS:
    per_v = {V: WITHIN_VERTICAL[V].get(a, 0.0)/100.0 * w[i] for i, V in enumerate(VERTICALS)}
    print(f"  {a:<16} " + "  ".join(f"{V}={per_v[V]*100:5.1f}%" for V in VERTICALS))
