# intent-proof — Go-to-market v2 (evidenciado, corregido, escéptico)

> Actualización 21-nov-2026, post-investigación profunda. **Sustituye a la v1.**
> Regla: toda cifra lleva fuente. Donde no hay evidencia, dice "no hay evidencia".
> Marca `CORRECCION` donde la v1 estaba equivocada.

---

## 0. Lo que cambió desde la v1 de esta misma tarde

Cuatro hallazgos que **corrigen o invierten** el análisis anterior:

| | v1 decía | Realidad verificada |
|---|---|---|
| **CORRECCION** | "AP2 define `Shopping Intent` con `Prompt Playback` en lenguaje natural" | **Falso en v0.2.** El Intent Mandate y el `natural_language_description` estaban en **v0.1** y fueron **borrados** en v0.2 (spec vigente, 28-abr-2026). v0.2 solo tiene Checkout Mandate y Payment Mandate, con constraints de *open mandate*. |
| **NUEVO** | (no lo conocía) | **Verifiable Intent (VI)** existe: Apache-2.0, `github.com/agent-intent/verifiable-intent`, v0.1-draft 18-feb-2026, de **Mastercard + Google**, donado a FIDO el 28-abr-2026. **Tu esquema propio ya tiene dueño.** |
| **NUEVO** | (no lo conocía) | **A-Comm Evidence Protocol (AEP)** existe: Apache-2.0 **con grant explícito de patente**, cadena hash de 8 capas sellada con **Ed25519 en la autorización**, export `DisputeBundle`. **Es tu producto, ya especificado, con Visa/Mastercard/Amex/Citi/Wells Fargo atrás.** |
| **NUEVO** | "el dolor es narrativo, el mercado chico" | **arXiv:2609.11757 (sep-2026) prueba que los protocolos de pago agentico "firman la transacción, no la decisión"**: el 73,3% de las veces una sola frase del merchant movió al agente del producto más barato al más caro, y **0 de 68** carritos fueron rechazados por los controles de binding. El dolor es real y documentado por terceros. |

**El veredicto no cambia de dirección, cambia de fundamento:** el problema está validadísimo y la solución ya
está especificada por la industria con licencia abierta. Tu oportunidad **no es inventar el protocolo**: es
**ser el implementador de referencia en LatAm de AEP/VI + la capa semántica que nadie hace**, con la evidencia
de disputa como producto.

---

## 1. EL PROBLEMA: validado con la evidencia más dura que existe

### 1.1 El benchmark de Visa (5-nov-2026) — el dato que cierra la discusión

**Visa Agentic Commerce Benchmark**: 22 modelos de 12 labs, 15 episodios de shopping, 990 corridas totales.

| Métrica | Valor |
|---|---|
| Pass rate general | **57,2%** |
| **Corridas que terminaron en COMPRA EQUIVOCADA** | **23,1%** (229 de 990) — *derivado*: 42,8% de corridas fallidas × 54% que igual compraron. Visa publica 42,8% y 54%; el 23,1% es la derivación, y el benchmark **solo cubre TVs, laptops y appliances** |
| Corridas sin compra (el agente se abstuvo bien, o no pudo decidir) | 19,7% |
| De los failures, **% que igual terminó comprando** | **54%** |
| Mejor modelo | 82,2% (Gemini 4 Argon, Meta Muse Wonder) |
| Peor modelo | 28,9% |
| **Los 3 intentos de un mismo escenario pasaron las 3 veces** | **34,2%** |
| 5 modelos que más requisitos descubrieron | 70,2% |
| 5 modelos más caros | 67,1% (los 5 más baratos: 50,2%) |

Fuente primaria: <https://investor.visa.com/news/news-details/2026/Visa-Releases-First-of-Its-Kind-Industry-Benchmark-for-AI-Shopping-Almost-Half-of-Test-Runs-Fail>

Y el análisis que traduce el benchmark a tu pitch, textual:

> *"The payment stack downstream of that buy button **has no way to distinguish a correct agent purchase
> from an incorrect one**. Correct and incorrect purchases alike arrive as properly authorized card
> transactions, settle through the same clearing process, and land on the same merchant ledger."*
> — <https://ledgerdrift.com/analysis/visa-agentic-benchmark-what-happens-after-the-buy-button>

> *"That 23,1% is the number that matters for payment infrastructure. Each of those 229 runs produced a
> transaction that looks perfectly normal to the acquiring bank, the card network, and the issuer... Nothing
> in the transaction data signals that the agent recommended a TV without HDR10+ support when the customer's
> kid needs it for Xbox clips."*

**Consecuencia directa: Visa acaba de publicar la certificación de que el problema existe, quantificar la
magnitud y declarar que la infraestructura que falta es la tuya.** Eso es validación con la mejor fuente
posible: la red que vendería tu producto.

**Visa Trust Index (sep-2026, Harris Poll, 2.065 adultos US):** 72% usó un asistente de IA, pero **solo 23%
confía en que la IA maneje transacciones de pago en su nombre**.

### 1.2 La segunda capa: el agente B2B que gasta plata real ya existe

