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

AGENT_SHARE = {"MUSE": 0.47, "OPENAI": 0.25, "GOOGLE": 0.03,
               "INSTINCT": 0.07, "GROK_BOT": 0.15, "CHATGPT_AGENT": 0.04}

A = np.array([[bv.get(V, {}).get(a, 0.0)/100.0 for V in VERTICALS] for a in AGENTS])
b = np.array([AGENT_SHARE[a] for a in AGENTS])
A2 = np.vstack([A, np.ones((1,4))]); b2 = np.append(b, 1.0)
sol, *_ = np.linalg.lstsq(A2, b2, rcond=None)
vol = dict(zip(VERTICALS, sol))

print("=== AGENT MIX PER VERTICAL (derived) ===")
print(f"{'agent':<16}" + "".join(f"{V[:11]:>13}" for V in VERTICALS))
for i,a in enumerate(AGENTS):
    row = [A[i][j]*vol[V] for j,V in enumerate(VERTICALS)]
    print(f"{a:<16}" + "".join(f"{x*100:>12.1f}%" for x in row))

print("\n=== PROTOCOL vs SCRAPER PER VERTICAL ===")
PROTO = {"OPENAI","GOOGLE"}
SCRAPE = {"MUSE","INSTINCT","GROK_BOT","CHATGPT_AGENT"}
print(f"{'vertical':<15}{'volume %':>10}{'protocol % of vertical':>23}{'scraper %':>12}{'AOV':>10}")
for V in VERTICALS:
    p = sum(A[AGENTS.index(a)][VERTICALS.index(V)]*vol[V] for a in PROTO)
    s = sum(A[AGENTS.index(a)][VERTICALS.index(V)]*vol[V] for a in SCRAPE)
    print(f"{V:<15}{vol[V]*100:>9.1f}%{p/(vol[V])*100:>22.1f}%{s/(vol[V])*100:>11.1f}%{aov[V]:>10.2f}")

print("\n=== SPEND SHARES ===")
spend = {V: vol[V]*aov[V] for V in VERTICALS}
tot = sum(spend.values())
for V in sorted(VERTICALS, key=lambda v: -spend[v]):
    print(f"  {V:<15} spend share {spend[V]/tot*100:5.1f}%   volume share {vol[V]*100:5.1f}%")

print("\n=== SANITY: reconstructed agent totals ===")
for i,a in enumerate(AGENTS):
    print(f"  {a:<16} derived {A[i] @ sol:.4f}  published {b[i]:.4f}")
print("  max residual:", round(float(np.max(np.abs(A2@sol - b2))), 4))
