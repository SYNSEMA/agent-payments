import json

p = r"C:\Users\joell\agentic_research\forter_api.json"
d = json.load(open(p, encoding="utf-8"))
w = d["widgets"]

print("=== whereAgentsShop.byVertical FULL ===")
for row in w["whereAgentsShop"]["chartData"]["byVertical"]:
    print(row)

print()
print("=== whereAgentsShop other keys ===")
for k, v in w["whereAgentsShop"]["chartData"].items():
    if k == "byVertical":
        continue
    print(k, "=>", json.dumps(v)[:1500])

print()
print("=== orderValue other keys ===")
for k, v in w["orderValue"]["chartData"].items():
    print(k, "=>", json.dumps(v)[:900])

print()
print("=== growth / checkoutType / agentMix ===")
for k in ["growth", "checkoutType", "agentMix"]:
    print("----", k)
    print(json.dumps(w[k], indent=1)[:2500])