**Visa + Andon Labs, "Dee" (mayo-2026):** un agente de IA corriendo un negocio con **tarjetas Visa reales**.
Elegió la tarjeta de 2% rewards para el **96% de sus compras de abastecimiento** y se comió problemas de
timing de liquidación el primer día.

Fuente: <https://ledgerdrift.com/analysis/visa-agentic-benchmark-what-happens-after-the-buy-button>

### 1.3 El gap, textual

Serge Elkiner (GM de Paze en Early Warning Services) a Payments Dive:

> *"El banco no tiene registro de que ese consumidor pidió esa camisa porque el agente hizo la compra, y la
> tienda tampoco. **El banco y el merchant vieron un pago válido, y solo vos (y quizás la plataforma del
> agente) sabés qué pediste realmente.**"*

Worldpay, sobre la consecuencia operativa:

> *"Cuando un merchant no puede mostrar qué aprobó el shopper, los issuers tienden a dar la razón al
> cardholder."* Los disputes caen en Visa 13.3 "not as described", donde **el merchant es el más expuesto**.

Chargeflow: *"No hay un track de disputa separado para compras de agentes. Un dispute de una orden hecha por
un agente sigue siendo un dispute de tarjeta, con los mismos reason codes y deadlines."*

### 1.4 ⚠️ CORRECCIÓN: la CE3.0 **no** hace obligatoria la evidencia (y por eso hay más hueco, no menos)

**Visa Compelling Evidence 3.0, disputes procesados desde el 24-oct-2026.** Lo que cambia: las 2 transacciones
previas no reportadas como fraude ya no tienen que ser del mismo merchant, y el elemento de *"login ID del cliente"*
ahora **"also includes login IDs for an Agentic Payment Provider"**.

**Pero — verificado contra el Visa Core Rules (edición 18-abr-2026, §11.7.5.3 / §11.7.5.6) y commentary legal:**

| | |
|---|---|
| **Es opcional** | *"An Acquirer **may** submit Compelling Evidence"*. No obliga a nadie |
| **Solo cubre 10.4** (fraude / "no autoricé") | CE3.0 vive dentro de §11.7.5.3, una sección de 10.4. **No alcanza las condiciones 13.x** — o sea, **"no era lo que pedí" (13.3) NO califica** |
| **El login ID del agente no está disponible** | *"el agente le pasa al merchant el nombre, direcciones y email del titular, no su propio login ID"*. No hay fuente de Visa sobre cómo un merchant lo obtendría |
| **La mecánica está detrás del muro del acquirer** | *"el matching, la retrieval y el armado del paquete están del lado del acquirer, en un muro que el merchant no puede ver"*. Footnote 5 a Table 11-31: *"An Acquirer must only submit Transaction data accepted and processed itself"* |

**El gap real, entonces, es más grande de lo que yo decía:**

> La primera oleada de disputas agentic **no es "no me MV"** (10.4) — es **"no era lo que pedí"** (13.3,
> not as described). Forter: *"the highest-value claims — and vast majority of chargebacks overall — have been
> service disputes, such as 'item not as described'"* (18-nov-2026; la mayor, **u$s5.514**).
>
> **Y 13.3 no califica para CE3.0.** O sea: **en el tipo de disputa que realmente está apareciendo, la red no
> ofrece ninguna defensa y el merchant pierde por default.**
>
> Tu capa no compite con la CE3.0: **cubre un hueco que la CE3.0 no cubre.**

**El "registro que falta", textual** (mapeo de reason codes, chargewood.com):
para 10.4, lo que falta es *"the shopper's own device, IP and session on your site; **the shopper's consent and
payment instruction, which the agent holds**; a 3-D Secure result if it was skipped"*.

> **El agente tiene el consentimiento y la instrucción de pago. Nadie más los tiene.**
> Eso es exactamente lo que tu producto vuelve transferible.

**Nota de ventana:** chargeback rate de órdenes de personal-assistant: **0,012 pp** sobre las humanas, y la
ventana es corta, **4-5 semanas**. Es un problema que ya está pasando y se cierra rápido.

### 1.5 La evidencia de que el dinero ya circula

| Señal | Dato |
|---|---|
| **Sardine**, job post **15-nov-2026**, "Lead, Agentic Commerce (Product & GTM)", $220-270K + equity | *"You will be directly responsible for the revenue goal for Agentic commerce. We are ambitious and want you and your team to hit **$10M ARR in year 1**."* También: *"Agents still get blocked by merchants because most Agents behave like Bots."* → <https://jobs.a16z.com/jobs/sardine/9d67eba9-e460-4409-9e1c-353f55d983c9--lead-agentic-commerce-product-gtm> |
| **A-Comm**, ángeles | Visa, Mastercard, Qualcomm, SoFi, **American Express**, Citi, Wells Fargo, Fintech Brainfood |
| **Visa Agentic Ready** | 140+ participantes iniciales (Adyen, Antom, Billtrust, Checkout.com, Fiserv, Global Payments, J.P. Morgan Payments, PayPal, Stripe), 18-nov-2026 |
| Forter (18-nov-2026) | 1 de cada 10.000 órdenes es agentic; **orden agentic $227 vs $133 humana (1,71x)**; 60% de sus merchants ya vio al menos una |
| Coupa (nov-2026, 600 líderes) | **94%** quiere audit trail, **90%** umbrales de aprobación, 72% dejaría transaccionar, **1%** lo hace hoy |
| Gartner | **>40% de los proyectos de AI agéntico se cancelan antes de fin de 2027** (razón principal: *"inadequate risk controls"*) |

