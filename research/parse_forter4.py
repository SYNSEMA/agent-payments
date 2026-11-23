import json, re

p = r"C:\Users\joell\agentic_research\forter_api.json"
raw = open(p, encoding="utf-8").read()
d = json.loads(raw)

# Collect all distinct keys anywhere in the tree, to find any volume-by-vertical field
keys = {}
def walk(o, path=""):
    if isinstance(o, dict):
        for k, v in o.items():
            keys.setdefault(k, []).append(path)
            walk(v, path + "/" + k)
    elif isinstance(o, list):
        for i, v in enumerate(o[:2]):
            walk(v, path + f"[{i}]")
walk(d)
for k in sorted(keys):
    if any(t in k.lower() for t in ("pct", "count", "orders", "volume", "share", "vertical")):
        print(f"{k}: {len(keys[k])} occurrences; e.g. {keys[k][0][:120]}")

print()
print("--- checkoutType ---")
print(json.dumps(d["widgets"]["checkoutType"], indent=1)[:1800])
print()
print("--- agentMix ---")
print(json.dumps(d["widgets"]["agentMix"], indent=1)[:2500])
