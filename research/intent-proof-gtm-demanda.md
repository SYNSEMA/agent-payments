# Intent-Proof — GTM y demanda real

**Investigación: 21 de noviembre de 2026.** Todas las cifras van con fuente y fecha. Cuando algo es
estimación del autor o del Mercado, está marcado.

---

## (a) VEREDICTO BRUCO

**El problema existe, es real, está documentado y cuesta plata. El producto ya existe, es gratis, y
lo están construyendo las redes y un consorcio con nombre propio. La demanda pagadora por una capa
independiente de "prueba de intención" es, hoy, casi nula y difícil de defender como negocio independiente.**

Desglose, sin suavidad:

**1. El dolor es genuino y está cuantificado.**
- Los agentes fallan: el benchmark de Visa (5 nov 2026) mandó 22 modelos a 15 escenarios de compra.
  **42,8% de las corridas fallaron**, y en ~54% de esas fallidas **el agente compró igual**. Eso da
  **~23% de todas las corridas = una compra mal o prematura**. Solo el 34,2% pasó las tres
  tentativas. ([Visa](https://usa.visa.com/about-visa/newsroom/press-releases.releaseId.22936.html))
- El costo existe: **261 millones de chargebacks en 2025 → 324 millones en 2028**, valor global
  **$33,79B → $41,69B**. Chargeback promedio al merchant: **$94** (viajes/hotelería: **$120**).
  Los merchants gastan **$100.000–$500.000 por año en tecnología de chargeback**. 45% de los
  chargebacks del merchant son fraudulentos (first+third party).
  ([Mastercard/Datos 2025](https://www.mastercard.com/content/dam/mccom/shared/news-and-trends/insights/2025/2025-global-chargebacks-outlook/pdf/2025-state-of-chargebacks-report.pdf))
- En USA los merchants pierden **$4,61 por cada $1 en chargebacks**
  ([Sift Q4 2025](https://sift.com/index-reports-disputes-q4-2025/)).
- **Worldpay, 30 jul 2026: "No liability shift exists yet."** El merchant sigue siendo merchant of
  record, responde la disputa con los reason codes de siempre, y la evidencia clásica (IP, device,
  sesión) **no sirve** porque la IP es del servidor del agente, no del cliente.
  ([Worldpay](https://worldpay.com/en/insights/articles/agentic-commerce-liability-is-still-being-written))
- **Visa Core Rules 4.1.23 (24 oct 2026)**: el cardholder responde por lo que haga el agente, pero
  **ninguna condición de disputa del Capítulo 11 invoca 4.1.23**. O sea: hay responsabilidad pero no
  hay procedimiento. Y el **4.1.23.3 pone la evidencia en manos del agente**, no del merchant.
  ([chargewood, 26 oct 2026](https://chargewood.com/blog/agentic-commerce-chargebacks))

**2. Pero el hueco ya lo llenaron, gratis, con las mismas primitivas que vos implementaste.**
- **Mastercard + Google, "Verifiable Intent" (5 mar 2026)**: especificación **abierta y open-source**
  en [verifiableintent.dev](https://verifiableintent.dev/) y
  [github.com/agent-intent/verifiable-intent](https://github.com/agent-intent/verifiable-intent/).
  Hace **exactamente tu (a) y tu (c)**: SD-JWT delegation chain que liga issuer→usuario→agente,
  **8 tipos de constraint** (amount bounds, merchant allowlists, budget caps, recurrence terms)
  **criptográficamente bound y machine-verifiable**, selective disclosure, y mapeos de integración
  para AP2/ACP/UCP. Tu advantage de Ed25519/did:key/W3C VC es **literalmente el diseño de la
  spec de Mastercard** (que además se apoya en FIDO, EMVCo, IETF y W3C).
  ([Mastercard](https://www.mastercard.com/us/en/news-and-trends/stories/2026/verifiable-intent.html))
- **A-Comm Evidence Protocol (AEP), 28 oct 2026**: el objetivo declarado es *"an adjudication-ready,
  cryptographically verifiable evidence standard for agentic commerce by capturing the full
  transaction lifecycle from AI-driven discovery to payment authorization and fulfillment"*.
  Eso es tu pitch, textual. Apache 2.0, sin cuotas, sin lock-in, con **15 empresas fundadoras**
  (Action State Group, Anchorage Digital, Axorum, Baselayer, Basis Theory, Chargeflow, Crossmint,
  GoTab, Kyance, Nevermined, People's Court, Rimark, Skyfire, Wink) **más 8 que entraron el
  9 nov 2026** — es decir, ~23. Sardine, Shift4 y Fiserv están en la Agentic Payments Alliance de
  Rain, no en AEP. **The Nilson Report (issue 1314, sept 2026) already covered it.**
  ([PRNewswire](https://www.prnewswire.com/news-releases/a-comm-technologies-inc-forms-founding-consortium-to-establish-open-source-evidence-protocol-for-agentic-commerce-302922270.html),
  [a-comm.ai/aep](https://a-comm.ai/aep))
- **IETF está llenando el mismo espacio, gratis y con más legitimidad**: `draft-stone-aivs`
  (Ed25519 sobre cadena hash, verificación offline self-contained, AIVS-Micro de ~200 bytes),
  `draft-bailey-scitt-agent-session-receipt`, `draft-sharif-agent-audit-trail`,
  `draft-watts-agent-continuity-evidence`. Más ACTIS (open standard de bundles firmados) y Pact
  (auditor pack + CLI offline + "Default Blame Logic").
  ([draft-stone-aivs](https://datatracker.ietf.org/doc/html/draft-stone-aivs-00),
  [draft-sharif-agent-audit-trail](https://datatracker.ietf.org/doc/draft-sharif-agent-audit-trail),
  [Pact](https://pactprotocol.com))
- **Mastercard ya Integrated + Worldpay partner**: "Having an open standard that allows them to
  verify intent, while also giving them the choice to preserve their privacy, is key to increasing
  adoption." O sea, **el procesador de pagos más grande del mundo está empujando la alternativa gratuita a
  tu producto**.

**3. El "problema" ya tiene dueño y presupuesto, pero es otro:**
La capa de evidencia **se está vendiendo embutida** en herramientas que ya tienen presupuesto:
Chargeflow (20.000+ merchants, **$49M funding**, ~$10M revenue jun 2025), Sardine (**$170M
funding**, $660M val), Justt ($32M revenue 2025), Forter, Riskified, Stripe. **Tu producto compite
contra un feature dentro de un suite, no contra un producto.**

**4. La evidencia de que alguien pagaría por tu cosa specifically:**
- **CertifiedData.io** es lo más cerca de tu producto que encontré, y **ya tiene pricing público**:
  certificado **$99**, Evidence Pack **$2.500** (funding price $1.500), Sprint **$12.500**,
  **Evidence Ledger $499/mes**. Ed25519, verificación offline, exactamente tu positioning. Lo que
  esto prueba: **hay alguien vendiendo esto a precio de servicios, no de infraestructura.**
  ([CertifiedData pricing](https://certifieddata.io/pricing))
- **El mejor indicador de presupuesto real que encontré**: Sardine abrió el puesto
  *"Lead, Agentic Commerce (Product & GTM)"* el **15 nov 2026** con un objetivo explícito en el
  announcement: **"hit $10M ARR in year 1"**, siendo el dueño del revenue goal de una línea de
  negocio nueva desde cero. Sardine tiene $170M raised. **Ese es un budget line real, con nombre,
  número y owner.** ([job post](https://jobs.a16z.com/jobs/sardine/9d67eba9-e460-4409-9e1c-353f55d983c9--lead-agentic-commerce-product-gtm))
- **t54 Labs** ($5M seed, feb 2026, Anagram/PL Capital/Franklin Templeton/Ripple) se posiciona
  explícitamente como la "evidence layer" para chargebacks y liability, y ya está en el release de
  Mastercard Agent Pay for Machines: *"a clear evidence layer for agent authorization, chargebacks,
  dispute resolution, and liability review"*. **Es tu competidor más directo y tiene respaldo de
  Franklin Templeton.** ([The Block](https://theblock.co/news/deals/2026-02-25-ripple-franklin-templeton-ai-agent-trust-startup-t54-labs-391273),
  [Mastercard](https://www.mastercard.com/us/en/news-and-trends/press/2026/june/mastercard-launches-agent-pay-for-machines.html))

**VEREDICTO EN UNA LÍNEA:** el problema es real y caro, pero **la capa ya es estándar abierto y el
punto de captura del presupuesto no es una capa neutral de evidencia: es el suite de chargebacks,
el risk vendor, o el wallet del agente**. Como empresa independiente con take rate, **no veo un
buyer #1 que pague hoy**. Como **feature/API vendible a los que sí tienen presupuesto** (Sardine,
Chargeflow, Agentcard, el agent app), sí.

---

## (b) LOS 5–8 BUYERS MÁS PROBABLES

Ordenados por **probabilidad de cheque firmado en 12 meses**, no por tamaño de mercado.

### 1. Chargeflow — el más probable, y también el que te puede comprar

**Dolor concreto:** es el vendor de chargebacks que wrote la "Evidence Playbook for Agentic
Commerce" (jul 2026) y la "AI Agent Chargeback Liability" — o sea, **ellos ya definieron el
problema en su propia voz**. Su página dice: *"Traditional chargeback evidence was built for a
world where humans buy things, and agentic commerce makes it incomplete."*
**Presupuesto:** 20.000+ merchants, $49M funding total ($25M Series A nov 2025 + $10M debt),
~160-186 empleados, ~$10M revenue (jun 2025), revenue triplicado YoY.
**Prueba de presupuesto:** su pricing ya es outcome-based — Prevent **$0,10** por transacción
scanneada, Automation **25%** por chargeback recuperado, Alerts **$29** por chargeback desviado.
Y ya tiene **"Evidence-as-a-Service" (EaaS)** como delivery model para ISPs/PSPs/plataformas.
**Ciclo de venta:** self-serve para merchants SMB; enterprise 4-8 semanas. Un partner API entra
por EaaS/Connect sin tocar el contrato del merchant.
**Argumento de compra:** "Tus datos de mandato y scope no llegan al merchant. El evidence gap que
describís en tu propio blog lo resolvemos nosotros o lo resolvemos con un partner; si no lo
hacemos nosotros, lo va a hacer Mastercard gratis."
**Contra:** **es competidor Y cliente potencial a la vez.** Chargeflow es miembro fundador de AEP,
y AEP hace tu trabajo gratis. Si le vendés la capa, le estás vendiendo lo que su propio consorcio
estandariza gratis.
**Veredicto:** buyer #1 por volumen de evidencia y por budget existente, pero **es también quien
más te puede comprar** (y lo haría como feature o como acquiring).
([Evidence Playbook](https://chargeflow.io/blog/agentic-commerce-chargebacks-the-evidence-playbook-merchants-need),
[pricing](https://www.chargeflow.io/pricing), [EaaS docs](https://docs.chargeflow.io/docs/start-here/product-model),
[funding Ctech](https://www.calcalistech.com/ctechnews/article/bk00v9g9e11g))

### 2. Sardine — el que tiene el budget line más explícito

**Dolor concreto:** tiene un **Disputes Agent** en producción y un **Merchant Risk Agent**, y
necesita exactamente tu primitiva: el score post-checkout que compara la orden contra el mandato.
**Presupuesto:** $170M raised, $660M val (Series C feb 2025, Activant), $25M C-II may 2026
(NAventures), 450+ enterprises (FIS, GoDaddy, Intuit, ZoomInfo, Checkout.com), 6B dispositivos
perfilados.
**Prueba de presupuesto — la más fuerte de todo el informe:** job post del **15 nov 2026**,
*"Lead, Agentic Commerce (Product & GTM)"*, con el texto **"hit $10M ARR in year 1"** y
responsabilidad de "pricing, packaging, design partners, pilots, and launch" + partnerships con
"merchants, PSPs, card networks, and AI agent platforms".
**Ciclo de venta:** enterprise, 3-6 meses, con pilot como primer paso.
**Argumento de compra:** "Necesitamos la señal 'el agente se mantuvo dentro del scope' separada
de 'el agente está autenticado' para cobrar mejor y defender representment. Hoy no la tenemos."
**Contra:** Sardine **está en la Agentic Payments Alliance de Rain** (Visa, Mastercard, Fiserv,
Circle, Solana, Crossmint, Basis Theory, Sardine, Shift4...) y tiene su propio Disputes Agent. Te
compraría para acelerar, no para existir.
([job post](https://jobs.a16z.com/jobs/sardine/9d67eba9-e460-4409-9e1c-353f55d983c9--lead-agentic-commerce-product-gtm),
[Sardine Series C](https://sardine.ai/press/pr/fraud-and-compliance-productivity))

### 3. Los agent apps / agent wallets — el que tiene el dolor más directo al usuario

**Dolor concreto:** es TU comprador ideal conceptualmente. El agent app es quien tiene el mandato,
la instrucción y el contexto. Si compra hotel <$600 con cancelación gratis y el agente compra
tarifa no reembolsable, **el que pierde plata y clientes es el agent app, no el merchant.**
**Presupuesto real:** **Instinct** dice **$1B+ de volumen anualizado de transacciones**, **40% de
usuarios conecta tarjeta personal en 3 semanas**, **$1.300/mes promedio de gasto entre quienes
compran**, **100.000+ usuarios en 3 semanas** (ago-sept 2026), valoración **$10B** (desde ~$50M
seed en 4 meses; reportedly raising at $30B). **Meta Muse**: 10M usuarios, 250k DAU en la primera
semana, 5M downloads en <1 mes. **Agentcard** ($8M seed nov 2026, YC S26) ya tiene como clientes a
Orchid, Almanac y Laso.
**Ciclo de venta:** B2B self-serve, 2-4 semanas. **Es el ciclo más corto de todos los de esta lista.**
**Argumento de compra:** "Vendemos 'tu agente compra bien'. Si un reviewer ve que le compramos un
vuelo no reembolsable, la confianza y el churn se nos caen. Necesitamos un recibo verificable."
**Contra — y es grave:** **hoy estos agentes monetizan con take rate sobre merchants, no sobre
infraestructura.** Solo **2% de los 44 productos AI-native web de a16z** monetizan vía transaction
o platform fees. Y el cálculo de unit economics publicado (Interspace, 5 nov 2026) da **-$1,93 por
booking** de vuelos a take rate 0,7%. **No tienen margen para un SaaS de evidencia.** A no ser que
cobren por transacción — y entonces tu take rate tiene que competir contra su propio take rate.
([a16z Top 100 7th ed, 5 oct 2026](https://a16z.com/100-gen-ai-apps-7/),
[Skift](https://skift.com/2026/09/28/instincts-app-is-fueled-by-travel-founder-says-its-half-of-volume/),
[Interspace unit economics](https://posts.interspace.ventures/p/mythbusters-the-unit-economics-of),
[Agentcard $8M](https://agentcard.com/blog/agentcard-raises-8m),
[Agentcard pricing](https://agentcard.com/pricing))

### 4. Los PSP/acquirers grandes (Adyen, Checkout.com, Fiserv, Global Payments, Worldline, dLocal)

**Dolor concreto:** el gap de visibilidad explícito. PwC (30 oct 2026, con EQTY Lab) lo framea
como "the vanishing point": *"Neither ISO 8583, ISO 20022, nor 3DS may convey key signals such as
customer intent in a uniform way to all stakeholders."*
**Presupuesto:** hay presupuesto, pero **es presupuesto de plataforma, no de una capa neutral.**
**Prueba de movimiento:** Visa Agentic Ready se abrió a la acceptance chain el **18 nov 2026** con
**140+ participantes** (Adyen, Antom, Billtrust, Checkout.com, Fiserv, Global Payments, J.P. Morgan
Payments, PayPal, Stripe, Moneris, VGS, Worldline) más **175+ emisores en 45 mercados**; Europa 85+,
APAC 50+. dLocal y Getnet participan. **Todos esos players están construyendo internamente o vía
red, no comprando.**
**Ciclo de venta:** 6-18 meses, con RFP/procurement.
**Argumento de compra:** "Necesitamos un formato de evidencia interoperable entre nuestras señales
propias y las de las redes, que no nos ate a un solo proveedor."
**Contra:** la respuesta obvia es "usemos AEP, que es Apache 2.0, tiene ~23 miembros incluyendo a
nosotros, y ya está en el Nilson Report."
([Visa US 18 nov 2026](https://usa.visa.com/about-visa/newsroom/press-releases.releaseId.22991.html),
[Visa Europa](https://visa.co.uk/about-visa/newsroom/press-releases.3478924.html),
[PwC](https://www.pwc.com/us/en/technology/trust-ai/agentic-commerce-new-layer-of-visibility.html))

### 5. Los merchants grandes (travel,Ticketing, retail de alto valor)

**Dolor concreto:** por arriba del arXiv 2610.17378 (12 oct 2026), que construyó un benchmark de
"decision sufficiency" — *¿la evidencia publicada del merchant alcanza para determinar si un
mandato concreto se cumple?* (costo total, condiciones de cancelación/devolución, fecha de
servicio, variante). Resultado: de **330 merchants**, solo **18 (5%)** tenían el **precio total
decidible** desde sus propios términos publicados. En el piloto, **3 de 4 merchants grandes
rechazaron los perfiles declarados** y **ningún mandato pudo confirmarse como cumplido** desde los
términos publicados.
**Traducción directa a tu pitch:** el merchant no publica condiciones suficientemente precisas para
que un agente decida bien, y por lo tanto **nadie puede probar después que la compra cumplió la
intención.** Eso es exactamente tu componente (c).
**Presupuesto:** los merchants que YA pagan por chargebacks ($100-500K/año en tech de
chargeback) y por compliance.
**Ciclo de venta:** 3-12 meses, con legal y risk en el loop.
**Contra:** **el buyer real del merchant es el chargeback vendor, no tu API.** El merchant quiere
"resolver disputas", no "recibir un bundle firmado". Y mientras el pain de volumen sea bajo, no hay
urgencia.
([arXiv:2610.17378](https://arxiv.org/abs/2610.17378))

### 6. Los bancos / emisores (el que tiene el presupuesto pero el ciclo más lento)

**Dolor concreto:** seis bancos (NatWest, Bank of America, ING, ASB, Capital One, CBA) publicaron
principios compartidos de "trusted agentic commerce" (22 sep 2026) pidiendo **disclosure cuando
hay agente, transparencia sobre cómo decide el agente y safeguards de datos**. El gap regulatorio
es real: **no hay guidance de CFPB para agentes**, y **Regulation E / Reg Z no tienen categoría
para "un agente malinterpretó la instrucción"** (Worldpay 30 jul 2026; [Gupta Deepak, nov 2026](https://guptadeepak.com/ai-agent-wrong-purchase-who-pays)).
**Presupuesto:** enorme, pero...
**Ciclo de venta:** **12–18 meses** (el CEO de Baselayer lo dice explícitamente, y es un partner de
Visa/Mastercard).
**Contra mortal:** **los bancos son los que escriben las reglas.** Si el banco define el estándar,
no compra un estándar de un startup. Y PwC ya recomienda KYA como el marco ("KYA is the new KYC").
([CNBC 22 sep 2026](https://www.cnbc.com/2026/09/22/banks-warn-ai-shopping-bots-raise-scam-fraud-data-privacy-risks.html),
[Worldpay](https://worldpay.com/en/insights/articles/agentic-commerce-liability-is-still-being-written),
[Baselayer job post](https://career.eoss.asu.edu/jobs/baselayer-partnerships-manager))

### 7. Las redes (Visa/Mastercard) — Rule out

No son buyers: **son la competencia**. Ya te free-aron. Verifiable Intent (open source, con
Worldpay partner), AEP (con Chargeflow, Crossmint, Basis Theory, Skyfire, Sardine y **Visa y
Mastercard también en la Agentic Payments Alliance** de Rain, 26 miembros fundadores, 18 ago 2026).
Tu única chance acá es **venderles la implementación/operación** (que es consultancy, no producto).
([Rain APA](https://www.rain.xyz/resources/rain-launches-the-agentic-payments-alliance))

### 8. LATAM — el ángulo que sí está verde, pero el mercado es chico

**Lo que hay (con fecha):**
- **Mercado Pago** lanzó **Commerce Agents Checkout** (beta, open source Python) el **24 sept 2026**:
  valida el carrito contra el catálogo propio antes de crear la order, **el modelo nunca decide el
  precio** y **nunca ve la URL de pago**; se integra con `anthropics/commerce-agents` vía
  `checkout_handoff`. El README es, literalmente, **una especificación de intent-proof** escrita por
  un gigante. **Y es gratis.** ([GitHub](https://github.com/mercadopago/commerce-agents-checkout),
  [news](https://www.mercadopago.com.ar/developers/pt/news/2026/09/30/Implement-payments-in-your-AI-agent-s-checkout))
- **Mercado Pago Mago** (28 sept 2026): asistente con 100+ funciones en **MX, AR, BR y CL**;
  **17M de usuarios** que usaron la versión previa desde oct 2025; **88M MAU** en la región (+30%
  YoY); **TPV >$100.000M** en Q2 2026 por primera vez.
- **PayPal**: "Agent Ready" + "Store Sync" + Agentic Commerce Protocol, con onboarding **gated por
  formulario** ("request access", el equipo de AI te contacta). Vendor lock-in suave.
- **dLocal**: quoting text en el anuncio de **AP2** de Google ("Payment agents are no longer an
  idea..."), y participante de Visa Agentic Ready en APAC.
- **PayU**: en la lista europea de Visa Agentic Ready (18 nov 2026).
- **Pagos reales con agentes en LATAM, no demos**:
  - **Mastercard (marzo 2026)**: transacciones agentic en vivo en la región con **BAC, Banco de la
    Nación Argentina, Banco Falabella, Banco Galicia, Banco Itaú, Banco Ripley, Banco Security de
    BICE, Bancolombia, Banamex, Cencosud-Scotiabank, Davivienda, Dock, Evertec, Oriental Bank,
    Pomelo, Santander y ueno bank** — comprando makeup, groceries, libros y goods digitales.
  - **Visa + Santander (marzo 2026)**: piloto en **5 mercados (AR, BR, CL, MX, UY)**; compras de
    **libros en Argentina, Chile, México y Uruguay** y **bombones en Brasil**.
- **Regulación Argentina que empuja la necesidad de prueba:** el **Anteproyecto de Ley General de
  Sociedades** (firmado 28 may 2026, en el Senado desde 1 jun 2026, **todavía no es ley**) crea la
  **"Sociedad Automatizada"** (art. 14), con deber de **configuración y supervisión** (art. 102) y
  responsabilidad patrimonial (art. 1757 CCCN). Un humano responsable es obligatorio desde la
  modificación del 19 ago 2026.
  **Y el RFC local ya existe:** `ar-agents` RFC-001 *"Identidad y firma de agentes en Argentina"*
  (Nazareno Clemente, draft-01, 8 may 2026, DOI Zenodo) — propone **adoptar AP2 como formato
  estándar de órdenes de pago grandes**, y alinea la sección de audit trail con
  `draft-sharif-agent-audit-trail` de la IETF. **Es el mismo tipo de documento que hiciste vos, con
  un mes de ventaja y escrito en el jurisdiction correcto.**
- **México**: **Ley de Economía Digital** publicada en el DOF el 4 nov 2026, en vigor el 5 nov 2026 —
  habilita pago digital obligatorio por sectores ( Hacienda tiene 15 días hábiles para definir
  sectores).
- **Brasil**: Pix ya es el **56% de las transacciones** (22.900M en Q2 2026), pero en valor la TED
  mueve más (R$12,1 billones vs R$10,5 del Pix). LGPD no es el driver; el driver es el pix de valor
  alto.
- **Un startup local que YA hace tu capa, gratis:** **Afentic** (afentic.com) — "límites,
  aprobación, firma", ECHEQ/factoring con topes, **auditoría completa de cada proceso**, y ya
  connects con **Pomelo, Banco Galicia, Banco Nación y Santander** en pilotos de pagos agénticos en
  la región. **Competidor local en tu propio terreno.**

**Veredicto LATAM:** **el problema es más real que en USA (pocos jugadores, pilotos reales,
regulación de identidad empujando) pero el mercado es 10x más chico y la única evidencia layer
grande (Mercado Pago) la da gratis en open source.** Tu ventaja ahí es **idiosyncratia normativa y
proximidad**, no tecnología.

---

## (b-bis) DIMENSIONAMIENTO HONESTO DEL MERCADO

### El mercado al que apuntás, en números

**El mercado de la evidencia como producto:** no existe una categoría con revenue propio. Los tres
puntos de comparación que existen:

| Referencia | Tamaño / precio | Qué te dice |
|---|---|---|
| **Chargeback management software** (mercado adyacente real) | **$1,83B de revenue de vendor en 2025 → $4,49B en 2034** (CAGR 10,5%, Dataintelo, rev. 8 nov 2026); **$2,3B (2025) → $9,7B (2035)** (Banking Industry Insights, oct 2026) | Es el pool donde vive tu buyer, no tu producto |
| **Justt** | **$32M revenue en 2025**, 400 clientes (venían de 225 en 2024), $100M funding a $170M val | El techo realisticamente alcanzable para un vendor de disputa bien posicionado |
| **Chargeflow** | ~$10M revenue (jun 2025, founder-stated), revenue triplicado YoY, 20.000+ merchants, $49M funding | Un player grande de ese mercado factura 10-30M, no 100M+ |

**Take rates comparables — la pregunta es qué cobra una capa de infraestructura que agrega valor
sobre volumen:**

| Producto | Take rate / precio | Contexto |
|---|---|---|
| **Stripe Billing** (infraestructura de billing sobre el pago) | **0,7%** del volume pay-as-you-go, o **$25–$1.650/mes + 0,67%** | Una capa que hace una cosa, embebida en la infra de Stripe, cobra **0,7%** |
| **Stripe Revenue Recognition** (capa contable sobre el pago) | **$25–$1.650/mes + 0,2%** del volume adicional | Precedente casi perfecto de "capa de infraestructura que cobra take rate sobre volumen" |
| **Shopify Plus** | **$2.500/mes** (1 año) / $2.300 (3 años), más variable platform fee | Lo que un merchant grande paga por plataforma, sin volumen agentic encima |
| **Lemon Squeezy / Paddle / Polar** (merchant of record) | **5% + $0,50** (Lemon Squeezy suma 1,5% intl + 1,5% PayPal + 0,5% suscripción) | MoR cambia el vendedor legal y por eso cobra más |
| **Agentcard opencommerce** (el análogo más cercano a tu GTM) | **1% de cada orden completada**; Issuing **$5.000/mes + 1% revenue share**; Vault gratis hasta 5.000 usuarios | **El fee de "infraestructura de agente" que un merchant ya paga hoy es 1%** |
| **Chargeflow** (outcome-based) | **25%** por chargeback recuperado, **$29** por desviado, **$0,10** por transacción scanneada | Cobra por el **outcome**, no por la evidencia |
| **Chargeflow EaaS / Connect** | white-label para ISPs y PSPs | Ya existe el canal "vendé tu capa al que ya tiene los merchants" |

**Conclusión de pricing: si tu producto vale 1% del volumen agentic, entrás en el mismo rango que
el fee que Agentcard ya le cobra al merchant — o sea, no hay room. Si vale 25% por chargeback
recuperado, estás compitiendo con Chargeflow en su propio terreno con su propia distribución.**

### El mercado de agent apps que te compra

- **a16z Top 100 Gen AI Consumer Apps, 7ª ed. (5 oct 2026)**: de los 44 productos AI-native web,
  **solo el 2% monetiza vía transaction o platform fees** — el 84% cobra suscripción, el 64% usage.
  O sea: **la mayoría de las agent apps no tiene take rate todavía.** Instinct y Muse "have
  described plans to earn a fee from transactions over time, though neither is doing this yet".
- **Instinct**: **$1B+ de volumen anualizado**, 100.000+ usuarios, valoración **$10B**. A 0,7% de
  take (el promedio real de OTAs/affiliates) eso son **$3,12 de revenue por booking** contra un
  costo de **$5,05** por booking (7 sesiones × $0,72 de inferencia). **-$1,93 por booking.**
  O sea: **la unidad económica del agente es negativa antes de cualquier capa de evidencia.**
- **Muse**: 10M usuarios, 250k DAU primera semana. Pero **Amazon le cortó el acceso** en menos de
  2 semanas y Walmart limitó agentes a perfiles con aprobación por defecto y límites de $50/$100/
  $200.
- **Cuántos agent apps hacen compras con volumen real:** Forter (16 nov 2026) ve **agentic orders
  up 7,9x entre mediados de agosto y mediados de noviembre**, con Muse al **49%** de las órdenes
  agentic de la semana del 2 de nov. Pero **el 80,5% de las órdenes agentic son scraping agents**
  (protocolos: 1,2% → 80,5%; el resto es browser automation). **Tu capa, que requiere
  integración del agent app, no ve el 80% de las órdenes agentic.** Eso es una restricción
  estructural de GTM que casi nadie dice en voz alta.
- **Base instalada de agent apps**: Dealroom mapea **112 startups** de pagos para agentes (81
  fundados en 2025-2026), con **$115M de venture funding en 2026** (14 rondas) vs $81M en 2025 —
  o sea, **el capital está entrando fuerte pero el mercado es de ~112 companies en todo el mundo,
  y 17 de ellas son "trust and risk"**. **Si tu buyer #1 es un agent app, tenés ~112 prospectos en
  el mundo, menos de 40 con volumen real, y ninguno con margen.**

**El número que define el techo:** el modelo de fee pool de Second Breakfast (27 sept 2026) calcula
que los **agent-layer fees** — toda la infraestructura nueva de agentes, no solo la evidencia —
son **$0,13B–$0,29B en 2030**, mientras que **~94% del pool total va a redes, emisores y
processors**. Tu producto tendría que capturar una fracción de ese $130–290M, y para eso
necesitás un take rate que hoy ningún agent app paga por una capa de evidencia.

---

## (c) 3 ARGUMENTOS POR LOS QUE ESTO PODRÍA NO SER UN NEGOCIO

### 1. El producto ya existe, es gratis, y lo construyeron las redes + un consorcio con nombre propio

Verifiable Intent (Mastercard+Google, open source, con Worldpay adjoint), AEP (Apache 2.0, sin
cuotas, sin lock-in, ~23 miembros que son tus clientes potenciales), AP2 (Google), TAP (Visa),
Actis, Pact, y cuatro drafts de IETF. Todo con las mismas primitivas criptográficas que vos
implementaste: Ed25519, cadenas hash, verificación offline, selective disclosure.

No tenés un moat técnico — **el moat técnico lo construyeron Mastercard y Google en marzo 2026 y
lo que sigue siendo software libre**. Tu moat real solo puede ser **distribución, confianza o un relationship
que ya tengas** (y no tenés ninguno de los tres: es un founder nuevo con un producto nuevo).

**Test público para saber si esto es cierto:** si un merchant puede hoy resolver tu caso con AEP +
un `did:key` + un verifier offline, **tu producto es un SDK, no una empresa**.

### 2. La capa neutral no captura el presupuesto: el presupuesto está en el outcome

La evidencia de que la industria paga por la *evidencia* la da Chargeflow con su modelo
**outcome-based** ($0,10/transacción escaneada, 25% por chargeback recuperado, $29 por
desviado) y el mercado de chargeback management: **$1,83B de revenue de vendor en 2025 →
$4,49B en 2034** (Dataintelo, rev. 8 nov 2026); **$2,3B en 2025 → $9,7B en 2035** (Banking
Industry Insights, oct 2026).

Esos vendors **no cobran por el bundle firmado**: cobran por el resultado (recuperar el
chargeback, desviarlo, resolver el caso). Y ya tienen **20.000+** y **400** clientes
respectivamente, con equipos de enterprise sales.

Un producto que cobra **por evidencia** compite contra **"gratis, dentro del suite que ya
comprás"**. Para cobrar por la evidencia, el merchant tiene que creer que con evidencia **gana
más disputes** — y hoy **ninguna red publica un win rate para disputas de agente**, porque no
existe el caso. Estás vendiendo insurance antes de que exista el riesgo.

### 3. La demanda de agentic commerce todavía no es volumen, y cuando llega, el volumen se lo lleva el incumbent

Los números que matan el momento:
- **Solo el 3% de las transacciones de los merchants UK/US involucran agentes**, aunque el 89% dice
  estar preparándose y el 42% ya está testeando (Checkout.com Agentic Commerce 2026, junio 2026,
  Censuswide: 12.005 consumidores en 6 países + 400 heads of payment).
- **OpenAI retrocedió con Instant Checkout en marzo 2026**: launched 29 sept 2025, **scaled back el 5 mar
  2026**, anuncio oficial el 24 mar. Razón declarada: *"the initial version of Instant Checkout did
  not offer the level of flexibility that we aspire to provide"*. Resultado: **~12 merchants de
  Shopify** (o ~30 según el propio Shopify) de **millones** conectados. Razón de fondo, según
  The Information: los usuarios investigaban en ChatGPT pero compraban en otro lado.
- **Walmart**: las compras adentro de ChatGPT convertían **a un tercio** del rate de los click-outs
  al sitio propio (citado por Second Breakfast).
- **El fee pool que los agentes generan es chico y va a los incumbents**: el modelo de Second
  Breakfast (27 sept 2026) calcula **~94% del fee pool de 2030 para redes, emisores y
  processors**; los **agent-layer fees son solo $0,13B–$0,29B en 2030** y el volumen wash-filtered
  de x402 es **~$0,9M/mes**. Los protocolos no cobran nada y casi no mueve plata: **x402 settled
  ~$240.000 en la semana al 26 sept 2026, 98% por debajo de su pico**.
- Y el punto que te mata si apuntás al merchant: **la tasa de disputa de agente no es conocida por
  nadie**. "Agent-error disputes in 2026 look like counterfeit card fraud before October 2015."
  No podés vender un producto para un riesgo que no tiene número medido.

**Consecuencia brutal:** el que paga por evidencia de agente **hoy** es el que ya tiene el riesgo
— y ese es el **issuer**, que tiene ciclos de 12-18 meses, o el **agent app**, que no tiene
margen.

### Bonus (4to, para ser honesto): la categoría se está consolidando ANTES de madurar, y los independientes están siendo comprados

IDSync (*The State of AI Agent Identity 2026*, 8 ago 2026): **8 adquisiciones en 8 meses** —
Veza→ServiceNow (~$1,2B), Oasis→Cyera (~$1B), Astrix→Cisco (~$400M), SGNL→CrowdStrike (~$740M),
StrongDM→Delinea, Natoma→Snowflake, Entro→SailPoint, Permiso→Okta (~$200M).
**Oso Cloud** — el player de authorization más cercano a tu thesis — **fue liquidado por $1,9M CAD
contra $5,3M ARR**: un multiple de 0,36x. Tiny (Oct 2026). **El precedente de precio de salida de
un vendor de authorization pura es un desplome.** Y WorkOS cerró $100M a $2B — o sea, **lo que se
vende es distribución de auth, no capas de evidencia criptográfica**.

---

## (d) QUÉ DATO FALTARÍA PARA ESTAR SEGURO

### El dato que decide todo (y es de una sola pregunta)

**¿Cuántas disputas reales de compras hechas por agentes hubo en Q4 2026 y cuál fue el win rate
con y sin evidencia de mandato?**

Ese número existe en tres lugares y ninguno lo publica: los Data & Analytics de Mastercard, el
*Dispute Monitoring Report* de Visa, y las queues de Chargeflow/Justt/Forter. Si en la temporada
Q4 2026 (que es cuando los volumes se muestran, y AEP yapredice "dispute season for Q4
transactions" en **Q1 2027**) hay **>5.000 disputas de agente en US**, y **ganar con evidencia de
mandato sube el win rate >15pp**, entonces hay un producto. Si hay 300 o el win rate no se mueve,
entonces el dolor es **narrativo, no económico**.

**Cómo obtenerlo sin esperar:** 4 llamadas, 2 semanas.
1. **Chargeflow y Justt** — preguntar el delta de disputes en su base entre Q3 y Q4 2026 y si pueden
   devolver un count desagregado por "agent order" (ellos ya tienen el tag de agente en Forter y
   el tag de canal en Stripe/Shopify).
2. **Un merchant de travel de $50M+** — “¿cuántos disputes de agent orders tuviste en Q4 y
   cuántos ganaste?" Un merchant grande de travel con volumen agentic va a tener el dato.
3. **Mastercard/Visa** — el "Agentic Commerce dispute data" de EMVCo/Agentic Payments Task Force.
4. **El issuer de un agent app** — Instinct o Muse con su PSP: “¿cuántos chargebacks les
   originatoraron en Q4 y de qué tipo?"

### Los otros 4 datos que faltan, en orden de importance

**1. El "take rate test" con un agent app real.** ¿Un agent app con volumen real pagaría un fee
por transacción por evidencia? La pregunta killer: **en Agentcard opencommerce ya cobran 1% por
orden completado** (Issuing: **$5.000/mes + 1% revenue share**). **Si Agentcard ya cobra 1% por
orden, entonces el merchant ya está pagando un fee de "infraestructura de agente" y tu capa
tiene que entrar como delta dentro de ese 1% o no entra.** Preguntarle a un customer de Agentcard
(Orchid, Almanac, Laso) cuánto les cuesta hoy cada dispute de agent order y si lo resolverían
pagando.
**Timeline:** 2 semanas. **Decision gate:** si la respuesta es "no, lo resolvemos con vendor
gratis", cerrá el track de agent apps.

**2. La prueba de que Mastercard NO va a absorber tu primitiva.** Verifiable Intent ya cubre
identity + delegation + constraints + selective disclosure. **Lo que NO cubre, según tu propia
documentación de Mastercard, es la verificación de *fidelity* — que la compra cumpla los términos
materiales** (cancelación gratuita, deadline, tipo de habitación). Eso es exactamente tu (c), y es
un hueco que Vijil está atacando por el lado del comportamiento ("the fidelity gap") y que el
arXiv 2610.17378 está midiendo del lado del merchant ("decision sufficiency").
**Pregunta a verificar:** ¿Visa/Mastercard van a agregar enforcement de constraints a Verifiable
Intent en 2027? Si sí, tu (c) desaparece. **Timeline:** 2 meses de lectura de spec + attending
FIDO Payments TWG + IETF drafts.

**3. Volumen de agentic commerce en LATAM medido, no inferido.** Mercado Pago dio **17M usuarios
de Mago** y **88M MAU** regional, pero **no publica cuántos son agent orders**. En USA, Forter
(16 nov 2026) ve **agentic orders up 7,9x entre mediados de agosto y mediados de noviembre**, con
**Muse al 49% de las órdenes agentic** de la semana del 2 de nov, y **scraping agents al 80,5%**
(protocolos solo 1,2%→creciendo). **Si el 80% de las órdenes agentic son scraping (el agente
escribe en tu checkout), entonces tu capa — que requiere integración del agent app — no las ve.**
Eso es una restricción estructural brutal para tu GTM y nadie la está discutiendo en voz alta.
**Timeline:** 1 mes de data de Forter + 1 llamada a Mercado Pago + preguntar a Pomelo/Nuvea/dLocal.

**4. Un pagador que no sea de USA ni una red.** El único caso donde una capa neutral sobrevive es
**una LATAM/UE donde la norma local pida prueba y el estándar global no la cubra**. El
anteproyecto de Ley General de Sociedades (Sociedad Automatizada, art. 14 y 102) es el hook más
concreto que tenés: si exige **auditar qué agente actuó, bajo qué regla, quién aprobó** — y eso
requiere un registro firmado verificable por un tercero — entonces hay un comprador regulado.
**Pregunta a verificar:** ¿el texto de la versión del 19 ago 2026 (con responsable humano) exige
un registro verificable por terceros, o alcanza con un log interno? **Timeline:** 1 lectura del
texto + 1 consulta a un abogado societario. **Esto es barato y es tu único shot de
diferenciación real.**

---

## RECOMENDACIÓN OPERATIVA (3 líneas)

1. **No arranques vendiendo "la capa".** Vendé **un resultado**: "ganás el 80% de los disputes de
   agent orders en vez del 30%". El bundle firmado es el mecanismo, no el producto. Modelalo con
   el pricing de Chargeflow (outcome-based), no con un SaaS por asiento.
2. **Apuntá a 2 buyers, no a 10:** (a) **Sardine** — tienen el budget line de $10M ARR en agentic
   commerce y necesitan la señal post-checkout; (b) **un chargeback vendor** (Chargeflow, Justt o
   uno nuevo en LatAm) — que ya tiene los merchants y el outcome. Vendé **API/SDK white-label**,
   no plataforma.
3. **Corré el dato de la pregunta 1 antes de escribir una línea de código.** Si las disputas de
   agente no aparecen en Q4 2026, el mercado no existe todavía — y tu mejor activo es esperar con
   evidencia, no quemarlo construyendo contra un estándar gratis de Mastercard.