---

## 2. EL PRODUCTO: ya existe, y tu implementación ya es casi-conforme

### 2.1 Verifiable Intent (VI) — el esquema que te falta

`github.com/agent-intent/verifiable-intent` · Apache-2.0 · v0.1-draft 18-feb-2026 · **Mastercard + Google** ·
donado a FIDO 28-abr-2026. SD-JWT (RFC 9901) + `cnf` (RFC 7800) + `sd_hash`. Solo ES256.

**Cadena de 3 capas:**
- **L1** (~1 año): el **Issuer** — *"Either a financial institution (bank) that issued the user's payment card,
  or a payment network (Mastercard) acting on the bank's behalf"* — liga
  `iss/sub/iat/exp/vct/cnf.jwk/pan_last_four/scheme/_sd_alg/_sd`. En el perfil de referencia `vct` MUST ser
  `https://credentials.mastercard.com/card`.
- **L2**: firmado por la **user key** de L1. Modo **Immediate** (2 capas, valores finales) o **Autonomous**
  (3 capas, con **constraints**).
- **L3a/L3b** (~5 min): firmados por la **key del agente** resuelta de `cnf.jwk` de L2. L3a = payment mandate
  (a la red), L3b = checkout mandate (al merchant). **L3 MUST NOT contener `cnf`** (delegación terminal).

**Los 8 constraint types registrados** (`spec/constraints.md`):
`mandate.checkout.allowed_merchants` · `mandate.checkout.line_items` · `mandate.payment.allowed_payees` ·
`mandate.payment.amount_range` · `mandate.payment.budget` · `mandate.payment.recurrence` ·
`mandate.payment.agent_recurrence` · `mandate.payment.reference`

**Y acá está el gap, textual (spec/credential-format.md §13.4, Verifier Conformance, punto 10):**

> *"MUST check L3 values against disclosed L2 constraints. **Machine-enforceable constraints MUST be verified;
> descriptive fields are informational.**"*

Y §2.1, que es literalmente tu pitch (y por eso mismo es una amenaza):

> *"Constraints are the mechanism by which VI converts broad human intent ('buy me a tennis racket') into
> cryptographically enforceable limits ('from these SKUs, at these merchants, for no more than $300')."*

**Lo que VI NO tiene:** ningún campo de intención en lenguaje natural. Ni shopping intent ni texto del usuario
firmado. `line_items` acepta `acceptable_items[{id,title}]`, `quantity`, `match_mode: minimum|exact`:
identificadores y cantidades, **nunca "cumplió lo que pidió"**.

**Consecuencia de trust:** en VI, **L1 exige un emisor que sea banco o red**. Sin licencia bancaria **no podés
ser la raíz de confianza**. Podés ser **verificador** de la cadena de otro. Eso define el producto: un
notario, no un emisor.

### 2.2 A-Comm Evidence Protocol (AEP) — el protocolo de evidencia, ya especificado

`github.com/A-Comm-Tech/a-comm-evidence-protocol` · **Apache 2.0 con grant explícito de patente** ·
v1.0.3-rc.2 (draft) · A-Comm Technologies Inc.

**Cadena hash de 8 capas:**
`discovery -> referral -> intent -> delegation -> policy -> cart -> authorization (SEAL) -> fulfillment / refund (post-seal)`

Y el diseño, textual del README:

> *"Each artifact is **canonicalized (RFC 8785 JCS)**, **SHA-256-linked to its predecessor**, and the chain
> is **sealed with an Ed25519 signature at authorization**."*

> *"What no wrapped protocol provides — and AEP does — is the durable outcome trail: identity signatures are
> ephemeral per-request artifacts, mandates prove authorization-time consent, but none of them produce a
> sealed, independently verifiable record of what was charged, delivered, refunded, and settled."*

**Tu implementación ya es AEP-shaped**: `canonical_json` + `sha256` + `document_sign` (Ed25519) + sello en la
autorización + export offline + verificación de cadena. No es casualidad: es la forma correcta. Pero significa
que el IP que tenés es **la implementación, no el formato**.

**Lo que AEP envuelve explícitamente** (o sea, es *adjacent*, no competidor):
ACP checkout · AP2 payment mandates · TAP trusted agents · **Verifiable Intent** · MCP · UCP ·
KYA / KYA Pay (Skyfire) · x402 · A2A · Web Bot Auth.

**Y define el output que vos querés vender:**

> *"**DisputeBundle**" — exported by conformant exporters, consumible por PSPs, acquirers, merchants,
> dispute-management platforms, fraud decisioning systems, e independent verifiers.*

