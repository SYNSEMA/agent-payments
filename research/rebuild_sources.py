import re, io, sys, subprocess, os

BASE = r"C:\Users\joell\agentic_research"
DOC = os.path.join(BASE, "ENTREGABLE_verticales_agentic.md")

# Annotated sources: id -> (annotation)
ANN = {
 1:  "Forter — *Agentic Transaction Data: What AI Agent Purchases Reveal*, 18-nov-2026.",
 2:  "Forter Intelligence — *Agentic commerce statistics, live* + API pública `GET https://intelligence.forter.com/api/v2/ai-agent-commerce` (descargada 22-nov-2026; window 24-ago → 22-nov-2026).",
 3:  "Forter Intelligence — *Top AI Shopping Agents 2026, Ranked by Real Orders*, 21-nov-2026.",
 4:  "Forter — *AI Agent Order Data: What's Actually Happening at Checkout*, 16-nov-2026.",
 5:  "Visa — *Visa Agentic Commerce Benchmark: Almost Half of Test Runs Fail*, 5-nov-2026.",
 6:  "TechCrunch — *DoorDash said it's only received 'several hundred' orders from AI agents*, 30-oct-2026.",
 7:  "TechBag — *AI agents booked 4 Uber rides and about 500 orders in a week of 320 million trips*, 9-nov-2026.",
 8:  "Mastercard — *Agentic commerce: somewhere between the hype and the checkout* (Rewiring Commerce, c/Temasek), 16-nov-2026.",
 9:  "Adobe — *AI Traffic Drives 83% Higher Revenue Per Visit*, 29-oct-2026; y *AI Traffic Trends Report*, agosto-2026 (PDF).",
 10: "Lantern — *Shopify's AI-referred orders grew 3.3x in a year while agent checkout stays small*, 4-nov-2026.",
 11: "Riskified — *Agentic Commerce Pulse: Confidence is Rising In AI Shopping While Adoption Remains Nascent*, 27-oct-2026.",
 12: "Visa Payments Frontier — *Frontier Commerce: Can AI agents run an e-commerce store and supply chain?* (Visa + Andon Labs), 2026.",
 13: "Mastercard — *How Verifiable Intent builds trust in agentic AI commerce* (con Google).",
 14: "TubStays — *Agentic Booking Adoption Tracker: Q3 2026 edition*, 12-nov-2026.",
 15: "OpenAI — *Commerce policies*, actualizado 29-sep-2026.",
 16: "CNBC — *Amazon's AI shopping tool sparks backlash from online retailers that didn't want websites scraped*, 6-ene-2026.",
 17: "GeekWire — *'Do not search Amazon': Meta's Muse now shops elsewhere by default*, 2026.",
 18: "Travelport — *Travelport, Booked AI, Basis Theory, and Mastercard complete live end-to-end agentic flight purchase*, 12-nov-2026.",
 19: "Disputed.ai — *Visa CE3.0 Now Accepts Cross-Merchant Evidence*, oct-2026; y Chargewood — *CE 3.0 from 24 October: multi-merchant history that stops at your acquirer*.",
 20: "C9 Group — *Agentic Payments in October 2026: How Visa, Mastercard, Stripe and PayPal Let AI Agents Pay, and Who Carries the Risk*, oct-2026.",
 21: "Riskified — *Agent Identity Risk Intelligence*, 8-oct-2026.",
 22: "Sardine — *Know Your Human (KYH): Fraud prevention for agentic commerce*; y *Agentic Commerce Compliance: Verified Is Not Authorized*.",
 23: "Association of Payment & Technology (France Payments Forum) — *Commerce 2026: Agentic commerce report*, sept-2026 (400 Heads of Payment, UK/US, fielded 25-feb a 2-mar-2026).",
 24: "Walmart — *Making It Possible for Agents to Shop on Behalf of Customers*, oct-2026.",
 25: "Motley Fool — *Uber (UBER) Q3 2026 earnings call transcript*, 4-nov-2026.",
 26: "Forbes (John Koetsier) — *Visa Is Building Agentic Commerce. But Will Amazon Let AI Agents Shop?*, 19-nov-2026.",
 27: "Modern Retail — *Brands are upset that 'Buy For Me' is featuring their products on Amazon without permission*, 2026.",
 28: "PayPal — *PayPal and Perplexity Launch Instant Buy*, 25-nov-2025.",
 29: "The Verge — *eBay is restricting third-party AI agents*, nov-2026; y Ars Technica — *eBay bans illicit automated shopping*, 20-ene-2026.",
 30: "Propellic / Breaking Travel News — *Propellic study finds most bookings made by agentic AI would route through an intermediary*, 12-nov-2026.",
 31: "Forter — *4 Ways Merchants Can Reduce [Agentic Risk]*, 2026.",
 32: "Stripe — *Agentic commerce: A guide for businesses*, 2026.",
 33: "Sardine — *Lead, Agentic Commerce (Product & GTM)*, job posting; y *Series C Funding Announcement*.",
 34: "Ling et al. — *Signing the Transaction but Not the Decision: Whisper Attacks and a Binding Defense for AP2*, arXiv 2609.11757, 2026.",
 35: "CHEQ AI — *Agentic Commerce: Meaning & What to Know*, 10-nov-2026.",
 36: "Ling et al. — *How Agentic Is Agentic Commerce? A Population-Scale Measurement of x402 Adoption and Authenticity*, arXiv 2607.12575, 2026.",
 37: "Machine Observer — *What Do AI Agents Actually Buy?* (x402), 2026.",
 38: "Quantum Payments — *x402 agent payments: 191 million transactions, $41M settled — the reality check for merchants*, 2026.",
}

S = r"C:\Users\joell\AppData\Local\hermes\skills\research\grounded-citations\scripts\sources.py"
res = subprocess.run([sys.executable, S, "render", "--cited-in", DOC],
                     capture_output=True, text=True, cwd=BASE)
block = res.stdout.strip()
lines = []
for ln in block.splitlines():
    m = re.match(r"^(\[\d+\])\s+(\S+)(.*)$", ln)
    if m:
        idx, url, extra = m.group(1), m.group(2), m.group(3).strip()
        n = int(idx.strip("[]"))
        ann = ANN.get(n, "")
        tail = f" — {ann}" if ann else ""
        lines.append(f"{idx} {url}{tail}")
    else:
        lines.append(ln)

new_block = "\n".join(lines)

s = open(DOC, encoding="utf-8").read()
start = s.index("## Fuentes")
s = s[:start] + "## Fuentes\n\n" + new_block + "\n"
open(DOC, "w", encoding="utf-8").write(s)
print("Sources block rewritten, lines:", len(lines))
