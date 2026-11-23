# intent-proof — Colisión de nombre y decisión de marca

> 21-nov-2026. Hecho con RDAP de registro (no DNS, que miente) y lectura de las páginas reales.
> Criterio: lo que verifiqué, lo que inferí, y qué hacer.

---

## 1. Quién es quién con "intentproof"

| Proyecto | Qué hace | Stack / estado | Amenaza |
|---|---|---|---|
| **egagofur** — npm `intentproof` (sin scope), github.com/egagofur/intentproof | **Testing E2E de browser** con Playwright + motor Aside. *"Don't just test the code. Prove the user intent."* | TS, MIT, 0 dependientes, 1 versión, ago-2026 | **Ninguna.** Mismo nombre, otro dominio (QA), otro buyer (devs), otro producto |
| **Org `IntentProof`** — npm `@intentproof/sdk` | SDK Node para firmar `ExecutionEvent` records: `wrap(intent, action, fn)`, **Ed25519 + JCS**, outbox SQLite, transparency log (sigstore) | v0.2.1, **0 dependientes**, 1 PR, 1 colaborador (`ngillett`), 7 versiones | **La única real, y es un fantasma.** Ver §2 |
| **FeeeeelixWong** — github.com/FeeeeelixWong/intent-proof | "Local-first **Solana** transaction policy firewall and tamper-evident receipt ledger for AI agents" | TS, MIT, **2★ 8 forks 23 commits 2 personas**, v0.3.0. Demo **solo Solana Devnet**, x402/EVM/Aptos. Empezó para el *Superteam Agentic Engineering Grant*, bounty de **100 USDC** por contribución | **Baja.** Pre-ejecución, crypto, sin humano firmando |

**No hay una empresa "IntentProof" en ninguno de los tres.** Hay una librería de testing, un SDK abandonado y un
proyecto de grant de $100.

---

## 2. El `@intentproof/sdk` es un fantasma — verificado

- `github.com/IntentProof` → **404**. `github.com/IntentProof/intentproof-sdk-node` → **404**.
- Su email de contacto es `security@intentproof.io` → y ese dominio está **parked con skin "Coming Soon"**,
  con formulario de contacto al dueño. No hay producto. No hay sitio. No hay empresa.
- 0 dependientes, 1 PR, 1 colaborador, v0.2.1.

**Pero es el que usa nuestro stack exacto** (Ed25519 + JCS canonicalization sobre eventos de agente). Si revive,
es un competidor directo de "evidence SDK". **Probabilidad baja, impacto alto → monitoréalo, no le des vueltas.**

---

## 3. El hallazgo de mercado que importa: el lado saturado es el PRE-ejecución

| Proyecto | Qué hace | Escala |
|---|---|---|
| `mattijsmoens/intentshield` | "Pre-execution intent verification. Audits what your AI is about to do, **not what it says**." | 20★, BSL → Apache 2036 |
| `junwezheng/Warrant` | "should this specific action happen, given what the human actually asked for?" — 7 checks pre-ejecución | Apache-2.0, 1 commit (jul-2026) |
| `theaniketgiri/aip` | Agent Intent Protocol: identidad + intent + límites antes de actuar, Ed25519 | MIT, PyPI, dashboard |
| `intentframe/intentframe` | "validate actions before they touch files, credentials, network, APIs, or money" | AGPL, **479 commits** |
| `aretta-ai/aristo` | SDK de intent verificable *inline en código* (claims sobre funciones) | MIT, 597 commits |
| `baixinghao/intent-gate` | intent alignment antes de codear | MIT, MCP + plugin |
| `manishklach/intent-ir` | IR determinista: compilar/verificar/reejecutar packets de agente | MIT, research prototype |
| `intentlayer.ai` | "Cryptographic proof for every AI action" | empresa |
| `IntentPro` (London, 51-200) | revenue intelligence (no es payments) | funding |
| `INTproof` | "Intelligence-grade proof for serious capital" | **trademark de IOSI Global** |
| `Intent & Proof` | acquisition web | — |

**Todos son pre-ejecución: firewall, shield, gate, policy, block.** "No dejes que el agente haga esto".

**Casi nadie en post-ejecución, específico de compra, firmado por un humano, con calidad de disputa, verificable
offline.** Eso es lo que es nuestro demo, y es el lado vacío.

**Conclusión: no hay que huir del nombre por la competencia. Hay que huir del nombre porque el namespace está
saturado de basura pre-ejecución y eso nos entierra en SEO.**

---

## 4. La decisión: separar el nombre del FEATURE del nombre de la EMPRESA

Es lo que hizo Stripe: "Shared Payment Tokens" es un feature, no el nombre de la empresa. AEP: "DisputeBundle"
es un artefacto, no la marca.

| Capa | Qué va | Por qué |
|---|---|---|
| **Artefacto / feature (público)** | **"intent proof"** — en venta: *"un intent proof bundle"*. El concepto, no la empresa | Es preciso, memorable y describe exactamente el objeto. **No lo toques.** |
| **Categoría / SEO / título del landing** | El problema y el estándar, no la marca | Tu buyer busca "evidencia de disputa", "AEP", "chargeback de agente". **No busca "intent proof".** |
| **Marca / empresa** | Otra cosa | Ver §5 |