Con **Pre-Dispute Data-Sharing Profile**: *"expect transaction data to be queryable within roughly 24 hours
of authorization and answered at sub-second latency across a 13-month history."*

**Ojo antes de posicionarte:** AEP tiene un §9.3 "What AEP Is NOT" y un §9.4 "Regulatory Classification".
Leelos: puede que la capa semántica que te propongo ya esté declarada fuera de alcance.

### 2.3 AP2 v0.2 — qué exige y qué NO tiene

Spec vigente (28-abr-2026, FIDO Payments TWG, chairs Mastercard y Visa). Cinco roles: Shopping Agent,
Credential Provider, Merchant, Merchant Payment Processor, **Trusted Surface (MUST ser non-agentic)**.

**Definición de rol non-agentic (tu producto):**
> *"Communication to and from the Role is handled using deterministic code that verifies the authenticity and
> correctness. And if no processing done by the role is delegated to an LLM."*

**Modos:**
- **Human Present (Direct):** el usuario ve y aprueba el Checkout cerrado.
- **Human Not Present (Autonomous):** *"The User sees and approves a set of **constraints** over what closed
> Checkout and Payment would meet their intent. The Shopping Agent then assembles and approves a closed
> Checkout and Payment Mandate on their behalf using these open Mandates."*

> **Insight clave que la v1 no tenía:** en el modelo que la industria eligió, **el usuario NUNCA firma la
> compra concreta**. Firma un *conjunto de constraints* por adelantado; el agente firma el mandate cerrado.
> Tu demo hace lo contrario (el usuario aprueba la propuesta concreta después). **Son modelos complementarios,
> no el mismo.** AP2 cubre "el agente respetó los límites"; tu producto cubre "el humano aprobó ESTA cosa" +
> "esta cosa cumple lo que pidió". El estándar no cubre lo segundo. Ahí sigue el hueco.

**Verificación del Merchant (tu `conformance.syn`):**
> *"If open Checkout Mandates are included, verify that the closed Checkout conforms to all of the Constraints
> by evaluating each Constraint."*

**Extension point oficial (la puerta legítima para tu capa semántica):**
> *"**Mandate Constraints** — To define a new constraint, the following MUST be specified: A uniquely defined
> `type`. A Schema, including which fields are selectively disclosable. The evaluation algorithm."*

En VI, el punto de extensión equivalente: *"Parsers MUST preserve any fields in a constraint object that they
do not recognize"* + modo **STRICT** para deployments de alta seguridad.

**Otro hueco declarado por AP2:**
> *"the Shopping Agent needs to determine the applicable Mandates and Disclosures **ad-hoc** based on the
> Checkout. In the future, utilizing an explicit query language in the commerce protocol can help practical
> interoperability."*

**Detalle criptográfico que te obliga a cambiar algo:** AP2 exige que el `checkout_jwt` se firme con ECDSA y
**no** con Ed25519 (*"To prevent rainbow table attacks"*). VI exige **solo ES256**. Tu issuer usa Ed25519. Si
querés interoperar, tenés que emitir ES256.

### 2.4 Lo que SÍ falta en toda la pila (tu hueco real, preciso)

| Requisito | VI | AP2 v0.2 | ACP/Stripe | AEP |
|---|---|---|---|---|
| (i) Qué pidió el usuario en lenguaje natural, firmado | No | No (borrado en v0.2) | No (`reason: one_time`) | Parcial: tiene layer `intent`, contenido semántico no especificado |
| (ii) La compra concreta cumple esa intención | No (descriptive = informational) | Solo constraints cuantitativas | No | No |
| (iii) Prueba de autorización humana | Parcial: L2 user key (no es WebAuthn) | Si (Trusted Surface) | Si (Link / passkey) | Si (layer authorization sellado) |
| (iv) Audit trail verificable offline por un tercero que no confía en la red | Si (SD-JWT + sd_hash, salvo resolver JWKS del issuer) | Si | No | Si (por diseño: *"No party — including A-Comm — has to be trusted"*) |
| **(v) Un verificador neutral que alguien pueda auditar** | Parcial: L1 exige banco o red | Parcial: idem | No | Parcial: rol `Verifier` definido, sin implementer neutral de confianza |

**(i) y (ii) son tu producto. (v) es tu posicionamiento: el notario, no el emisor.**

### 2.5 Los competidores que ya están financiados en tu exacto terreno

