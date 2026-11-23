# intent-proof → **Open**: la estrategia del repositorio libre en la organización de Synsema

> 22-nov-2026. Esto **reemplaza** el plan comercial de THESIS.md §6 y §4. No es un plan B: es el plan.
> Tesis: *la evidencia de que un agente pidió, autorizó y gastó como debía no debería ser un monopolio.*

---

## 0. Por qué esto es mejor que la startup (y por qué no es gratis)

### Lo que ya tenés y no estabas viendo

Synsema **ya tiene los primitivos**:

| Primitivo de Synsema | Lo que es | Nuestro producto |
|---|---|---|
| `receipt` / `verify` / `keygen` | Receipts firmados, verificados offline | El proof bundle |
| `--attested` · `--auditor <X25519 pubkey>` | **Auditor independiente** del que el runtime espera | El verificador de terceros |
| `document_sign` / `document_verify` | Ed25519 / did:key | La firma del statement |
| `canonical_json` | RFC 8785 JCS | La canonicalización |
| `require net(...)` / `require llm(...)` | Capacidades deny-by-default | **El modelo de mandato** |
| `synsema get <pkg>` | Package manager propio | La distribución |
| `code --mcp` · `@synsema/wasm` | MCP + WASM | El embedding en cualquier runtime |
| `--cap-set` · `--deterministic` | Ejecuta sin red, determinista | La verificación offline |

> **No estás agregando un concepto nuevo al lenguaje. estás aplicando a un dominio el primitivo que el lenguaje
> ya tiene — receipts firmados con auditor externo — y que hoy nadie usa para dinero.**

Y el encuadre es nativo, no forzado: **Synsema es "the programming language for AI agents", deny-by-default.**
La tesis del lenguaje es *"no le des a un agente capacidades que no le concediste explícitamente"*. Un agente que
gasta dinero sin dejar prueba es **exactamente el failure mode que Synsema existe para evitar**.

### La matemática que cambia

| | Startup cerrada | Repo abierto en Synsema |
|---|---|---|
| Distribución | ventas, una por una | **todo agente escrito en Synsema** (MCP, WASM, `get`) |
| Costo de servir | Marginal ~u$s0,0002 | Marginal ~0, y el costo es **mantenimiento** |
| Escalabilidad | lineal con el equipo de ventas | exponencial con la base de devs |
| Precio | u$s24-60k/año, a pocos | u$s0 para el 99% |
| Riesgo | un competitor te compra | un fork te copia el formato |

**Lo que cambia no es el producto: es la distribución.** Y la distribución es el único problema que no
resolverías nunca desde Rosario.

---

## 1. Qué se abre y qué NO

> **La línea que hay que sostener, sin excusas:**
> **Abierto = el formato, el verificador, la referencia y los tests.**
> **Cerrado = la confianza.** Root de custodia, gestión de claves, el vault de evidencia, el servicio de
> attestation, los exports de compliance, SOC 2, SLA.

Si abrís la capa de confianza, no tenés empresa: tenés un proyecto de hobby con mucho trabajo. Si abrís todo y
no cobrás nada, **tenés autoridad sin fuente de fondos** — y la autoridad sin fondos se muere en 18 meses de
mantenimiento.

### Reparto concreto

