import json, sys

p = r"C:\Users\joell\agentic_research\forter_api.json"
d = json.load(open(p, encoding="utf-8"))

print("TOP KEYS:", list(d.keys()))
print("window:", d.get("window"))

w = d.get("widgets", {})
print("WIDGET KEYS:", list(w.keys()))

# Look for anything industry-related
def walk(o, path=""):
    if isinstance(o, dict):
        for k, v in o.items():
            walk(v, path + "/" + k)
    elif isinstance(o, list):
        for i, v in enumerate(o[:3]):
            walk(v, path + f"[{i}]")
    else:
        s = str(o)
        if any(t in path.lower() for t in ("industr", "vertical", "categor")):
            print("HIT", path, "=", s[:200])

walk(d)