| Empresa | Qué prueba | Evidencia offline | Funding |
|---|---|---|---|
| **Agentrail** | Runtime authz determinista + **"Railgun evidence layer"** | Parcial: receipts firmados | **US$28M Series A** |
| **Basis Theory** | Shared Payment Model; la Allowance lleva *"a customer-facing description of what the spending is for"* | No | ~**US$50M** ($33M Series B Costanoa 10/2025). Lidera el Agentic Commerce Consortium (20+) |
| **Crossmint** | Agent Cards sobre VIC | No | US$23,6M |
| **Beltic** | Identidad y legitimidad transaccional del agente en tiempo real | No (API online) | US$8,8M |
| **Skyfire** | KYA / KYA Pay | Parcial: JWT firmado | ~US$10M |
| **Nekuda** | Secure Agent Wallet + Agentic Mandates; launch partner de VIC | No | US$5M seed (Madrona + Amex/Visa Ventures) |
| **Alchemy AgentCard** | Pago con tarjeta del usuario vía VIC | No | (Alchemy) |
| **Authrail** | **Evidencia firmada Ed25519** (`ExecutorPrivateKey`, `GuardEvidence{inputSha256, outputSha256}`) | Parcial: firmado por la app, no por el usuario | Sin dato |
| **Darwinium** | *Intent Intelligence*: capa de fraude; valida firmas TAP/AP2/Agent Pay en el edge | No | Sin dato |
| **IntentPay** | "Intent orchestration protocol" + IntentGuard — **competidor directo de tu idea** | Sin dato | waitlist, pre-revenue |
| **A-Comm** | AEP + Evidence Vault + merchant agents | Si | ángeles: Visa, Mastercard, Amex, Citi, Wells Fargo, SoFi, Qualcomm |

**Conclusión competitiva:** el terreno de "evidencia de agente" está **más parecido al de las fintechs
infraestructura que al de un greenfield**. No estás entrando a un mercado vacío.

---

## 3. LA MATEMATICA QUE MATA EL TAKE RATE

### 3.1 El daño real por orden

- Tasa de chargeback agentic: **~0,65%**
- Ticket promedio US: **$110** (fuente primaria; **$120 en viajes**, la vertical más alta)

Perdida esperada por orden agentic = **$0,72**.

### 3.2 Lo que costaria cobrar

| Modelo | Cobro por orden de $100 | Multiplo vs. el daño |
|---|---|---|
| Take rate 1% | $1,00 | **15x mas caro que el daño que alivia** |
| Take rate 0,5% | $0,50 | 0,7x |
| Per-verification | $0,05 | 0,07x |

**Nadie paga 15x el daño para evitar el daño.** Un take rate es indefendible acá por razones de *valor*, no de
margen.

El margen no es el problema: servir un bundle cuesta **$0,0002** (firmas Ed25519 + WebAuthn + verificación
offline). **99,6% de margen.** El problema es que el pool es chico y el dolor es chico.

### 3.3 El pool

| Metrica | Valor |
|---|---|
| Agentic commerce US (valor de transaccion) | ~$1,44B |
| A 1% de take rate, **para TODA la categoria** | **$14,4M de revenue pool** |
| Agente-completed orders sobre e-commerce | 0,68% (Q3-2026) |
| Agent-mediated | 0,4% por volumen / **0,2% por valor** (Mastercard) |
| x402 oct-2026 | 5,1M txns = $274K -> **$0,054 por pago** |
| Allium Q3-2026 | 68,3M pagos = $3,7M -> $0,054 por pago |

**Take rate: descartar.** No por falta de margen, sino porque se cobra 15x el daño.

### 3.4 El pool que si tiene el tamaño correcto

| Mercado | Tamaño | Por que importa |
|---|---|---|
| **Disputas de tarjeta (global, 2026)** | 286M disputas, **$36,9B** de costo | 100x el de agentic commerce. La evidencia de mandato es el insumo del representment. |
| **Compliance B2B (SOX/COSO/AI Act)** | 94% de los leaders quieren audit trail, 90% thresholds | Ya hay presupuesto y deadline regulatorio (PCAOB AS 2201, 15-dic-2026). |

**El negocio vive en esos dos, no en el commerce agentico en sí.**

---

## 4. QUE MODELO DE NEGOCIO QUEDA EN PIE

### 4.1 (c) Per-verification — el ancla

- **$0,05 por bundle verificado.** Anclaje de mercado: Stripe SPT **$0,15** · Basis Theory **$0,0498** ·
  Crossmint **$0,05**. Estas en el mercado, no inventas.
- A 1.000 usuarios x 30 compras/mes = **$18.000/año**. No es un negocio. Es **una puerta de entrada**.
- Funcion: **credenciales + distribucion**, no revenue.

### 4.2 (e) Licencia de compliance — el negocio real

**$24.000–60.000/año** a issuer / PSP / plataforma de spend.

Lo que compran, en realidad: **evidencia lista para auditor**. Root de confianza dedicado, receipts firmados
e inmutables, export para SOX 404 / COSO (feb-2026) / PCAOB AS 2201 (efectivo **15-dic-2026**) /
EU AI Act Art. 11, reporte de exceptions, retencion, chain-of-custody.

### 4.3 (d) OEM / white-label — el multiplicador

Vendes el verificador embebido a quien lo implementa. Cobras por **volumen de mandates verificados**.

### 4.4 Lo que hay que evitar

| Descartado | Por que |
|---|---|
| Take rate | 15x el dano |
| Por-seat | No hay "usuario" en una cadena de software |
| Cobrarle al merchant | Ya paga 5,8-8,1% (PSP + interchange + processor) |
| Ser el emisor (L1) | VI exige banco o red. Sin licencia bancaria, imposible |
| Escribir un esquema propio | VI y AEP ya existen, Apache-2.0, con patente |

---

