import json

p = r"C:\Users\joell\agentic_research\forter_api.json"
d = json.load(open(p, encoding="utf-8"))
w = d["widgets"]

def dump(name, obj, depth=0, maxd=3):
    pad = "  " * depth
    if isinstance(obj, dict):
        for k, v in obj.items():
            if isinstance(v, (dict, list)):
                print(f"{pad}{k}:")
                if depth < maxd:
                    dump(name, v, depth + 1, maxd)
                else:
                    print(f"{pad}  <...>")
            else:
                print(f"{pad}{k} = {v}")
    elif isinstance(obj, list):
        print(f"{pad}[list len={len(obj)}]")
        for i, v in enumerate(obj[:40]):
            if isinstance(v, (dict, list)):
                print(f"{pad}  [{i}]:")
                dump(name, v, depth + 2, maxd)
            else:
                print(f"{pad}  [{i}] = {v}")

for key in ["orderValue", "whereAgentsShop"]:
    print("=" * 70)
    print("WIDGET:", key)
    print("=" * 70)
    dump(key, w[key], 0, 4)