| Componente | Licencia | Dónde | Por qué |
|---|---|---|---|
| **Especificación del formato** (intention + proposal + approval + conformance + receipt) | **Apache-2.0 + grant explícito de patente §3** | Spec en el repo, referenciable | Igual que AEP. **Es la diferencia con el 99% de los repos Apache-2.0**, que no tienen grant explícito |
| **Referencia de constraints AP2 v0.2** (`amount_range`, `allowed_merchants`, `budget`, `line_items` + `match_mode`) | Apache-2.0 | `synsema get` / stdlib | Es lo que AP2 §Verification le exige al merchant |
| **Extensión semántica** (un constraint type propio, registrado) | Apache-2.0 | Propuesta al FIDO Payments TWG + AEP | La puerta oficial: *"a uniquely defined `type`. A Schema… The evaluation algorithm"* |
| **Verificador de receipts** | Apache-2.0, **portable** | Repo + binding en TS/Python | **Debe ser portable.** Si solo existe en Synsema, es un walled garden |
| **Test vectors** + corrida de **AP2-WhisperBench** (1.544 escenarios) | Apache-2.0 | Repo, CI | La prueba de que no es humo |
| **Demo del vuelo** (6 constraints, `REJECTED — 2 de 6`) | Apache-2.0 | `synsema run demo/flight.syn` | Se instala y se corre en una línea |
| **DSL de intención** (`intent.syn` → `constraints.syn`) | Apache-2.0 | Módulo de Synsema | El lenguaje, no el producto |
| — | — | — | — |
| **Trust service**: root de custodia (HSM), rotación de claves, auditor remoto | **Propietario** | Repo/servicio cerrado | El negocio |
| **Evidence Vault** (append-only, con retention y chain-of-custody) | **Propietario** | Servicio | El producto que se vende |
| **Attestation de conformance** (tu implementación es conforme) + **programa de certificación** | **Propietario** | Servicio | El "seal of approval" — modelo OpenStack/CNCF/Let's Encrypt |
| **Exports de compliance** (SOX 404 / COSO / PCAOB AS 2201 / EU AI Act Art. 11) y **pack de disputa 13.3** | **Propietario** | Servicio | Lo que compra un controller |
| **Soporte / SLA / pen-test / SOC 2** | **Propietario** | Contrato | La confianza |

---

## 2. Los tres modos de fallo (y por qué los importan)

### 2.1 🔴 El cambio de licencia — el error HashiCorp

**Regla innegociable: la licencia del core no cambia nunca.** El precedente es caro: HashiCorp publicó su
cambio de BSL a licencia proprietaria, la comunidad forkeó (**OpenTofu**), y ellos perdieron el proyecto que
habían construido con años de comunidad. Si abrís Apache-2.0 y en 18 meses lo passes a BSL, perdés **la
autoridad que justamente estás construyendo**, y perdés a los que te ayudaron a construirla.

> **Decidilo hoy, por escrito, en el README: el core es Apache-2.0 + patent grant, para siempre.**
> Si algún día necesitás plata del core, la plata sale del **trust service**, no de cambiar la licencia.

### 2.2 🟠 La autoridad sin maintainer

Un repo Apache-2.0 en una org con **43 weekly downloads** y un solo maintainer es un pasivo. Si las releases se
pisan, los adoptantes se van al fork. **La implementación de referencia tiene que ser siempre la mejor que
existe.** Eso es una cinta de treadmill para un founder solo, y es el costo real (no monetario) de esta estrategia.

Mitigaciones:
- **Releases con ritmo** (Synsema va por 0.6.66 → 0.6.67; el ritmo es alto). Publicar el módulo con cada release.
- **CI que corra WhisperBench** en cada commit → si el estándar se mueve, te enterás antes que nadie.
- **Aceptar PRs** del ecosystem (ar-agents, MCP devs) con criterio, rápido.
- **Second maintainer**: convencé a alguien del `ar-agents` (el repo ya tiene la fase 2 en roadmap).
- Si a los 6 meses no tenés un segundo maintainer ni 10 estrellas fuera de tu propia red, **el modelo de
  autoridad no está funcionando** → ese es tu kill criterion.

### 2.3 🟡 "No negocio para pocos" puede terminar en "no negocio"