## 5. ESTRUCTURA, PRESUPUESTO Y PAIS

### 5.1 Estructura minima (sin licencia bancaria)

| Item | Detalle |
|---|---|
| Jurisdiccion | **SAS en Argentina con Ley 27.506** (alicuota de Ganancias **35% -> 14%** para sociedades con condiciones Pyme). Delaware solo al levantar ronda. |
| Licencia bancaria | **No.** No movés fondos, no custodias credenciales de pago, no sos emisor de instrumentos. El L1 de VI lo firma el banco o la red; vos verificás. |
| PCI-DSS | **No**, con scoping opinion formal ($2-5k). No tocas datos de tarjeta en tu plano. |
| SOC 2 | **NO en el año 1.** Type I mes 5-6 (~$12-20k, 5-15 meses). Type II mes 12-15. |
| Presupuesto año 1 | **$45.980 de $50k.** ARR target ~$198.000. |

### 5.2 A quien hay que contratar

| Rol | Por que |
|---|---|
| **Head of Partnerships** (no engineering) | Tu producto se vende **embebido en el flujo de otros**. El que vende es partnerships, not sales. |
| Security / compliance part-time | Scoping opinion, checklist de PCI, preparacion de SOC 2 Type I. |

### 5.3 El precedente de salida que hay que tener presente