### El landing debería titularse por el problema

> **Evidencia de disputa para compras de agentes**
> Intent proof: bundle firmado por compra, verificable offline, AEP-compatible.

**SEO objetivo:** `agentic dispute evidence` · `AEP implementer` · `Verifiable Intent verifier` ·
`chargeback AI agent` · `agent purchase authorization evidence`.

---

## 5. Por qué NO registrar "intentproof" como marca

En el mismo campo ya están: `INTproof` (**trademark registrado** de IOSI Global), `IntentPro`, `Intent & Proof`,
`Intent Layer`, `IntentR`, `IntentTechnologies`, `IntentShield`, `Warrant`. Un wordmark en clase 42/36/9 en un
campo tan cargado tiene alta probabilidad de:
- **rechazo §2(d)** (confusión con un marca previa en el mismo campo), o
- **aceptación con alcance estrecho** que no te cubre el uso real (software de pagos).

**El nombre del repo no necesita trademark.** Invertí el trademark en el concepto que elijas como marca, si
elegís uno distinto.

---

## 6. Qué comprar (verificado con RDAP de registro, no con DNS)

| Dominio | Estado | Nota |
|---|---|---|
| `intentproof.com` | **OCUPADO — a la venta en GoDaddy** (`forsale.godaddy.com/forsale/intentproof.com`) | Es el que querés. Parking de GoDaddy = con precio publicado. |
| `intentproof.io` | **OCUPADO — parked, skin "Coming Soon" con formulario de contacto al dueño** | Negociable directo, probablemente más barato que el .com |
| `intentproof.dev` | OCUPADO — parked en Hostinger | |
| `intentproof.so` | **LIBRE** (RDAP oficial .so → 404) | Feo pero libre |
| `intentproof.co` | Sin respuesta del RDAP de .co | Verificar en registrar |
| `attesto.dev` | **LIBRE** | Backup |
| `attesto.com` / `attesto.ai` | OCUPADOS | — |
| `proofintent.ai` | **LIBRE** | Backup |
| `proofintent.dev` | **LIBRE** | Backup |

**Dato clave: `intentproof.com` e `intentproof.io` están parked = sus dueños los quieren vender.** Eso es
negociable, y te da una posición de negociación fuerte: el nombre vale más que el TLD feo.

---

## 7. Candidatos de marca (verificados disponibles)

| Candidato | Dominio | Lectura |
|---|---|---|
| **Attesto** | attesto.dev | "Attestation" en latín. Habla del producto (atestación verificable), corto, ES/EN. **Riesgo:** "attesto" ya es fuerte en data lineage (project de Databricks), o sea hay uso previo en software |
| **Proof Intent / ProofIntent** | proofintent.ai · proofintent.dev | Literal y dice exactamente qué es. **Además el orden invertido funciona mejor en SEO**: "proof" tiene más volumen de búsqueda y aparece primero. `proofintent.com` está ocupado |
| — | — | Descartado: `declared.ai/.dev` ocupados; `intentproof.co` sin verificar |

**Mi preferencia: `ProofIntent` (proofintent.ai).** Dice qué es, ordena bien para búsqueda, y `intentproof`
queda libre como **nombre del feature** ("an intent proof bundle"), que es donde el concepto rinde.

---

## 8. Qué hacer, en orden

1. **No renombres todavía.** La decisión de marca puede esperar 30 días. La conformidad AEP no.
2. **Hoy:** publicá el repo como **`@tuorg/intent-proof`** (scope propio). Cero colisión de npm, cero colisión
   de marca, y el nombre del concepto sigue siendo el que querés.
3. **Esta semana:** mail al owner de `intentproof.com` (vía GoDaddy for-sale) y al de `intentproof.io` (vía el
   formulario del parked). **Presupuesto máximo: u$s2.000-5.000.** Por encima de eso el nombre no lo vale, y
   la alternativa (`proofintent.ai`) sale u$s15 por año.
4. **Hoy:** reservá `proofintent.ai` + `attesto.dev` como backup. U$s30 en total, y te asegura la opción.
5. **En 30 días:** con los primeros design partners vas a saber qué palabra usa tu buyer. Elegí marca con ese dato.
6. **Monitoreá:** si `github.com/IntentProof` vuelve, cambia el panorama. Revisalo una vez por mes.

---

## 9. Lo que NO cambia

Ni el producto, ni el demo, ni la definición, ni el GTM. **Esto es una decisión de superficie, y la tomás con
información que todavía no tenés.** Lo que sí es urgente es la otra cosa: agregar AEP.

**El repo se llama `intent-proof` (o `proofintent`) porque es el nombre del feature. La empresa se llama como
haga falta para que el buyer te encuentre. No son la misma decisión y no hay que tomarlas juntas.**