La tensión real: **quien puede pagar por confianza y compliance es, por definición, "los pocos".** El esquema
que usás en la vida real (Linux, Mozilla, Signal, Let's Encrypt, curl, PostgreSQL) resuelve esto de una forma
específica: **el artefacto es libre, y hay una capa (o un fondo) que lo sostiene.**

Tres mecanismos honestos, y hay que elegir **uno** explícitamente:

| Mecanismo | Ejemplo | Cuándo sirve |
|---|---|---|
| **Trust service propietario** (root de custodia, vault, attestation, compliance exports) | Stripe API sobre un SDK libre; Sentry self-hosted; Supabase | Si querés **que el negocio sea tuyo** y que el free tier sea el canal. Es el más simple y el que ya tenés priced |
| **Programa de conformance + certificación** | OpenStack, CNCF conformance, Let's Encrypt CA, TUF | Si querés que el **estándar** sea el negocio. Requiere que el free tier tenga volumen (sino no hay marca que certificar) |
| **Donaciones / sponsoring del conformance program** | Signal, EFF, curl, Haskell, Rust Foundation | Si de verdad no querés cobrar por confianza. **Es un negocio legal pero no un negocio comercial** |

**Mi recomendación: el #1 (trust service) como mecanismo de fondos, y el #2 (conformance) como mecanismo de
autoridad.** Se refuerzan: el certificado de conformance lo emite quien opera el trust service, y el trust
service solo es confiable si hay un programa de conformance abierto.

> Y la frase que lo ordena: **la confianza sí es un negocio, pero no tiene que ser un monopolio.**

---

## 3. El endgame de gobernanza (esto es lo que hace verdadera tu misión)

"No debería ser un negocio para unos pocos" tiene un mecanismo canónico, y no es buena voluntad: **la
neutralidad estructural**.

**Secuencia:**

| Fase | Qué | Cuándo |
|---|---|---|
| **0 — hoy** | El spec + la referencia + los tests viven en la **org de Synsema**, Apache-2.0 + patent grant. El trust service es propietario y **ni siquiera se anuncia todavía** | Ahora |
| **1 — 6-12 meses** | Adopción real: 10+ repos que lo usan, 2+ países, alguien Contributing que no sos vos. **El FIDO Payments TWG y AEP conocen la implementación de referencia** | Cuando se cumpla |
| **2 — 12-24 meses** | **El spec se mueve a un steward neutral** (una fundación nueva, o una existente: LF, FinOps Foundation, o una de payments). Synsema queda como **una implementación de referencia más**, no como el dueño | Cuando haya tracción real |
| **3 — 24+ meses** | Nadie es dueño del formato. El "producto" es la infraestructura de confianza sobre un estándar neutral. **Ahí sí, "no es un negocio para unos pocos" es un hecho estructural, no un eslogan** | — |

**Por qué esto también es defensa:** si Mastercard+Google extienden Verifiable Intent para cubrir el ítem
(ventana 12-18 meses), te come el topo **si el formato era tuyo**. Si el formato es neutral y vos sos **el
mantenedor de la implementación de referencia y del conformance program**, el topo se vuelve **tu trabajo**.

---

## 1.5 La pieza que faltaba: `agent-payments` es **la portable**

Todas las recetas actuales de `synsema.com/recipes` son *"on Horizen's Vela"* y exigen `VELA_SECP_KEY` /
`VELA_ADMIN_KEY` / `VELA_P521_KEY`. Son **benchmarks del enclave** de Horizen: por eso son desechables.

Y una de ellas es, textualmente, este producto:

> **Payment policy engine** — *"an AI agent reviews invoices and proposes payments, the enclave enforces the
> spending policy (**allowed payees, caps, an automatic limit, approvals**) and a contract pays. **The agent
> never holds a key.**"*

**Payees permitidos = `checkout.allowed_merchants`. Caps = `payment.budget` / `amount_range`. Aprobaciones = la
aprobación humana. "El agente nunca tiene una llave" = la clave del emisor.**

Lo que la versión de Vela **no** tiene: **evidencia portable**. Su audit log vive adentro del enclave, y
cualquiera — merchant, red, auditor — no lo puede verificar por su cuenta.

**`agent-payments` = el mismo motor de política, pero portable:**

| | |
|---|---|
| **Sin `VELA_*` y sin enclave** | Corre en cualquier backend. Secrets: los que pida (Stripe test keys, un RPC, un vault propio) |
| **Con una receipt chain portable** | Cadena AEP de 8 capas, sellada Ed25519 en `authorization`, `previous_hash` **dentro** del hash input, exportable como `DisputeBundle` |
| **Con el conformance check** | Que la compra cumpla lo que el humano pidió y aprobó — y que el resultado se verifique **offline, con solo claves públicas**, por un tercero que no confía en Synsema, ni en Vela, ni en la plataforma |
| **Con el demo del vuelo** | `REJECTED — 2 de 6` |

**La frase de posicionamiento:**

> **El motor de política de pagos que ya existía para el enclave — pero portable.**
> La evidencia de que la compra cumplió lo que el humano pidió y aprobó, verificable por cualquiera.

**Y el cliente natural es Horizen mismo:** ellos ya tienen el producto y el enclave; lo que no tienen —y nadie
tiene— es la evidencia portable que un merchant pueda presentar. Y como ya lo validaste con ellos, tenés el
caso de uso real, no uno inventado.

---

## 4. La trampa que tenés que evitar: el walled garden invertido

> **Si el primitivo de pagos solo existe en Synsema, entonces no estás haciendo "un negocio para pocos" —
> estás haciendo un monopolio con otro nombre, y encima de un lenguaje con 43 descargas semanales.**

Consecuencias concretas:

- Basis Theory, Mastercard, Google, un PSP en Tokio: **no pueden usar tu primitivo.** Y con el time-to-market
  que tiene el estándar (spec consolidada FIDO a mediados-2027), van a construir su propio.
- El spec tiene que ser **JSON plano + SD-JWT + un verificador de referencia en 2-3 lenguajes**. Synsema es la
  implementación de **primer nivel**, no la única.

**La regla:**
> **Spec portable, implementación portable, Synsema primero.**
> El repo del spec es **neutral** (no "synsema/…"). El código en Synsema es **una implementación más**, y es la
> que vos mantenés porque es la que usás. La diferencia entre vos y un fork es **el programa de conformance y
> el trust service**, no el acceso al formato.

**Consecuencia operativa:** el repo del spec **no** va en la org de Synsema (sería llamarlo propiedad de Synsema). Va en una org neutral desde el día 1. Lo que va en Synsema es **el módulo de lenguaje**: el DSL de
intención, los bindings, los ejemplos, el `synsema get`. Eso sí es territorio de Synsema, y es donde tenés el
derecho natural.

---

## 5. El repo, en detalle

### 5.1 Nombres (evitando las colisiones que ya vimos)

| | Nombre | Licencia | Dónde |
|---|---|---|---|
| **Spec + verificador de referencia** | `agentic-purchase-evidence` (org neutral) | Apache-2.0 + patent grant | Org neutral, desde el día 1 |
| **Módulo de Synsema** | `synsema agentic-payments` (o `synsema get @synsema/agentic-payments`) | Apache-2.0 | Org Synsema |
| **Trust service** | sin nombre público hasta que tenga cliente | Propietario | Aparte |

❌ No `intent-proof` / `intentproof`: **ocupados** (`egagofur` npm, org `IntentProof`, `FeeeeelixWong/intent-proof`)
y SEO muerto. ❌ No `proofintent`: es una marca, no un formato.

### 5.2 El README, en 30 segundos

```markdown
# Agentic Purchase Evidence

Qué pidió el usuario. Quién lo aprobó. Cuánto gastó. Qué compró.
Y que eso se pueda probar — sin confiar en la plataforma que lo hizo.

Una compra hecha por un agente de IA produce un receipt firmado:
- la intención declarada (qué pidió, con qué límites),
- la aprobación de un humano (passkey / biometría, clave del dispositivo),
- y la conformidad comprobada (si el cargo cumple lo aprobado).

El verificador corre offline, con solo claves públicas.
Compatible con AP2 v0.2, Verifiable Intent y A-Comm Evidence Protocol.

Por qué existe: los protocolos de pago agentico firman la transacción,
pero no la decisión (arXiv:2609.11757). Y la disputa que realmente aparece
—"no era lo que pedí"— no califica para Compelling Evidence 3.0:
no hay defensa de red.

Apache-2.0, con grant explícito de patente. La confianza (custodia de claves,
evidence vault, attestation de conformance) es un servicio aparte:
esto es el formato, no el negocio.
```

### 5.3 El primer commit

| | |
|---|---|
| `spec/` | Esquemas JSON + el modelo de constraints (AP2 + la extensión semántica) |
| `reference/` | Verificador en **2 lenguajes mínimo** (Rust + TS), con la misma salida |
| `testvectors/` | Casos de la auditoría de mutaciones (291) + **WhisperBench** (1.544) |
| `demo/flight.syn` | El vuelo a Madrid, 6 constraints, `REJECTED — 2 de 6` |
| `CONFORMANCE.md` | Cómo declarar conformidad + qué se evalúa |
| `LICENSE` | Apache-2.0 **+ `NOTICE` y el grant de patente explícito** |
| `PATENTS.md` | El grant explícito, redactado al estilo del de AEP |
| `GOVERNANCE.md` | La promesa de que la licencia no cambia. **Es lo que te compra autoridad.** |

---

## 6. Plan de 30 días — u$s0

| | Qué | Costo | Por qué |
|---|---|---|---|
| **S1** | **Extraer** `intent.syn` / `conformance.syn` / `proof.syn` a un **módulo de Synsema** (`require money(...)` o `synsema get`). Dejar el repo de hoy como la implementación de referencia | u$s0 | El lenguaje es la distribución |
| **S1** | Reescribir el bundle a **AEP-shaped**: 8 capas, `previous_hash` dentro del hash input, sello Ed25519 en `authorization`, export `DisputeBundle`, `chain_complete` | u$s0 | Conformidad real, no marketing |
| **S2** | **Portar el verificador a un segundo lenguaje** (TS o Python) y **correr AP2-WhisperBench** (1.544 escenarios) | u$s0 | Prueba de que no es humo + prueba de portabilidad |
| **S2** | Crear la **org neutral** del spec + el repo + `LICENSE` (Apache-2.0 + patent grant) + `GOVERNANCE.md` | u$s0 | La neutralidad estructural |
| **S3** | **PR al repo `ar-agents`** — su Phase 2 pide textualmente el AP2 mandate verifier | u$s0 | El canal de LatAm |
| **S3** | Publicar el **demo del vuelo** + un post en X con el `REJECTED — 2 de 6` en pantalla | u$s0 | Distribución + el video |
| **S4** | **Postularte al FIDO Payments TWG y a AEP**: "implementación de referencia, portable, con benchmark de 1.544 escenarios" | u$s0 | Ser parte del estándar |
| **S4** | **15 llamadas**: a developers de agentes, a PSPs, a risk/disputes de merchants | u$s0 | Ver si el *trust tier* se vende |
| — | **Gasto: u$s0.** El primer peso es cuando alguien paga por confianza | | |

---

## 7. Kill criteria (actualizados)

| | Si… | Entonces |
|---|---|---|
| **K1** | En 30 días: 0 llamadas de developers/PSPs/merchants | La tesis no se sostiene. Pivotá a anti-disputa general |
| **K2** | En 6 meses: <10 estrellas fuera de tu red y sin segundo maintainer | El modelo de autoridad no funciona. Volvé a startup cerrada o buscá otro wedge |
| **K3** | En 12 meses: nadie paga por el trust tier | El open source no financia nada. Decidí: ¿es un negocio o es una fundación? |
| **K4** | Mastercard+Google extiende VI al ítem | Te come el topo **si el formato era tuyo**. Por eso el spec es neutral |
| **K5** | En 90 días: sin 1 disputa real resuelta con tu paquete | Es un demo, no un producto |
| **K6** | En 12 meses: la licencia del core cambia o se pone ambigua | **Muerte de la estrategia.** Es la razón por la que existe `GOVERNANCE.md` |

---

## 8. La frase

> **Pedir, autorizar y cobrar con un agente debería ser auditable por cualquiera.**
>
> El formato es libre y siempre lo va a ser. La confianza es un negocio — pero no tiene por qué ser un
> monopolio.

*Relacionado: [THESIS.md](THESIS.md) (mercado, vertical, buyer) · [POSITIONING.md](POSITIONING.md) (post y
definición) · [NAMING.md](NAMING.md) (nombre) · [GTM.md](GTM.md) (mercado con correcciones)*