**Oso Cloud** (el vendor de authorization mas cercano a tu tesis — "access controls for humans **and AI
agents**", 80 clientes, US$25,9M levantados en 3 rondas):

- Cerrado **1-oct-2026**. Tiny (TSX: TINY) pago **~C$1,9M** (C$1,2M en efectivo al cierre + ~C$0,7M en
  holdback) por los **assets**.
- ARR al cierre: **~US$5,3M** en ~80 clientes, todos por suscripcion, varios multi-anio.
- Multiplo: **~0,36x ARR** (spot 1,42 USD/CAD). **Menos de 1x.** BetaKit: *"equity holders likely wiped out."*
- **El equipo se fue a Temporal** (valorado US$12,55B, ~50x run rate). **El valor fue con las personas, no con el producto.**

Fuentes: <https://investors.tiny.com/news/news-details/2026/Tiny-Completes-Acquisition-of-Oso-Cloud/default.aspx> ·
<https://betakit.com/tiny-buys-oso-clouds-assets-for-1-9-million/> · <https://cybermerge.com/stories/oso-split-tiny-temporal-authorization-repriced>

**Traduccion para tu plan:** si construis un producto de infraestructura de autorizacion que se vende a
compradores de seguridad/fintech, **el escenario base de salida es un multiplo sub-1x y el equity se diluye.**
El upside esta en que el **equipo se vaya a algo grande** (como paso con Temporal), no en vender la empresa.
Planifica el negocio como un negocio de servicios con opcion de producto, no como una startup que va a vender
a 8x.

---

## 6. QUIEN TE COMPRA — ranked, con presupuesto y ciclo

| # | Buyer | El argumento de compra | Presupuesto | Ciclo | Veredicto |
|---|---|---|---|---|---|
| **1** | **PSP / acquirer / dispute platform LATAM** | *"Las primeras disputas agenticas no son 'no me MV' (10.4) sino 'no era lo que pedí' (13.3) — y 13.3 no califica para Compelling Evidence 3.0. En el tipo de disputa que realmente aparece, la red no ofrece defensa. Necesito el paquete."* | Real | 30-60 días | **El buyer #1.** CE3.0 es opcional y solo cubre 10.4; el login ID del agente no lo obtiene el merchant; la mecánica está detrás del muro del acquirer. El pool es el de disputas ($36,9B), 100x el de agentic commerce. Competis con Chargeflow y Justt (financiados). |
| **2** | **Issuer / banca LatAm** (Banco Nacion, Santander, Galicia, BBVA, Itau, Nubank) | Sus pilotos de Agent Pay / TAP necesitan un verificador de mandates que no sea la red | Alto, ciclo larguisimo | 9-18 meses | Escriben las reglas. Dificil de entrada, imposible de ignorar |
| **3** | **Plataformas de spend / procurement** (Ramp, Brex, Zip, Coupa, Oracle, Mercado Pago, dLocal, Kushki, Conekta) | Sus clientes enterprise exigen audit trail + thresholds verificables (94% / 90%). Hoy ofrecen un log | **Real** (Ramp: 85.000+ negocios, $15/user/mes) | 3-9 meses | **Buyer #2.** El que tiene el cheque ya escrito |
| **4** | **Implementadores de AEP / VI** (tu propio `ar-agents`, los MCP de commerce, los que arman AP2 en LatAm) | El modulo esta en su roadmap (AP2 mandate verifier, constraint evaluation). Necesitan un library, no un vendor | Es una linea de dev | Corto | **Distribucion, no revenue.** Y es tu mejor palanca |
| **5** | **Vendor de riesgo** (Sardine, Forter, Riskified, Darwinium, Beltic) | Un signal nuevo ("¿la compra cumplia la policy?") mejora la decision de chargeback | Muy alto | 6-18 meses, exigen SOC 2 | **Destino año 2.** Sardine ya tiene el budget line ($10M ARR año 1) y lo esta construyendo in-house |
| **6** | **Agent apps / merchants de consumo** | 23% de compras equivocadas, disputes | **Cero** | — | **No es tu buyer.** Es tu demo target |

---

## 7. QUE HARIA YO — 12 meses, $50k, desde Rosario

### Fase 0 — Meses 1-2: dejar de inventar

1. **Cambiar el bundle a AEP-shaped.** `canonical_json` (RFC 8785 JCS) -> `sha256` -> cadena de 8 capas ->
   **sello Ed25519 en el punto de authorization** -> `DisputeBundle` export. Estas a ~70% de AEP v1.0.3-rc.2.
   Tu `intent.syn` / `conformance.syn` pasan a ser una **extension de constraint** legitima: tipo unico, schema,
   algoritmo de evaluacion — exactamente lo que AP2 §Extension Points exige.
2. **Emitir ES256** (no Ed25519) para interoperar con VI (que exige solo ES256) y AP2 (que exige ECDSA en
   `checkout_jwt`). Ed25519 queda para tu capa interna.
3. **Leer AEP §9.3 "What AEP Is NOT" y §9.4 "Regulatory Classification"** antes de seguir. Puede que la capa
   semantica ya este declarada fuera de alcance.

### Fase 1 — Meses 2-4: distribucion (costo cero)

4. **PR al repo `ar-agents`** — su Phase 2 es exactamente "AP2 mandate verifier/signer". Implementa el modulo,
   hace el PR. **Es el canal mas denso que existe hoy en LatAm y cuesta $0.**
5. **Publicar un verificador AEP/VI open source** (Apache-2.0, con test vectors). El objetivo NO es revenue:
   es ser el implementador de referencia antes que otro, y capturar el feedback sobre que prueba necesita un
   auditor.
6. **Postularte al FIDO Payments TWG y a AEP.** AEP esta en ronda de comentario y busca implementadores. Si tu
   capa semantica entra como constraint type registrado, **dejas de competir y empezas a ser parte del estandar.**

### Fase 2 — Meses 4-9: los 3 design partners

7. **1 PSP / dispute platform LATAM.** Objetivo: **1 disputa agentic real resuelta con tu bundle en el export de
   representment.** Ese numero mata a todos los demas.
8. **1 plataforma de spend / procurement** (Coupa ecosystem, Zip, o un ERP mid-market LatAm).
9. **1 merchant / agent app con volumen agentic.** Cobra entre **$2.000 y $8.000/año** a cada uno. Objetivo:
   **logos y feedback, no revenue.**

### Fase 3 — Meses 9-12: compliance product

10. **Scoping opinion de PCI** ($2-5k). **SOC 2 Type I** arranca mes 5-6. Empaqueta el export para auditor
    (SOX 404, COSO feb-2026, PCAOB AS 2201 desde 15-dic-2026, EU AI Act Art. 11) como **compliance tier** de
    $24-60k/año.

### Fase 4 — Meses 12-24

11. **SOC 2 Type II** + pitch a Sardine / Forter / Darwinium / un issuer.
12. **Evaluar el pivot de seguridad:** si a los 12 meses no tenes un issuer o PSP pagando, el pool del
    anti-dispute para **todo** el e-commerce es 100x mas grande. Competis contra Chargeflow y Justt, que ya
    estan financiados — pero es mejor pelear ahi que en un mercado de $14M.

---

## 8. EL DATO QUE FALTA (y el test de $200)

### 8.1 Los tres numeros que no sabés

1. **¿Cuantas disputas reales de compras agentic hubo en Q4-2026 y el win rate con vs. sin evidencia de
   mandato?** Esta en los datos de Mastercard/Visa y en las queues de Chargeflow/Justt/Forter. **Obtenible en
   2 semanas con 4 llamadas.** Si son <5.000 y el win rate no se mueve, el dolor es narrativo.
2. **¿Un controller paga por un audit trail *verificable* o por un *log*?** -> 15 entrevistas (CFO/Internal
   Audit, empresas LatAm con >$50M de spend). Pregunta: *"¿cuanto pagaste el ano pasado por reconstruir un
   audit trail que no tenias?"* Ese numero es tu pricing.
3. **¿El rol de "Verifier neutral" que AEP define queda abierto o lo cierran?** -> el §9.4 y el avance de EMVCo
   (el comentario publico de *Intent Services* cerro el 30-sep-2026 con *"who operates it"* explicitamente
   abierto).

### 8.2 El test de $200 que reemplaza 3 meses de research

Landing + 15 llamadas. Copy:
> *"Verificá que la compra de tu agente cumple la policy. Bundle firmado, verificable offline, sin red."*
> *"A-Comm-compatible. Verifiable Intent compatible. Sin lock-in."*

Si en 30 dias no tenes 3 llamadas, **la tesis no se sostiene en LatAm** y tenes que ir a B2B-US o pivotear a
anti-dispute.

---

## 9. LOS 3 ARGUMENTOS POR LOS QUE NO ES UN NEGOCIO (sin endulzar)

1. **El formato ya existe, con patente y con las redes atras.** VI (Mastercard+Google) define el esquema de
   intencion; AEP define el protocolo de evidencia con grant de patente y angeles de Visa/Mastercard/Amex.
   Tu producto es **una implementacion de standards ajenos**. El unico activo propio es (a) la capa semantica
   y (b) la confianza / root / compliance — que es exactamente el negocio de servicios que termina en
   multiplos sub-1x (ver Oso Cloud).
2. **Gartner: >40% de los proyectos de AI agentico se cancelan antes de fin de 2027**, y la razon principal es
   *"inadequate risk controls"* — o sea: resolves el motivo por el que muere la mitad de la industria, y esa
   mitad lo resuelve a medias y sin prueba.
3. **El take rate esta muerto por aritmetica** (15x el dano) y el pool de agentic commerce es de $14,4M de
   revenue para toda la categoria. El negocio solo existe si el volumen lo aporta un mercado adyacente:
   **disputas (que ya son un mercado de $36,9B) o compliance B2B (que ya tiene presupuesto y deadline
   regulatorio).** Si no logras entrar a ninguno de esos dos, no hay empresa.

---

## 10. Fuentes primarias

**Standards**
- VI spec: <https://github.com/agent-intent/verifiable-intent> — `spec/credential-format.md` (§13.4 #10), `spec/constraints.md` (§2.1, §4.1-4.8)
- AP2 v0.2: <https://ap2-protocol.org/ap2/specification/> (roles, modos, verification del Merchant, extension points, dispute evidence)
- AEP v1.0.3-rc.2: <https://github.com/A-Comm-Tech/a-comm-evidence-protocol> · <https://aep.a-comm.ai/> · <https://a-comm.ai/>
- FIDO: <https://fidoalliance.org/building-the-trust-layer-for-agentic-payments-with-ap2-and-verifiable-intent> · <https://blog.google/products-and-platforms/platforms/google-pay/agent-payments-protocol-fido-alliance>

**Demanda**
- Visa Agentic Commerce Benchmark (5-nov-2026): <https://investor.visa.com/news/news-details/2026/Visa-Releases-First-of-Its-Kind-Industry-Benchmark-for-AI-Shopping-Almost-Half-of-Test-Runs-Fail>
- Analisis del benchmark: <https://ledgerdrift.com/analysis/visa-agentic-benchmark-what-happens-after-the-buy-button>
- Visa Trust Index (sep-2026): <https://corporate.visa.com/en/sites/visa-perspectives/press-releases/visa-trust-index-consumer-insights-into-ai-shopping-payments.html>
- Visa Agentic Ready (18-nov-2026): <https://usa.visa.com/about-visa/newsroom/press-releases.releaseId.22991.html>
- Sardine job post (15-nov-2026): <https://jobs.a16z.com/jobs/sardine/9d67eba9-e460-4409-9e1c-353f55d983c9--lead-agentic-commerce-product-gtm>
- Forter agentic data (18-nov-2026): <https://www.forter.com/blog/agentic-transaction-data/>
- Coupa State of Agentic Procurement (nov-2026): <https://procurementmag.com/news/coupa-data-quality-is-holding-back-ai-agents-in-procurement>
- Worldpay liability: <https://specoria.com/insights/who-pays-when-a-shopping-agent-gets-it-wrong>
- Payments Dive / Elkiner: <https://radlof.com/articles/an-ai-agent-bought-the-wrong-thing-who-pays-refunds-chargebacks-and-liability>

**Volumen**
- Mastercard Encoding Trust via CHEQ: <https://cheq.ai/blog/agentic-commerce-explained/>
- First Page Sage via GEO by Alibaba: <https://geo.alibaba.com/geo-hub/ai-shopping-sessions-vs-agent-orders-2026>
- Allium/Fireblocks: <https://cognitionwire.co/guides/agent-commerce-in-two-numbers>

**Competencia y precedentes**
- Oso Cloud / Tiny: <https://investors.tiny.com/news/news-details/2026/Tiny-Completes-Acquisition-of-Oso-Cloud/default.aspx> · <https://betakit.com/tiny-buys-oso-clouds-assets-for-1-9-million/> · <https://cybermerge.com/stories/oso-split-tiny-temporal-authorization-repriced>
- Gartner: <https://www.gartner.com/en/newsroom/press-releases/2025-06-25-gartner-predicts-over-40-percent-of-agentic-ai-projects-will-be-canceled-by-end-of-2027>
- ar-agents (AP2 Phase 2): <https://github.com/ar-agents/ar-agents/tree/main/packages/agentic-commerce-bridge>
- Mercado Pago Commerce Agents Checkout beta: <https://mercadopago.com.ar/developers/es/news/2026/09/30/Implement-payments-in-your-AI-agent-s-checkout>

**Investigacion de soporte (subagentes)**
- `research/intent-proof-gtm-demanda.md` (500 lineas, 108 fuentes; ledger en `~/AppData/Local/hermes/cache/citations/ledger.json`)
- `research/intent-proof-pricing-report.md` (1.863 palabras, 68 fuentes)
