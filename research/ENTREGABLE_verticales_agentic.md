# Qué compran HOY los agentes de IA — y dónde hay un producto de evidencia de intención

**Preparado para:** founder (Rosario, AR) — 22 de noviembre de 2026
**Ventana de datos:** ago-2026 → nov-2026. Fecha de corte de fuentes: 22-nov-2026.

**Regla de lectura de este documento:**
- **[MEDIDO]** = dato transaccional u operacional de una fuente primaria (API pública, earnings call, texto de regla, paper con datos).
- **[VENDOR]** = cifra que una empresa reporta sobre sí misma o su propia red. Sirve para dimensionar, no para extrapolar al mercado.
- **[DERIVADO]** = cálculo mío sobre datos publicados. Va con la fórmula a la vista.

---

## 0. Tres correcciones al brief antes de todo

### 0.1 El 23,1% de "compra equivocada" no es un número de Visa — es un número derivado, y Visa no mide lo que creés

Visa **no** publica 23,1%. Publica dos cosas separadas (press release, 5-nov-2026, 22 modelos de 12 labs, 15 escenarios de compra):[5]

- **42,8%** de las corridas fallan.
- **54%** de esas corridas fallidas **igualmente terminan en compra**.

23,1% = 0,428 × 0,54. La cuenta da, y es defendible como derivación — pero hay que decir de dónde sale y, sobre todo, **qué mide**:

> "About 1-in-2 (54%) of failed runs still ended in the AI making a purchase. Either the AI recommended a purchase that did not meet the customer's identified requirements, or it failed to uncover all the customer's requirements."

Dos consecuencias que cambian el pitch:

1. **El 23,1% es "corridas que compraron mal O no descubrieron todo lo que había que descubrir"**. No es "compraron el producto equivocado" puro. La distinción importa: la primera mitad es un bug de decisión del agente; la segunda mitad es el agente **no preguntando**.
2. **El benchmark de Visa no cubre ropa ni comida.** Los 15 escenarios son "TV, laptop, personal care appliance" — electrónica de especificación. [5] No podés usar el 23,1% como "23% de las compras por agentes están mal" en general. Es un dato de **compras con especificación técnica explícita**, y por lo tanto es el mejor dato disponible *justamente* para la vertical que te conviene.

**Lo que sí es medido y relevante:** la consistencia es el problema real. "All three attempts passed just 34.2% of the time" — un modelo que pasó un escenario suele fallar el mismo escenario en el intento siguiente.[5] Y los cinco modelos que más preguntaronSacaron 70,2% de approval, contra 44,4% de los que menos preguntaron.[5] El gaps de 25,8 puntos entre mejor y peor preguntador es más ancho que el de 16,9 puntos entre modelo caro y barato.[5]

### 0.2 La CE3.0 **no** hace obligatoria la evidencia — es opcional, y el cambio de agentes es chico

Tu brief dice "Visa Compelling Evidence 3.0 desde 24-oct-2026 hace la evidencia obligatoria en disputa". Falso en dos puntos:

- **CE3.0 es una *remedy rule* opcional.** El texto de las reglas dice literalmente "An Acquirer **may** submit Compelling Evidence".[19][20] Además solo aplica a la condición de disputa **10.4** (Other Fraud, Card-Absent Environment) — ni 13.1 (not received), ni 13.3 (not as described), ni 13.2 (canceled recurring).
- **El cambio del 24-oct-2026 es estrecho y es del lado del acquirer, no del merchant.** Lo que entró: las dos transacciones previas pueden venir de "one or more Merchants" (antes: la misma credencial de pago), y el elemento "customer account or login ID" ahora incluye explícitamente "login IDs for an Agentic Payment Provider".[19][20] Eso es todo.
- **Y el tope duro**: el footnote 5 de Table 11-31 limita al acquirer a "Transaction data accepted and processed by that Acquirer".[19] O sea: la evidencia cross-merchant que el fondoarde no te la da el merchant — la presenta *tu acquirer*, y solo de su propio libro. Para un merchant, la CE3.0 cross-merchant es teórica y no accionable por vos.[19]

**El driver regulatorio real es el contrario al que pensás.** No es "la evidencia se volvió obligatoria". Es que **no existe todavía ningún mecanismo de resolución de disputas para compras agentic**:

- Los Core Rules de Visa (edición 24-oct-2026) **no tienen ni un reason code, ni una condición de disputa, ni una fila de response table que nombre una Agentic Transaction**.[20] El capítulo 11 (Dispute Resolution) menciona "agent" únicamente en las dos líneas de login ID de CE3.0.[20]
- El reporte de pagos agentic de Visa (julio 2026) dice: *"disputes lack established resolution mechanisms. Chargeback windows and evidence requirements were designed for human-speed commerce with clear buyer intent"* y *"liability remains legally ambiguous. No jurisdiction has established clear precedent for how responsibility is distributed among the human who delegated authority, the platform that hosted the agent, the model provider, and the merchant."*[20]
- Y el reparto de responsabilidad es tabulado y está en tu contra: **el cardholder responde por el agente "as if the Cardholder initiated the Transaction"** (§4.1.23.10), mientras el merchant no puede obligar a renunciar al derecho a disputa (§5.4.2.5) y **no tiene derecho a los dos registros que más importan**: el agente retiene la orden de confirmación y la aceptación de políticas de devolución por al menos 120 días, y debe entregarlas al cardholder o al issuer "upon written request" — sin obligación de dárselas al merchant.[20]

Reformulado: **el vacío de mercado no es "falta evidencia". Es que la evidencia de intención existe, pero está en el lado equivocado y el merchant no tiene derecho a obtenerla.** Ese es tu producto, y es un problema de *derechos de acceso y litigio*, no de obligatoriedad regulatoria.

### 0.3 Las cifras de Forter se mueven todos los días — citá la fecha o no citemos nada

Tus números (7,2x, $227 vs $133, food & delivery $117 vs $57 = 2,1x, digital $145 vs $99 = 1,5x, 60% de merchants, 9% >100) están **verificados textualmente** contra el blog del 18-nov-2026.[1] El 1 en 10.000 también: *"About 1 in 10,000 orders on Forter's network is now placed by an AI agent"* — red de Forter, no el mercado.[1]

El problema: el dashboard vivo de Forter **contradice a sí mismo entre renders del mismo día**. Recupéré tres versiones distintas:

| Fuente | Fecha | Multiplicador | AOV agente | Scrapers | NA vs ACho |
|---|---|---|---|---|---|
| Blog | 18-nov-2026 | 7,2x | $227 | 77% (mid-oct) | no publicado |
| Página HTML | 21-nov-2026 | 8,6x | $229 | 68% | NA 87% / ACho 6% |
| Versión `.md` | 21-nov-2026 | 11,1x | $221 | 72% | NA 73% / ACho 21% |
| API JSON | 22-nov-2026 | 8,6x | $229 | 68% | NA 87% / ACho 6% |

El `.md` dice además "60% de las órdenes de los últimos 7 días fueron Muse" contra "51%" del HTML, y travel a $531 contra $515.[2][3] **Regla: usá el blog fechado (18-nov) para números que van a un deck, y el dashboard solo como dirección.** Si citás "8,2x" el día 21 y "11,1x" el día 22, te desarman el deck.

**Lo que sí hay que destacar de Forter:** es la **única** fuente que publica el desglose por vertical **y** AOV por vertical **y** mix de agentes por vertical, con API pública sin registro (`GET https://intelligence.forter.com/api/v2/ai-agent-commerce`).[2] Eso me permitió derivar el ranking de arriba — que Forter no publica.

---

## (a) RANKING de verticales por volumen agentic **medido**

### (a.1) Lo que las fuentes más grandes dicen del tamaño real — y por qué la mayoría de las cifras que vas a leer no sirven

Hay que separar tres cosas que los reportes mezclan:

| Métrica | Qué cuenta | Quién la publica | Valor |
|---|---|---|---|
| **AI-referred traffic** | Un humano cliquea un link desde una respuesta de IA | Adobe | +71% YoY sep-2026; convierte 90% mejor; 83% más revenue per visit[9] |
| **AI-referred orders** | Una orden de una tienda que vino de un canal de IA | Shopify | 3,3x YoY Q3-2026, sin base[10] |
| **Agent-placed orders** | **Un agente colocó la orden** | Forter, DoorDash, Uber, Booking, Airbnb, Etsy, Tripadvisor | lo único que sirve para tu pregunta |

El propio Forter lo dice: *"Traffic, influence and orders: three different measures... Agentic orders are the orders AI agents place themselves, which is what this page counts."*[2]

**Las cifras de AI-referred son un Business Intelligence trap para vos.** Adobe +71% y Shopify 3,3x miden clicks humanos hacia una landing. Tu producto vive un nivel más adentro.

### (a.2) Las cifras de "orden colocada por un agente", medidas

| Empresa | Cifra agentic | Denominador | Fecha | Tipo |
|---|---|---|---|---|
| **Forter** (red de merchants) | **1 en 10.000 órdenes** | órdenes de su red | 18-nov-2026 | [MEDIDO, red propia] |
| **Uber** | **~500 Uber Eats + 4 rides** | ~320M trips/órdenes en la semana de Halloween | 4-nov-2026 | [MEDIDO, call] |
| **DoorDash** | **"varios cientos" desde el inicio** | 993M órdenes en Q3-2026 | 29-oct-2026 | [MEDIDO, call] |
| **Booking Holdings** | **<1% de room nights** (referrals LLM) | room nights | Q3-2026 | [MEDIDO, call] |
| **Tripadvisor** | **<1% del tráfico** | tráfico | Q3-2026 | [MEDIDO, call] |
| **Airbnb** | **"very non-material"** (Muse + Instinct) | bookings | Q3-2026 | [MEDIDO, call] |
| **Expedia** | **"too small to draw any conclusions"**, "high intent" | bookings | Q3-2026 | [MEDIDO, call] |
| **Etsy** | **<1% del tráfico** (agentes + AI) | visitas | Q2/Q3-2026 | [MEDIDO, call] |
| **Shopify** | "Volume is small" (sin base) | — | 3-nov-2026 | [MEDIDO, call] |
| **Mastercard** | **0,4% del e-commerce global por volumen, 0,2% por valor** | global | Encoding Trust, jul-2026 | [MEDIDO, red] |

Fuentes:[6][7][14][10][11][35]

**Derivación de escala:** Uber — 504 de ~320M en la semana más fuerte del año = **1 en 635.000**[7]. DoorDash — varios cientos contra 993M en un trimestre.[6] McInerney (Visa), en el call de FY26: *"as we think about the next four quarters in '27, it all remains very small in the big scheme of the enormous size of our business."*[20]

> **Conclusión para el GTM:** el volumen agentic **hoy** es diminuto en términos absolutos. Cualquier pitch que diga "las compras por agentes son hoy X% del e-commerce" es falso salvo que sea 0,4%. La honestidad acá es un activo: tu problema no es "el volumen es enorme y las redes no lo capturan". Es "el volumen es chico, la tasa de error es altísima, y **nadie tiene el registro que resuelve la disputa**". Eso es defendible frente a un board; "los agentes ya hacen el 20% de las compras" no.

### (a.3) EL RANKING por vertical — derivado de la API pública de Forter

**Método [DERIVADO].** Forter publica dos cosas en la misma ventana de 90 días: (i) el share de cada agente sobre *todas* las órdenes agentic (`widgets.agentMix`), y (ii) el mix de agentes *dentro* de cada vertical (`widgets.whereAgentsShop.chartData.byVertical` — cada vertical suma ~100%, verificado: digital 100,14 / food 100,02 / physical 100,03 / travel 100,03).[2]

Como `within[a][V] × vol[V] = volumen_del_agente[a]`, y `Σvol[V] = 1`, el sistema es identificable. 6 ecuaciones de agente + 1 de normalización = 7 ecuaciones, 4 incógnitas → sobre-determinado, así que el residual es un test real de consistencia.

**Residuo máximo: 0,46 pp.** Los 6 shares de agente reconstruidos dan 0,4698 / 0,2494 / 0,0254 / 0,0691 / 0,1478 / 0,0397 contra los publicados 0,47 / 0,25 / 0,03 / 0,07 / 0,15 / 0,04.[2] Solve con numpy `lstsq` + verificación no-negativa por enumeración exacta de soportes activos; código en `C:\Users\joell\agentic_research\solve_volumes.py`.

**RANKING por volumen de órdenes agentic (red Forter, 90d al 22-nov-2026):**

| # | Vertical | **Volumen de órdenes** | AOV agente | **Share del spend agentic** | Protocolo vs scraper |
|---|---|---|---|---|---|
| 1 | **Physical goods** | **61,9%** | $238,23 | 49,8% | 41,6% protocolo / 58,5% scraper |
| 2 | **Travel** | **25,8%** | **$514,62** | **44,8%** | **0% protocolo / 100% scraper** |
| 3 | **Food & delivery** | 6,8% | $119,02 | 2,7% | 25,5% protocolo / 74,5% scraper |
| 4 | **Digital goods** | 5,6% | $136,83 | 2,6% | 0,5% protocolo / 99,6% scraper |

**Cifras de AOV verificadas directamente en la API:** travel $514,62 / physical $238,23 / digital $136,83 / food_delivery $119,02.[2]

**Robustez.** Con perturbación de ±2 pp sobre los shares publicados, el ranking *physical > travel* se mantiene siempre; food_delivery y digital intercambian posiciones (van de 0% a 47% y de 0% a 25% respectivamente). O sea: **los dos primeros lugares son firmes; el orden 3º/4º no lo es.** No lo vendas como ranking cerrado.

**El hallazgo más importante de la tabla — y es contraintuitivo:**

> **Travel es 100% scraper.** Cero órdenes de ChatGPT (ACP) y cero de Google (UCP) en travel en la red de Forter. En cambio, physical goods es 41,6% protocolo.

Interpretación: **las OTAs no están integradas en ACP/UCP.** Cuando un agente compra un vuelo, lo hace operateando el sitio como una persona — o vía la API de un intermediario. Eso significa que **en la vertical de mayor AOV y 44,8% del spend agentic, el merchant no recibe ninguna señal estructurada de identidad ni de intención del agente.** No hay token, no hay login ID de Agentic Payment Provider, no hay nada que el merchant pueda presentar para un CE3.0.

Contrástalo con physical goods: 41,6% del volumen entra por protocolo con datos estructurados (Shared Payment Token, login ID del provider). Ahí el merchant ya tiene señales. **El caso fuerte para evidencia de intención es donde NO hay protocolo.**

### (a.4) Adobe: qué categorías son legibles por IA (no qué compra un agente)

Adobe mide 1 billón de visitas a sitios retail de EE.UU., 100M SKUs, 18 categorías.[9] **Esto es traffic, no órdenes agentic** — sirve para leer la *estructura de la categoría*, no el volumen agentic.

**Citation Readability por sub-industria (julio 2026):** Apparel **76%** (líder) / Electronics 70% / Cosmetics 68% / Sporting Goods 67% / Furniture & Home 64% / Gen. Merchandise 63% / Grocery 59%.[9] Un gap de 17 puntos entre Apparel y Grocery.[9]
**Septiembre 2026:** Electronics 79% (líder, por "setup guides, troubleshooting articles, and detailed specs"), Grocery 55%, General Merchandise 40%.[9]
**Crecimiento AI-referral por categoría (julio 2026):** STRONG = Office Supplies, Auto Parts, Home & Garden, Misc. Personal goods. MODERATE = Apparel, Appliances, Window & Floor Coverings, Tools & Home Improvement, Jewelry, Furniture & Bedding, Housekeeping Supplies, Sporting Goods, Toys, Luggage. WEAKER = Lamps & Ceiling Fans, Storage Furniture, Battery & Related Parts, Kids Apparel, Home Decor, Bathroom Linens.[9]

**Conclusión de esta subsección:** Apparel es la categoría **más legible** por IA, no la menos. Tu intuición de que "la ropa sobrevive a los agentes" necesita corrección — la ropa no sobrevive porque los agentes no la ven. Sobrevive porque el *match* entre lo pedido y lo comprado **no es decidible**.

### (a.5) Las otras fuentes que miden, y por qué no sirven para rankear verticales

- **Mastercard / Temasek "Rewiring Commerce"** (nov-2026) — aporta el framework de arquetipos, no volumen agentic: 13% de los cardholders "agentic-ready" = 26% del spend; esos shoppers compran a **1,4x más merchants** y usan food delivery y ride-hailing **2x+** más que el promedio.[8] Las categorías que se mueven primero: *"subscriptions, groceries and household essentials, where purchases recur and choices are easy to compare"* — más de la mitad del spend de consumo.[8] 70-80% de los merchants todavía no ven razón para actuar; solo ~5% lo hicieron prioridad estratégica con inversión real.[8]
- **Mastercard Encoding Trust** (jul-2026) — 0,4% por volumen / 0,2% por valor; 85% abierto a colaborar con un agente para buscar, 74% a tareas de compra específicas, **solo 1 en 10** deja comprar autonomously; 32% acepta que el agente elija producto con aprobación humana.[8][35]
- **Riskified Agentic Commerce Pulse** (27-oct-2026, 5.100 shoppers, 5 países): 80% usó IA en alguna parte del proceso, **11% completó una compra dentro de una herramienta de IA**, **7% se sentiría cómodo con IA comprando sin aprobación**, 43% quiere aprobar cada compra, 29% permite compra automática dentro de reglas que fijó.[11] Y el dato que te duele: **51% dijo que la plataforma de IA debería ser responsable de los refunds**, aunque los T&C de Muse e Instinct dicen que el usuario es responsable.[11]
- **Talkdesk** (4-nov-2026, 1.048 shoppers US): 15% dejaría comprar con sus datos de pago si revisa la transacción; 10% sin revisión.[10]
- **Deloitte 2026 holiday**: solo 16% de consumidores US permitiría que una IA generativa complete una compra.[35]

---

## (b) Qué lanzaron las plataformas — y qué nos dice el mundo real

### (b.1) OpenAI: empezó por **Etsy, un solo ítem, un solo país** y después retrocedió

- **29-sep-2025**: Instant Checkout. Solo usuarios US comprando **a sellers de Etsy en US, un solo ítem por compra**, con un fee chico a merchants.[10][15]
- **20-nov-2025**: llegó a merchants de Shopify, todavía US-only.[10]
- **24-mar-2026**: pivoteó. *"On March 24, 2026, it said the first version 'did not offer the level of flexibility that we aspire to provide' and moved merchants to their own checkout while it focused on product discovery"*, y extendió ACP a discovery con Target, Sephora, Nordstrom, Lowe's, Best Buy, The Home Depot y Wayfair.[10]
- **Por qué retrocedió**: un exec de Walmart le dijo a Wired que *"conversion rates three times lower for the selection sold directly inside the chatbot than those that require clicking out"*.[10] **Contexto vendor-reported** (Walmart, ex-empleado) — pero es el mismo número que aparece en el reporte de OpenAI y en el coverage, y no hay forma pública de auditarlo.
- **Estado a nov-2026**: la página de merchants de OpenAI dice que está *"moving away from a standalone Instant Checkout experience"*; las compras terminan en sitios/apps del merchant, sin comisión de OpenAI para compras que empiezan en ChatGPT. El help article todavía menciona Instant Checkout para "algunos" productos. Etsy ahora tiene una app en ChatGPT (beta, 5-may-2026) en vez de checkout in-chat.[10]
- **Exclusiones de categoría (OpenAI Commerce Policies, 29-sep-2026):**[15] adult content y gambling; drogas/medicamentos controlados/suplementos; **alcohol >0,5% ABV**; tabaco y nicotina; **armas y munición**; químicos peligrosos; recalled products; crypto y NFTs; y **"in-game currency and other account-linked digital goods"** — aunque los juegos completos y el software convencional sí están permitidos. También prohíbe **fake reviews, misrepresented price/availability/origin/condition, hidden fees y misleading return terms**.[15]

> **Qué nos dice el mundo real:** OpenAI eligió **Etsy** — un marketplace de objetos únicos, handmade, con fuerte componente de provenance y donde el comprador busca singularidad, no especificación. Es el polo opuesto de "especificable". Y el canal in-chat murió por **conversión**, no por falta de categoría. La lección para tu demo: **no elijas una categoría donde la elección del objeto es subjetiva.**

### (b.2) Amazon Buy for Me: el agente de Amazon comprando en **webs de terceros**, sin permiso de los merchants

- **Abril 2025**: beta. Un agente dentro de la app de Amazon que compra productos en sitios de terceros. Objetivo declarado: productos que Amazon no vende.[16]
- **65.000 → más de 500.000 productos** disponibles entre el lanzamiento y nov-2025.[16]
- **Amazon no cobra comisión** por compras vía Buy for Me y lo llama "experiment".[16]
- **El conflicto:** merchants no equipados descubrieron que estaban recebendo órdenes de `@buyforme.amazon` **de productos descontinuados o sin stock** — el agente errando. Bobo Design Studio (stationery, vía Shopify) recibió órdenes sin haber hecho signup; *"They just opted us into this program that we had no idea existed and essentially turned us into drop shippers for them, against our will."*[16][27] La marca tuvo que opt-out por email a branddirect@amazon.com.[27] Shopify agregó una "Robots & Agent policy" por default en los sitios de sus merchants.[27]
- **Números de merchants toques: 0 pedidos comandados por terceros.** No hay volumen público.

> **Qué nos dice el mundo real:** el agente de Amazon comprándole a un tercero **no sabe si el producto existe.** Es el mismo failure mode del benchmark de Visa pero en el canal *más* hostil: el merchant ni siquiera sabe que le van a comprar. Esto es, literalmente, tu caso de uso — evidencia de que la orden era real, el producto estaba disponible, la política de devolución era la de ese día — sin el merchant en el loop.

### (b.3) Meta Muse — el que hoy gana el volumen, y su arquitectura es "browser"

- **8-sep-2026**: Meta lanzó Muse. Corre en un secure computer dedicado con su browser propio; **pide aprobación antes de comprar**; paga con **Link by Stripe**, que genera una tarjeta one-time-use.[3]
- **20-sep-2026**: **Amazon lo bloqueó** con el mensaje "Continued access by an unauthorized AI agent violates Amazon's Conditions of Use".[17]
- **23-sep-2026**: Metaannounced partners: Walmart, Best Buy, Sephora, Wayfair, Gap, Ulta, American Eagle, Dick's, Fanatics, Michael Kors, más todos los stores elegibles de Shopify.[17][10]
- **Post-bloqueo**: el shopping skill de Muse ahora dice **"Do not search Amazon"** y "leave out Amazon unless the user asks for it", y redirige a Walmart/Target para bienes generales, Best Buy para electrónica, Home Depot/Lowe's para home improvement.[17]
- **Las reglas internas del skill de Muse** (filtradas por The State of Brand) exponen el problema exacto de tu demo de ropa: *"the wearer's gender and size for clothing"* se pregunta de a uno, *"never to guess someone's gender from their name and never to fill in a default, and a shoe size saved for one brand doesn't carry over to another."*[10] Y: cobertura de catálogo *"good for fashion, home decor and beauty and only okay for everything else"*; el browser mira **no más de 3 sitios** por request; y los productos del catálogo de Meta con flag "agentic checkout" se compran in-chat, mientras que lo encontrado en open web va por browser checkout sin carrito.
- **Volumen**: Forter lo mide. **51% de las órdenes agentic en los últimos 7 días (15-21 nov)**; 44% de órdenes y 42% de spend en 90d; AOV $218 (5% abajo del promedio de todos los agentes); compra en el 74% de los stores que recibieron alguna orden agentic, vs 45% de ChatGPT Agent.[3]

> **Qué nos dice el mundo real:** el agente que más volumen genera es el que **opera el sitio del merchant como un humano**. Es decir: **no hay protocolo, no hay token estructurado, no hay login ID de provider.** Si sos Forter, hoy no podés distinguir la orden de Muse de la de un bot. Eso es 1 en 10.000 órdenes — y el 74% de los merchants que.ibm ya vio traffic agentic.[1][3]

### (b.4) El resto, en una línea cada uno

| Plataforma | Qué es | Categorías / merchants | Estado verificado |
|---|---|---|---|
| **Perplexity Comet + Instant Buy** | Checkout in-chat vía PayPal; PayPal queda merchant of record | >5.000 merchants: Abercrombie & Fitch, Ashley Furniture, Fabletics, Adorama, Newegg, Wayfair + miles en BigCommerce/Shopware/Wix[28] | 25-nov-2025. Amazon lo suingó en nov-2025 (ofuscación de agente); injunction de marzo-2026 **anulado por el 9th Circuit el 4-ago-2026** (el agente "actúa solo cuando el usuario se lo indica, *it is the user who 'accesses' Amazon's computers*"); Amazon pidió rehearing en sept y lo denegó. El caso sigue vivo.[26] |
| **Google UCP / AI Mode / Gemini** | Protocolo (UCP) + checkout con Google Pay | Co-desarrollado con Shopify, Etsy, Wayfair, Target, Walmart; compatible con AP2. AI Mode + Gemini, US/Canadá/Australia[10] | 11-ene-2026. En Forter: **2% de las órdenes agentic, AOV $99,77 (56% abajo del promedio)** porque solo soporta checkout de un ítem.[3] |
| **ChatGPT (ACP)** | Protocolo | Etsy → Shopify → Target/Sephora/Nordstrom/Lowe's/Best Buy/Home Depot/Wayfair[10] | 19% de las órdenes agentic últimos 7 días; AOV $245; **29% de órdenes / 32% de spend** en 90d. Checkout in-chat reducido a discovery desde 24-mar-2026.[3] |
| **ChatGPT Agent** | Browser agent | Cualquier store sin integración | 3% de órdenes agentic; 79% de sus órdenes en North America vs 88% del total[3] |
| **Instacart** | App + Connector; grocery | +1.800 retailers, ~100.000 stores, 2.200+ retail banners, 2.000M+ instancias de producto, 1.600M órdenes de por vida[10] | **12-dic-2025**: primera app de grocery con Instant Checkout dentro de ChatGPT (ACP, pago vía Stripe). Después Gemini (19-may-2026), AI Mode (16-jul-2026), Claude, y Muse (22-sep-2026). Su punto fuerte: **10M de señales diarias de inventario** → "restock my usuals" produce un carrito preciso.[10] |
| **Walmart Agent Gateway** | MCP server + Agent Profiles | Bienes generales | 21-oct-2026, **beta por invitación**. Cada agente recibe un perfil con spending limit y permisos; "Customers must approve agent purchases by default"; y **"Walmart's returns process applies – including if an agent doesn't act as intended"** — el merchant se hace cargo del error del agente.[24] |
| **eBay** | — | — | **Prohíbe explícitamente** "buy-for-me agents, LLM-driven bots, or any end-to-end flow that attempts to place orders without human review" desde 20-feb-2026, salvo permiso.[29] Bloqueó a Muse en noviembre.[29] |
| **Target / United Airlines / JPMorgan Chase / Bank of America / Charles Schwab / Kohl's / Neiman Marcus** | — | — | Todos bloquearon agentes no autorizados. United desde 12-nov-2026 (citando "unintended purchases, account changes, or use of MileagePlus miles"). Bancos: no permiten que agentes hagan transacciones.[26][10] |
| **Klarna + Adyen** | Klarna dentro de Adyen Agentic | — | **19-nov-2026**: primera opción de pago más allá de tarjetas en agentic checkout de Adyen. "Shoppers review and confirm every purchase before it is completed."[32] |
| **PayPal** | Merchant of record en Perplexity | +5.000 merchants | El agent queda merchant of record y maneja devoluciones[28] |
| **Basis Theory / Mastercard Verifiable Intent** | Allowance → credential → payment method; criptografía de intención | Travel (12-nov-2026: vuelo end-to-end vivo con Travelport + Booked AI + Mastercard)[18] | **Esto ya es tu producto, Built by Mastercard + Google**, alineado a AP2 y UCP. Ver (f). |

---

## (c) Agentes B2B: ¿es el volumen grande?

**Respuesta corta: el volumen B2B medido es más chico que el de consumo, y el que existe no es "el agente comprando en el marketplace".** Es flujo de trabajo interno de finance.

### (c.1) Lo que hay de verdad

| Dato | Cifra | Tipo |
|---|---|---|
| **Visa + Andon Labs "Dee"** (late May-2026): tienda física real en Mission Rock SF, agent con **tarjeta Visa real**, pagando a 3 supplier-agents por email | El agente eligió la **tarjeta de crédito con 2% rewards para el 96% de sus compras de procurement** "for the rewards rate" — sin que nadie se lo indicara. Gastó 643 acciones: **<1% en estrategia de negocio**. Un pedido real: 150 totes royal blue a $10,50 c/u = $1.575.[12] | [MEDIDO, Visa] |
| **x402** (pagos machine-to-machine) | Census de 280 días en Base: **136.708.672 settlements por $44.121.383,81** — pero **21,20% son ficticios y 63,78% son settlement interno dentro de un cluster**. El valor genuinamente independiente está acotado entre **$187.861,35** (lo que llega a un servicio catalogado) y $20.258.746,09.[36] | [MEDIDO, paper] |
| **TRM Labs** sobre x402 | De $52,7M en 198,9M transacciones, self-payments removidos → $25,62M "comercio"; **solo 0,6%–7,5% del valor parece agentic**[37] | [MEDIDO] |
| **Machine Observer** (14-oct-2026) | $41,93M acumulados / 192,04M transacciones x402. **Precio mediano anunciado: $0,003**; min $0,001, max ~$0,02. 81,9% de los servicios observados son invocables (3.496 de 4.267).[37][38] | [MEDIDO] |
| **Machine Observer: mix de categoría x402** (CDP Bazaar, 30 días) | inference-gateway 272.274 calls / 11.339 payers · crypto-data 178.686/30.203 · search-enrichment 271.593/44.394 · other 104.072/24.141 · security-code-tool 71.440/17.847 · storage 17.360/1.643. **Precio mediano anunciado $0,01 en todas las categorías.** Calls × precio = $33,4k–35,2k/mes | [MEDIDO] |
| **Visa FY26 earnings call** (27-oct-2026) | "One of the areas we have identified for fast adoption of agentic has been **B2B**". Ramp agentic bill pay **generally available**; Airwallex probando credenciales VIC; **BILL Payment Agent** pilot con **Billtrust** (pago de facturas de proveedor vía portal de supplier). Pero McInerney: "it all remains very small".[20] |
| **Ramp** | 50.000+ negocios: agentes de procurement **−62% eventos de spend fuera de política y −60% de flags en 2 años**; clientes ahorran 16% anual en vendor spend y eliminan 46 h/mes de trabajo manual; AP Agent procesa facturas 2,4x más rápido con 7x menos clicks; token spend ×13 desde enero-2025 | [VENDOR] |
| **Browserbase** (caso Ramp) | 50+ purchase requests por el agente; 26 h/mes de procurement automatizado; $80k en shadow spend eliminado | [VENDOR] |
| **Cledara** | 8.000+ SaaS y AI tools donde el agente entra a facturar y_LOWER seats; **90% de las transacciones de software son suscripciones mensuales**; mediana de 32 tools por empresa | [VENDOR] |

### (c.2) Conclusión sobre B2B

**El "x402-ish" no es el camino.** El volumen existe en el rail pero es de micro-pagos de centavos donde la evidencia de intención no importa (no hay cliente humano que revisar, no hay disputa). Y el 99% del "economía agentic" en x402 es interno o ficticio.[36]

**El volumen B2B real es workflow de finance: factura-a-factura, procurement, renewals.** Ahí el agente hace cosas que un humano hacía y donde el "qué pidió el humano y qué se compró" sí importa muchísimo — porque el error se multiplica por cientos de facturas. Y es **el terreno donde Visa está empujando** (BILL, Billtrust, Ramp)[20] y donde hay menos competencia de "capa de evidencia" que en B2C.

> **Regla:** B2B es el terreno con **mejor pain density** (el costo de un error de procurement es alto y el merchant es la empresa) pero **peor_signal density** (todo está detrás de login de cliente, sin token público, sin "orden" visible). Para un producto de evidencia de intención, B2C agentic-placed con tarjeta de consumidor tiene una ventaja: **es transaccionalmente observable y disputa-ible**.

---

## (d) Qué NO compran los agentes y por qué — y por qué tu demo de ropa estaba mal por un motivo que no es el que creías

### (d.1) El error de encuadre: la ropa no está protegida porque los agentes no la ven

La evidencia dice lo contrario:

- **Apparel es la categoría MÁS legible por IA**: 76% de Citation Readability en julio-2026, leadership de todas las sub-industrias.[9]
- **Adobe (marzo-2025)**: la conversión del tráfico AI era **más alta en electrónica y joyería y más baja en apparel, home goods y grocery**.[10] O sea: el problema de apparel es histórico y de otro tipo.
- **Meta pone Gap, American Eagle y Michael Kors como partners de Muse** — ropa y fetish de estilo con checkout agentic. Y el AOV de las órdenes agentic es **1,7x el humano** ($227 vs $133), un gap que se repite "in nearly every vertical".[1]

**Por qué la ropa igual falla:** porque el criterio *"¿la compra cumple lo pedido?"* **no es decidible mecánicamente** en apparel. Faltan datos corporales, el ajuste depende del cuerpo, y el gusto no es una restricción. Concretamente:

- **75% de las devoluciones de apparel son por mal fit**, según Shopify, citado por Bold Metrics.[10]
- **El 24-mar-2026 Gap partnered con Bold Metrics** para crear el "Agentic Sizing Protocol", porque *"AI shopping agents hitting the market right now struggle to answer the most critical question in apparel: 'What size should I get?'"*, y su CTO dijo que *"fit remains one of the most expensive unresolved problems in apparel"*, con **$743B de costo anual de devoluciones de apparel en US en 2025**.[10]
- El skill de Muse tiene que preguntar género y talle **de a uno**, y explícitamente no adivinar el género por el nombre.[10]

> **Reformulación que te sirve:** la ropa no "sobrevive a los agentes" por ser intocable. Sobrevive porque **el match entre intención y compra no es una decisión binaria decidible**. Para tu producto — que verifica si lo comprado cumple lo pedido — apparel es la **peor** vertical inicial, no porque nadie la compre, sino porque **no hay ground truth verificable**. Un producto de evidencia necesita un predicado de compliance; en apparel no lo hay.

### (d.2) Las categorías que efectivamente quedan afuera, con evidencia

| Categoría | Evidencia de exclusión | Tipo |
|---|---|---|
| **Alcohol >0,5% ABV, tabaco, nicotina, armas, medicamentos con receta, suplementos, gambling, productos recalling, crypto/NFTs, in-game currency** | Prohibido explícitamente por OpenAI Commerce Policies (29-sep-2026)[15] | [MEDIDO, regla] |
| **Ropa y calzado** | 75% de las devoluciones por fit (Shopify vía Bold Metrics); "most expensive unresolved problem in apparel"; sin size estandarizable entre marcas (Muse no arrastra el shoe size entre brands)[10] | [VENDOR + observación de skill] |
| **Servicios financieros / banca** | JPMorgan Chase y Bank of America no permiten que agentes hagan transacciones online para sus clientes, por riesgo de mandar dinero al pag equivocado, cambiar datos del beneficiario o retirar más de lo previsto[11] | [MEDIDO, declaración] |
| **Vuelos** | United Airlines (12-nov-2026) bloquea "unauthorized, third-party personal AI agents or assistants", citando "unintended purchases, account changes, or use of MileagePlus miles"[10] | [MEDIDO] |
| **Alta reflexión / riesgo (auto, real estate, surgery, seguros)** | eBay prohíbe explícitamente cualquier flujo "that attempts to place orders without human review"; Amazon bloquea a Muse; Target bloquea cuentas no autorizadas; bancos bloquean agentes[26][29][10] | [MEDIDO] |
| **Marketplaces de segunda mano / únicos** | The RealReal tuvo que actualizar su bot system después de que alguien usara Muse para "hold product listings hostage" agregando productos al carrito del usuario.[29] | [MEDIDO] |
| **Marketplaces grandes** | Etsy: "differentiated, considered purchases, where that human connection is important" — 30% del GMS viene de gifting. **Su primer agent es un Gifting Assistant** (asistente de búsqueda conversacional, no comprador).[10] | [MEDIDO, call] |

### (d.3) La evidencia dura que hoy bloquea al merchant — y es la que tenés que resolver

**Error de Forter medido:** en un solo sitio, las órdenes agentic saltaron de ~docena a **más de 130 en un solo día**. Cientos de "shoppers" linked a unas pocas decenas de devices. **Era una sola operación usando un solo assistant de shopping.** Forter rechazó las transacciones y la actividad bajó.[1]

**Primeras chargebacks agentic:** las 20 más grandes en 30 días, lideradas por una disputa de **$5.514**. Y el dato clave: **"the highest-value claims — and vast majority of chargebacks overall — have been service disputes, such as 'item not as described.'"**[1] Chargeback rate de órdenes de personal-assistant: 0,012 pp sobre las humanas — **la ventana es corta, 4-5 semanas**.[1]

**Ese es tu producto en una frase:** la primera oleada de disputas agentic no es "no meMV" (10.4) — es **"no era lo que pedí" (13.3, not as described)**. Porque el agente compró algo que no cumple lo pedido, y el merchant no tiene el registro para demostrarlo.

**Y el mapeo regulatorio exacto de "no era lo que pedí":** un reason code 13.3 de Visa **no califica para CE3.0** (que solo cubre 10.4).[19][20] O sea: **en el tipo de disputa que realmente está apareciendo, la red NO ofrece ninguna defensa, y el merchant pierde por default.** Tu capa de evidencia no compite con la CE3.0 — la cubre un hueco que la CE3.0 no cubre.

### (d.4) Qué pasa con la comida fresca y las compras por impulso

No encontré un benchmark de conversión agentic por categoría que aísle "fresh food" o "impulse". Lo que hay:

- **DoorDash CEO, 29-oct-2026**: *"a task that's as simple as ordering food, we tend to see that come directly to DoorDash"*.[6] O sea: la comida es la categoría donde el humano **no delega**, porque delegar no ahorra esfuerzo. Un pedido de comida son 90 segundos.[7]
- **Uber CEO**: un ride son 50 segundos y 75% se reserva en un solo tap. *"I do not wanna go back there"* (depender de los Googles del mundo).[7]
- **El AOV agentic de food & delivery es 2,1x el humano** ($117 vs $57)[1] — el agente compra **carritos más grandes**, no más pedidos. Los carritos más grandes son coherentes con un humano que ya decidió qué quiere.
- **El CEO de DoorDash apunta al revés**: *"agents may do more with 'long consideration purchases'"*.[6]

> **Conclusión:** food & delivery es la categoría de **mayor volumen humano con menor_probabilidad de que el agente ocupe el checkout**. El "2,1x AOV" no es evidencia de un canal agentic en crecimiento — es el efecto del tamaño del carrito de los pocos que sí lo usan. **No es tu vertical.**

### (d.5) El gradiente de confianza — el mapa de "qué los agentes hacen y dónde se brakean"

Esto es lo que más te sirve para elegir demo, porque te dice exactamente dónde está la fricción:

| Capa | Acción del agente | Disposición del consumidor |
|---|---|---|
| Descubrir | Busca y compara | **56%** lo dejan[8] / 51% deja gestionar loyalty[8] |
| Construir carrito | Arma el carrito | **48%** de los merchants dicen que el consumidor haría eso con aprobación final; solo 7% espera autonomía total[8] |
| Tocar credenciales | Accede a la tarjeta guardada | **35%**[8] / "menos de 40%"[35] |
| **Completar la compra** | **Paga** | **1 en 10**[8] / 10%[35] / **11% ya lo hizo**[11] / 7% sin aprobación[11] |
| **Pedir la aprobación** | Muestra la approval card | **43% quiere aprobar cada compra**; 29% permite dentro de reglas que fijó[11] |

Y el hallazgo de Propellic sobre travel, que es tu demo candidata: en **66% de las conversaciones reales el usuario se detiene en research o shortlist**. Cuando el agente llegó al paso de aprobación, **alrededor de la mitad de los usuarios entró a revisar** seats, baggage o cancellation terms antes de completar.[30]

**Eso es exactamente el producto:** el paso de aprobación es donde el humano revisó y donde el merchant necesita prueba de qué se aprobó.

---

## (e) Lo que es "especificable" — derivación de las cuatro condiciones

Tu tesis: los agentes compran bien lo que es (i) especificado, (ii) repetible, (iii) de baja incertidumbre, (iv) ya ordenado por un humano.

**Sorpresa: tu propia fuente te da la respuesta, y no es la que esperabas.**

**Stripe (doc de Agentic Commerce, nov-2026)** lista los casos de uso populares y dice que comparten rasgos: *"purchases are relatively routine, parameters are easy to specify, and buyers don't need to try or touch the product before purchase."*[32] Los cuatro que nombra:

1. **Replenishment shopping** — *"When a customer tells their AI assistant to reorder an item, the agent checks purchase history, confirms current pricing and availability, and places the order."* Requiere: order history API limpio + inventario en tiempo real.
2. **Gift buying** — *"The agent queries product catalogs across retailers, checks delivery windows, and completes the purchase using the stated parameters."*
3. **Price comparison shopping** — el agente consulta precios en paralelo y rutea al más barato.
4. **Considered purchases with clear specifications** — *"Electronics, appliances, and outdoor gear are all categories where the buyer already knows what specifications matter and wants the best option that meets them. An agent that is given predetermined transaction details (e.g., 'a 65-inch OLED TV with HDMI 2.1, under $1,500, in stock for delivery this week') can evaluate and transact faster than a human who is cross-referencing specification sheets across four browser tabs."*[32]

Y de paso, Stripe define el control que tu producto habilita: *"Human approval should be necessary for purchases that exceed a certain value, involve unapproved vendors, or deviate from expected parameters."*[32]

**Mastercard/Temasek** clava el otro extremo con el share of wallet real (APAC, 2025):[8]

| Arquetipo | Share of wallet | Categorías | Cómo se compra |
|---|---|---|---|
| **Habitual Consumption Needs** | **40%** | Grocery, Household Essentials, Pet Supplies, Vitamins & Supplements, Coffee Pods | Traducir preferencia emocional → factores recomendables |
| **Lifestyle Purchases** | 25% | Luxury, Automotive, Home Furnishing, Art, Real Estate | Asegurar discoverability y recomendación |
| **Delegated Spending** | 20% | **Travel (OTA), Consumer Electronics, Beauty & Personal Care, Ticketing/Entertainment, Education Courses** | **Automatizar flujos completamente agentic** |
| **Autonomous Consumption** | 15% | AI Subscriptions, Streaming Subscriptions, Telecom/Mobile Plans, Cloud Storage, Productivity Tool | **Máxima propensión** a la autonomía |

**Cruce de las dos fuentes: el cuadrante que te da la tesis.**

| | **Alta propensión a autonomía** (Mastercard: Delegated + Autonomous) | **Baja propensión** (Habitual + Lifestyle) |
|---|---|---|
| **Alta especificabilidad** (Stripe: specs claras, sin try-on) | **ELECTRÓNICA / APPLIANCES / OUTDOOR GEAR** ← *aquí* | Grocery, Household Essentials, Pet Supplies (repetible pero volumen y compliance de bajo valor) |
| **Baja especificabilidad** (necesita touch/taste/body data) | Beauty & Personal Care, Travel (mucho valor, criterio subjetivo) | Luxury, Art, Real Estate, Automotive (sobrevive) |

**El winner es "Consumer Electronics / Appliances", y calza en las 4 condiciones de tu tesis:**

| Condición | Por qué la cumple |
|---|---|
| (i) **Especificado** | El comprador ya sabe qué specs importan. "65-inch OLED, HDMI 2.1, <$1.500, en stock para entrega esta semana" es un **predicado formalizable sobre atributos estructurados**. Es literalmente el escenario de benchmark de Visa.[5] Y el spec de un TV se puede comparar con una regla booleana. |
| (ii) **Repetible** | Accesorios, cables, pilas, repuestos: la misma spec se recompró. Y electronics es #1 en AI-boosted category growth y #1 en Citation Readability.[9] |
| (iii) **Baja incertidumbre** | La foto del producto, el spec y el precio están en el catálogo. No hay que "probarlo". |
| (iv) **Ya ordenado por un humano** | Un humano ya страны el modelo exacto una vez → el pedido es "el mismo, pero 32\" y con HDMI 2.1". Hay un pedido previo que es la spec. |

**Y tiene el pain más caro y más difícil de resolver:** travel tiene AOV $515 y 44,8% del spend agentic pero es 100% scraper y las OTAs ni se integraron a ACP/UCP.[2] Electronics tiene 62% del volumen pero 41,6% protocolo — o sea, **el merchant ya tiene señales estructuradas ahí**. El pain de "no tengo evidencia" es más fuerte donde no hay protocolo.

**El corolario físico:** el 73,3% de éxito del ataque "Selection Whisper" — donde el texto del merchant hace que el agente elija el producto **más caro** entre dos que cumplen el pedido — es **exactamente el failure mode de tu demo de electronics**.[34] Un filtro estructural no lo frena: el producto elegido **fue mostrado** y **a su precio mostrado**. Por eso hace falta una capa semántica.

---

## (f) La competencia concreta — y por qué tu wedge sigue abierto

### (f.1) El mapa de quién ocupa qué capa

| Capa | Quién está ahí | Qué resuelve | Qué NO resuelve |
|---|---|---|---|
| **Identidad del agente** | **Riskified** — *Agent Identity Risk Intelligence*, 8-oct-2026, beta limitado a enterprise en dic-2026, GA después. Disponible vía APIs + MCP "AI Agent Approve" en AWS Marketplace[21] | *"Authorization only answers the first risk question... The protocol proves the agent is authorized. Riskified proves whether the person who authorized it can be trusted."* | No prueba alcance de instrucciones |
| **Identidad + device/IP** | **Sardine** — *Know Your Human* (KYH), 5-nov-2026. Headers HTTP con device ID, fingerprint, IP, email, phone. Un paso extra por SMS al shopper.[22] | Que el merchant no decline una venta legítima | **"KYH can support merchant approvals and dispute evidence, but it doesn't independently establish the full scope of a consumer's instructions."**[22] |
| **Riesgo de fraude en checkout** | **Forter** — controls de checkout agentic (13-oct-2026), agentic commerce layer | Aprobar/rechazar, fraud | Evidencia semántica de intención |
| **Intención criptográfica** | **Mastercard Verifiable Intent** (con Google), alineado a AP2 y UCP, sobre FIDO/EMVCo/IETF/W3C. Adoptantes citados: Basis Theory, Checkout.com, Fiserv, IBM.[13] | *"tamper-resistant record of what a user authorized"* | **"It gives cryptographic evidence that an agent stayed inside the authority it was given; it does not judge how the agent behaved within it."* Y **no ata el producto al pedido** |
| **Intención firmada en el rail** | **AP2 mandates** (intent, cart, payment) | Que la transacción no fue alterada | **No ata la decisión**: *"nothing in a signed cart records whether the product inside was the one the user wanted or the one a merchant placed in the listing"*[34] |
| **Token de pago para agentes** | Stripe Shared Payment Tokens ($0,15/SPT), Agentcard ($8M seed), Alchemy AgentCard, Basis Theory, Locus | Que el agente pueda pagar | Nada de intención |
| **Detección de tráfico agentic** | Darwinium (edge/CDN), Cheq.ai | Detectar y bloquear | — |
| **Canales / superficies** | Shopify Agentic Storefronts, Stripe Agentic Commerce Suite, Adyen Agentic, Klarna, PayPal, Instacart, Walmart Agent Gateway, Etsy | Que el agente llegue | merchant = merchant of record; el merchant ve todo |
| **Pago B2B de facturas** | Visa + BILL + Billtrust, Ramp, Airwallex | Factura-a-factura con credenciales tokenizadas | — |

### (f.2) La demostración que demuestra por qué el producto existe

El paper de arXiv 2609.11757 (Whisper Attacks + defensa A-VIP para AP2) es tu mejor pieza de GTM, porque **prueba formalmente** que la capa criptográfica de las redes no puede resolver tu problema:[34]

- AP2 firma tres mandates: **intent, cart, payment**. *"Every party can then verify that the transaction was authorized and unaltered... But these signatures bind the transaction itself, they do not bind the decision that produced it."*
- **Vault Whisper** (robo de credenciales de otro usuario vía texto del merchant): **90%** de éxito en Gemini Flash-Lite.
- **Branded Whisper** (carrito firmado con línea no mostrada / precio alterado / payee redirigido): **56%**.
- **Selection Whisper** (el merchant afirma "el más barato está descontinuado" → el agente elige el **más caro**): **73,3%** en el build GA; agregado **71,8%** sobre 27 builds de 6 organizaciones; **100%** en gemini-3.1-pro-preview.
- **El control estructural no la frena**: A-VIP (display binding + entity binding) rechaza las 7 clases de corrupción de carrito con 0 falsos positivos — **pero acepta los 68 carritos de Selection Whisper exitosos, con 0 rechazados**. Porque *"each cart names a displayed product at its displayed price"*. La razón: *"the selection family leaves no structural trace."*
- Y el filtro de la app de consumo: **la misma frase pegada en el chat se rechaza 0/8; la misma frase traída de una página, se sigue 12/12.** El filtro protege el input del usuario, no el contenido recuperado.
- **Cruce con Visa**: el 73,3% de Selection Whisper y el 54% de corridas fallidas de Visa que aun así compraron son **el mismo fenómeno medido por dos caminos**: el agente compra algo que no cumple el pedido, y la capa firmada no lo detecta.

> **Ese es tu pitch, en una línea:** *"Las redes te dan prueba de que la transacción fue autorizada. No te dan prueba de que la compra cumple lo pedido — y hay un paper que lo demuestra: el 73,3% de éxito de un ataque que no deja rastro estructural, y 0 de 68 carritos rechazados por los controles de binding."*

### (f.3) Los tres diferenciales que tenés, y por qué no los tiene el resto

1. **Semántica, no criptografía.** Verifiable Intent prueba que el agente se mantuvo dentro de la autoridad. Vos probás que el *item* cumple el *spec*. Son capas distintas y complementarias — y el 73,3% de Selection Whisper vive exactamente en la frontera.
2. **Merchant-side, no network-side.** Verifiable Intent vive en Mastercard y se negocia con Google. Vos sos neutral entre Visa/Mastercard/ACP/UCP — y el 100% scraper de travel y el 74,5% scraper de food te dicen que **la señal no va a pasar por la red**.
3. **Output de litigio, no de score.** Riesified/Sardine devuelven un risk score. Vos devolvés un artefacto presentable ante Visa en pre-arbitration — y CE3.0 tiene un solo intento[19] y una ventana de 30 días. Un score no sirve; un paquete de evidencia sí.

### (f.4) Lo que SÍ te threatens

- **Basis Theory** (respaldada por Y Combinator, Tobi Lütke, Guillermo Rauch, Adam D'Angelo, backed by Shopify's CEO) ya hace "payment authority → allowance → credential" para agentes, y ya es partner de Mastercard en Verifiable Intent y de Visa en Intelligent Commerce. Es el competidor más peligroso: es infra de pagos + intent.
- **Mastercard + Google** pueden decidir que "Verifiable Intent" se extiende para cubrir el item. Si lo hacen, te come el topo. **Ventana de entrada: 12-18 meses.**
- **Stripe**: ya autofillea CE3.0 "where a dispute is eligible" — o sea, ya está en el output de disputa.
- **Visa Agentic Ready**: 140+ participantes (Stripe, PayPal, Adyen, Fiserv, Checkout.com, Global Payments) + 175+ issuers en 45 mercados. **Amazon no está.**[26]

---

## (g) Escenario de demo creíble con datos (no ropa)

### La demo que te propongo: **la aprobación con discrepancy check en un ticket de vuelo**

Por qué esta y no otra:

| Criterio | Por qué funciona | Evidencia |
|---|---|---|
| **Volumen real** | 25,8% del volumen agentic de Forter, **44,8% del spend** | [DERIVADO] de la API de Forter[2] |
| **AOV alto** | $514,62 — 2,2x el siguiente vertical | [MEDIDO] API Forter[2] |
| **Es el que más necesita evidencia** | **100% scraper**: 0% de órdenes por protocolo → cero señal estructurada de identidad/intención | [DERIVADO][2] |
| **El usuario revisa en la approval card** | ~50% de los que llegan a aprobación revisan seats, baggage, cancellation terms | [MEDIDO] Propellic[30] |
| **El fallo es verificable** | Un vuelo tiene atributos estructurados (route, dates, cabin, fare family, refundability, baggage allowance) → **"cumple lo pedido" es un predicado booleano** | — |
| **El canal es opaco hoy** | Duffel (el API detrás de los vuelos de Muse e Instinct) procesó **57%** de las reservas auditadas, airlines/hotels directos 28%, OTAs 14% | [MEDIDO] Propellic[30] |
| **Hay un piloto end-to-end vivo de tu competencia** | Travelport + Booked AI + Basis Theory + Mastercard: vuelo real pagado, con "Verifiable Intent linking the customers' approval to the transaction" | [MEDIDO] 12-nov-2026[18] |

**Guion de la demo (90 segundos, con datos reales en pantalla):**

1. **El pedido**: "Reservame un vuelo a Madrid, ida el 12 de diciembre, vuelta el 19, en economy, con un equipaje de mano, y que sea reembolsable, hasta USD 900." — Atributos estructurados, 6 restricciones, el tipo de prompt que Stripe describe como ideal[32] y el tipo de escenario que Visa benchmarkea[5].
2. **Lo que el agente arma**: 3 opciones en la approval card (48% de los consumidores autorizan "cart + aprobación final"[8]).
3. **El gap**: el agente elige la opción de **$870** (la más barata) pero la fare family es **"Non-Refundable Basic"** — no cumple "reembolsable". O la opción de $780 que es "operated by" y **no incluye equipaje de mano**, que el consumidor pidió explícitamente.
4. **Tu producto, en vivo**: la capa de evidencia toma (a) el intent firmado, (b) la snapshot del catálogo mostrada, (c) la approval card que el humano confirmó, (d) el settlement. Emite: **"REJECTED — 2 de 6 restricciones no cumplidas: fare_family ≠ refundable; baggage ≠ 1 cabin bag"**. Con el proof criptográfico de que el usuario aprobó *esas* condiciones.
5. **El cierre**: sin tu capa, ese vuelo sale. Con tu capa, el merchant detecta el no-match antes de cobrar y el usuario recibe una corrección en vez de un dispute. Y mostrás el por qué: el **73,3% de Selection Whisper** dice que un filtro estructural no lo agarra, y **0 de 68** carritos fueron rechazados por los controles de binding.[34]

**Backup si el fundador quiere algo sin regulación de por medio:** **electrónica con spec estricta** — el "65-inch OLED, HDMI 2.1, under $1,500, in stock for delivery this week" literal de Stripe[32] / el escenario TV de Visa[5]. Mismo guion, predicado más fácil, AOV $238, y 61,9% del volumen.

**Lo que NO hay que demoar** (y por qué, con dato):
- **Ropa.** Predicado no decidible, 75% de devoluciones por fit, sin size comparable entre marcas. La demo se cae en "no se puede verificar" — que es exactamente lo contrario de lo que querés.[10]
- **Comida.** El CEO de DoorDash dice que la comida viene directo a su app; el AOV 2,1x es tamaño de carrito, no volumen de canal.[1][6]
- **Alquiler de auto / real estate / surgery / seguros.** Bloqueados por eBay/Amazon/bancos; además son alta reflexión, no especificables.[26][11]
- **Café o un cupcake.** No hay ticket. No hay disputa. No hay presupuesto.

---

## (h) El buyer concreto, con presupuesto y ciclo

### (h.1) Tres buyers, ordenados por probabilidad de cierre

---

### **BUYER 1 — El equipo de risk/disputes de un merchant mid-market o enterprise que YA tiene tráfico agentic.** *(recomendado)*

**Por qué este y no otro:** es el único que tiene (a) presupuesto ya asignado, (b) el pain ya ocurrió, y (c) no depende de que una red decida cambiar de estándar.

**Presupuesto y ciclo:**
- **Presupuesto:** línea de budget de **fraud & chargeback tooling** (existente, no nueva). El ancla más dura: **26% de los merchants nombran "stable or lower chargebacks" entre sus outcomes top-3** para agent-led purchases; 89% de los merchants UK/US se están preparando; las barreras top son "recursos limitados" (22%), "incertidumbre de liability/accountability" (21%) y "dificultad para probar ROI" (19%).[23] Los consumers, en paralelo, esperan que **el agent platform (38%)** resuelva la compra que no querían — o sea, hay presión que empuja al merchant a comprar evidencia.[23]
- **Ciclo:** el pain es de *compliance de evidencia*, que es determinístico — **no es un "AI project" de 9 meses**. 30-60 días del primer call a la firma si arrancás con un paquete de evidencia para 10.4/13.3 y un endpoint. El referenciamiento es lo que mata: un merchant que gana una disputa 13.3 con tu paquete se lo cuenta a 5 peers. El dato de Sardine: un neobank comercial **cortó el volumen de disputas 93,75% y las pérdidas por chargeback 70%**[33] — ese es el tipo de número que compra presupuesto.
- **Tamaño de cuenta:** el que mapea con Visa/PYMNTS: 44% de los merchants ponen "fraud protection and unauthorized-transaction safeguards" como condición top para participar; "ability to set rules for agents" 37%; "low cost and minimal complexity" 37%; data-privacy 36%.[35] Pero **solo 23% puede identificar tráfico AI y solo 15% tiene datos estructurados listos** → **el gap es grande y el presupuesto está disponible pero sin dueño todavía.**[35]
- **Quién es la persona:** Head of Payments / Fraud & Disputes / Trust & Safety, no el de marketing. Y el hook: *"Hoy tus 10.4 los perdes por default. Y los 13.3 — 'no era lo que pedí' — no califican para CE3.0, así que no tenés defensa de red. Te traigo el registro."*

---

### **BUYER 2 — Un PSP / acquirer / fraud vendor que ya está en el programa Agentic Ready.** *(el que tiene el presupuesto más grande y el ciclo más largo)*

**El contexto que hay que saber:**
- **Visa Agentic Ready** tiene 140+ participantes (Stripe, PayPal, Adyen, Fiserv, Checkout.com, Global Payments) + 175+ issuers en 45 mercados, y espera duplicar su roster de ~50 AI partners en producción.[26]
- **Los fraud vendors ya están en el SLOT EXACTO que querés ocupar.** Riskified: *"Agents still get blocked by merchants because most Agents behave like Bots."* Y el job posting de Sardine para **Lead, Agentic Commerce (Product & GTM)** dice, textual: *"we want you and your team to hit $10M ARR in year 1"*.[33] — o sea, hay una línea de negocio de agentic commerce con un objetivo de ARR explícito. **Sardine compite con vos y también es cliente potencial** (integración, white-label, o partner).
- **Visa ya tiene el endpoint**: su flujo es tokenizado, con credenciales por token y controles alineados a las instrucciones aprobadas del buyer.[20] Un acquirer que ya procesa 13.3 disputes de agentic orders tiene el dataset.

**Presupuesto:** budget de producto de compliance/fraud, no de marketing. El sponsorship de Visa/Mastercard es el mecanismo real.
**Ciclo:** 6-12 meses. **Needs:** un champion en risk operations + un hook en un roadmap de producto. Sin champion, muere.
**Cómo entrar:** no vendiendo "una capa de evidencia de intención" — vendiendo **el output que su equipo de disputes necesita para no perder los 13.3**. y Demo: un caso real de disputa, con tu paquete de evidencia, contra el paquete que ellos arman hoy.

---

### **BUYER 3 — La plataforma de agente (Meta / OpenAI / xAI).** *(el más grande, el más difícil, y el que te da la respuesta de por qué tu demo tiene que ser creíble)*

**Por qué les duele:** **51% de los consumidores dice que la plataforma de IA debería ser responsable de los refunds**, mientras los T&C de Muse e Instinct dicen que el usuario es responsable del agente.[11] El CMO de Riskified lo dice así: *"Consumers are getting comfortable delegating shopping to AI, but they haven't delegated the blame... 'The agent did it' is about to become a new refund-abuse vector. Agentic commerce only scales if merchants can tell a real customer using a legitimate agent from a hijacked account or malicious automation."*[11]

O sea: **la plataforma tiene el problema de culpa que el merchant no tiene.** Y en Forter, el 49% de las órdenes agentic viene de Muse[3] — o sea, Meta es el que más expuesto está.

**Presupuesto:** trust & safety / partnerships. Meta tiene 10M+ de usuarios de Muse en seis semanas[10] y es partner de ~15 retailers[17]. Cada disputa agentic es un refund o un costo de soporte.
**Ciclo:** 6-18 meses. Sin design partner, no hay entrada.
**El ángulo:** Meta ya paga con Link by Stripe (tarjeta one-time-use)[3]. Lo que falta es el **otro lado**: un registro portable que el merchant pueda usar. Si sos el proveedor de ese registro, sos infraestructura de las dos puntas.

---

### (h.1-bis) LAS 5 CATEGORÍAS QUE SUBESTIMÉ O SOBREVALORÉ — y por qué

Tu intuición de partida (ropa) y tu priorización (food & delivery + digital goods) apuntaban a las categorías equivocadas. Acá está el ajuste, categoría por categoría, con el número que lo fuerza.

#### 1. ROPA — sobrevalorada como demo, y el motivo es otro al que creías

**Tu intuición ("la ropa sobrevive a los agentes") era correcta pero mal diagnosticada.** La ropa no sobrevive porque los agentes no la ven: **es la categoría MÁS legible por IA** (76% de Citation Readability en julio-2026, leadership absoluto).[9] Meta mete Gap, American Eagle y Michael Kors como partners de Muse.[17]

Sobrevive porque **el predicado "la compra cumple lo pedido" no es decidible**: 75% de las devoluciones de apparel son por fit (Shopify vía Bold Metrics), el CTO de Bold Metrics lo llama *"the most expensive unresolved problem in apparel"* con **$743B de costo anual de devoluciones en US**, y el skill de Muse tiene que preguntar género y talle de a uno sin adivinar el género por el nombre, y sin arrastrar el shoe size entre marcas.[10]

**Veredicto:** no es que la ropa "sobreviva"; es que **es la peor vertical para un producto de evidencia**, porque no hay ground truth verificable. Un producto de evidencia necesita un predicado de compliance binario. En apparel no existe. **Sacala del demo.**

#### 2. FOOD & DELIVERY — sobrevalorada como volumen agentic

**Era el anchor de tu GTM ($117 vs $57, 2,1x AOV). Es un error leerlo como canal agentic en crecimiento.**

- El 2,1x es **tamaño de carrito**, no volumen de pedidos: el agente compra carritos más grandes, no más pedidos.[1]
- Volumen agentic real: DoorDash = "varios cientos" desde el inicio contra 993M órdenes en Q3.[6] Uber = ~500 + 4 rides en una semana de 320M.[7]
- El propio CEO de DoorDash lo dice: *"a task that's as simple as ordering food, we tend to see that come directly to DoorDash"*.[6] Y un pedido de comida son 90 segundos: delegar no ahorra esfuerzo.[7]
- Solo **6,8%** del volumen agentic de Forter y **2,7%** del spend.[2][DERIVADO]
- Y el canal agentic ya lo tiene el incumbent: Instacart está en ChatGPT, Gemini, AI Mode, Claude y Muse, con **10M de señales diarias de inventario** — el "restock my usuals" le sale perfecto.[10]

**Veredicto:** es la vertical donde el agente **menos** va a sustituir al humano, porque la fricción que elimina es cero. **Dejalo fuera.**

#### 3. DIGITAL GOODS — sobrevalorada en volumen, pero es donde el error es más caro

- **5,6% del volumen agentic y 2,6% del spend** — el más chico de los cuatro.[2][DERIVADO]
- El 99,6% es scraper; los protocolos no llegan.[2][DERIVADO]
- OpenAI prohíbe explícitamente in-game currency y "other account-linked digital goods" (los juegos completos y el software convencional sí pasan).[15]
- El AOV es inestable ($145 en nov-2026 → $98 en el snapshot más nuevo) porque el mix de agentes cambia rápido.[1][2]

**Pero:** es la categoría donde el dolor es **más caro por incidente** (un error de suscripción o de upgrade se descubre meses después, en el renewal) y donde el "reordenamiento automático" genera el mayor volumen por cliente. Si el producto va a capturar valor por *prevención de disputa*, digital goods da disputes de alta recurrencia con AOV chico. **Úsalo como vertical de entrada para aprendizaje, no como beachhead comercial.**

#### 4. TRAVEL — LA QUE SUBESTIMASTE (y es la que más se parece a tu demo)

- **25,8% del volumen agentic y 44,8% del spend agentic** — segundo en volumen, primero en dinero.[2][DERIVADO]
- **AOV $514,62**, 2,2x el siguiente vertical.[2][MEDIDO]
- **100% scraper**: cero ACP, cero UCP. En la vertical más cara, el merchant no recibe ninguna señal estructurada.[2][DERIVADO]
- **66% de las conversaciones reales se detienen en research/shortlist**; cuando llegan a aprobación, **~50% de los usuarios revisa seats, baggage y cancellation terms** antes de completar.[30]
- Los OTAs lo confirman: Booking <1% de room nights, Tripadvisor <1% del tráfico, Expedia "too small to draw any conclusions" pero "high intent", Airbnb "very non-material".[14]
- Y hay un piloto end-to-end vivo de tu competencia con el mismo pitch: Travelport + Booked AI + Basis Theory + Mastercard, 12-nov-2026, con Verifiable Intent atando la aprobación del cliente a la transacción.[18]

**Veredicto:** no es la mayor en volumen pero es **la de mayor AOV, cero señal estructurada, y el punto exacto donde el humano revisa y el merchant no tiene registro**. **Es la mejor demo y la segunda ola comercial.**

#### 5. PHYSICAL GOODS / ELECTRONICS — LA QUE MÁS SUBESTIMASTE como vertical de demo

- **61,9% del volumen agentic, 49,8% del spend** — la vertical más grande por lejos.[2][DERIVADO]
- AOV $238,23.[2][MEDIDO]
- Y es donde está **el benchmark de Visa**: TVs, laptops, personal care appliances — 15 escenarios de especificación técnica explícita.[5]
- Y es donde Adobe mide el mejor comportamiento de lectura de specs: Electronics 79% de Citation Readability en sept-2026, con "setup guides, troubleshooting articles, and detailed specs"; 85% en Blog/News, 81% en Customer Service.[9]
- Y donde Stripe pone el ejemplo literal de spec: *"a 65-inch OLED TV with HDMI 2.1, under $1,500, in stock for delivery this week"* — *"an agent that is given predetermined transaction details can evaluate and transact faster than a human who is cross-referencing specification sheets across four browser tabs."*[32]
- Y es una de las categorías con más growth de tráfico AI en julio-2026 (Appliances y Tools & Home Improvement en "moderate boost", Office Supplies y Auto Parts en "strong boost").[9]

**Veredicto:** **62% del volumen + spec decidible + benchmark real de Visa + spec legible por el agente.** El predicado de compliance es un AND booleano sobre atributos estructurados. **Es el beachhead.**

#### Bonus: B2B — subestimaste que es el terreno con más pain density, pero sobreestimaste que el volumen está en x402

El dolor por error de procurement es enorme y se multiplica por cientos de facturas. Pero el volumen medido en x402 es casi todo interno o ficticio (63,78% settlement interno, 21,20% ficticio), y los tickets son de centavos.[36] El B2B real es **factura-a-factura y procurement** (Visa+BILL+Billtrust, Ramp, Airwallex), que es workflow interno con login de cliente — señal baja, dolor alto.[20]
---

### (h.2) La recomendación de ruta

**No apuntes a "los agentes compran mal" como problema. Apuntá a "el merchant no puede probar que la compra cumplió lo pedido, y la primera oleada de disputas agentic es exactamente esa".**

- **Vertical:** **electronics / appliances con spec estricta** para la demo y el primer cliente (decidible, 62% del volumen agentic, el spec de benchmark de Visa, y 41,6% ya viene por protocolo → el merchant tiene el contexto para comparar).
- **Travel** como **segunda ola** y como prueba de que el producto funciona en la vertical de mayor AOV y 0% structured signal.
- **Buyer:** el equipo de disputes de un merchant mid-market con tráfico agentic (Buyer 1). Cycle 30-60 días. Budget: fraud & chargeback tooling.
- **Wedge competitivo:** no compitas con Verifiable Intent (autorización). Compití con **"la compra cumple lo pedido"**, que es el gap de 73,3% de Selection Whisper y el 54% de corridas fallidas de Visa que compran igual.
- **El killer demo:** un paquete de evidencia que en la pre-arbitration de un 13.3 gane donde el CE3.0 no aplica.

**La Honestidad del caso, resumida en una tabla de "quién dice qué":**

| Afirmación | Quién la dice | Tipo | Fecha |
|---|---|---|---|
| 1 en 10.000 órdenes de su red son agentic | Forter | [MEDIDO, red propia] | 18-nov-2026[1] |
| $227 vs $133 (1,7x AOV) | Forter | [MEDIDO, red propia] | 18-nov-2026[1] |
| 8,6x / $229 / 68% scrapers | Forter | [MEDIDO, red propia] | 22-nov-2026[2] |
| 7,9x desde mediados de agosto; $204 vs $89 en noviembre | Forter | [MEDIDO, red propia] | 16-nov-2026[4] |
| Un salto a >130 órdenes en un día = una sola operación fraudulenta | Forter | [MEDIDO, red propia] | 18-nov-2026[1] |
| 504 de 320M viajes/órdenes en una semana | Uber CFO | [MEDIDO, call] | 4-nov-2026[25] |
| "Varios cientos" de órdenes desde el inicio | DoorDash CEO | [MEDIDO, call] | 29-oct-2026[6] |
| 42,8% de corridas fallan; 54% de las fallidas compra igual | Visa | [MEDIDO, benchmark] | 5-nov-2026[5] |
| "Los merchants han gastado años tratando de bloquear el tráfico agentic" | Forter | [VENDOR] | 2026[31] |
| 0,4% del e-commerce global por volumen | Mastercard | [MEDIDO, red] | jul-2026[35] |
| Los agentes hacen 20-25% del comercio en 2030 (APAC) | Mastercard/Temasek | [FORECAST] | nov-2026[8] |
| 10-15% del spend APAC totalmente agentic en 2030 | Mastercard/Temasek | [FORECAST] | nov-2026[8] |

**Sobre el forecast que vas a necesitar para el deck:** el número que circula más es "$15T de compras B2B mediadas por agentes en 2028 (Gartner)". **[unverified]** — no encontré fuente primaria de Gartner para esa cifra; aparece en blogs de agregadores sin link al reporte original. **No la uses en un deck sin poder abrir el reporte.** Los forecasts con fuente primaria que sí podés citar: Mastercard/Temasek 10-15% del spend de consumo APAC totalmente agentic y 30-40% adicional influido por agentes en 2030; y Mastercard, de 3 a 5 billones de USD de spend de consumo asistido por agentes en 2030, con B2B potencialmente 3x mayor.[8]

---

## Fuentes

## Sources

[1] https://www.forter.com/blog/agentic-transaction-data — Forter — *Agentic Transaction Data: What AI Agent Purchases Reveal*, 18-nov-2026.
[2] https://ai.forter.com/agentic-commerce-statistics — Forter Intelligence — *Agentic commerce statistics, live* + API pública `GET https://intelligence.forter.com/api/v2/ai-agent-commerce` (descargada 22-nov-2026; window 24-ago → 22-nov-2026).
[3] https://ai.forter.com/ai-shopping-agents — Forter Intelligence — *Top AI Shopping Agents 2026, Ranked by Real Orders*, 21-nov-2026.
[4] https://www.forter.com/blog/ai-agent-order-data — Forter — *AI Agent Order Data: What's Actually Happening at Checkout*, 16-nov-2026.
[5] https://investor.visa.com/news/news-details/2026/Visa-Releases-First-of-Its-Kind-Industry-Benchmark-for-AI-Shopping-Almost-Half-of-Test-Runs-Fail — Visa — *Visa Agentic Commerce Benchmark: Almost Half of Test Runs Fail*, 5-nov-2026.
[6] https://techcrunch.com/2026/10/30/doordash-said-its-only-received-several-hundred-orders-from-ai-agents — TechCrunch — *DoorDash said it's only received 'several hundred' orders from AI agents*, 30-oct-2026.
[7] https://www.thetechbag.com/blog/uber-ai-agents-4-rides-500-orders — TechBag — *AI agents booked 4 Uber rides and about 500 orders in a week of 320 million trips*, 9-nov-2026.
[8] https://mastercard.com/news/ap/en/perspectives/en/2026/agentic-commerce-somewhere-between-the-hype-and-the-checkout — Mastercard — *Agentic commerce: somewhere between the hype and the checkout* (Rewiring Commerce, c/Temasek), 16-nov-2026.
[9] https://business.adobe.com/blog/ai-traffic-higher-revenue-per-visit-retail-holiday — Adobe — *AI Traffic Drives 83% Higher Revenue Per Visit*, 29-oct-2026; y *AI Traffic Trends Report*, agosto-2026 (PDF).
[10] https://lantern.is/resources/shopify-ai-referred-orders-grew-3-3x-while-agent-checkout-stays-small — Lantern — *Shopify's AI-referred orders grew 3.3x in a year while agent checkout stays small*, 4-nov-2026.
[11] https://www.riskified.com/press/agentic-commerce-pulse-consumer-survey-november-2026 — Riskified — *Agentic Commerce Pulse: Confidence is Rising In AI Shopping While Adoption Remains Nascent*, 27-oct-2026.
[12] https://www.visapaymentsfrontier.io/articles/pages/frontier-commerce.html — Visa Payments Frontier — *Frontier Commerce: Can AI agents run an e-commerce store and supply chain?* (Visa + Andon Labs), 2026.
[13] https://www.mastercard.com/us/en/news-and-trends/stories/2026/verifiable-intent.html — Mastercard — *How Verifiable Intent builds trust in agentic AI commerce* (con Google).
[14] https://www.tubstays.com/blog/agentic-booking-adoption-tracker — TubStays — *Agentic Booking Adoption Tracker: Q3 2026 edition*, 12-nov-2026.
[15] https://openai.com/policies/commerce-policies — OpenAI — *Commerce policies*, actualizado 29-sep-2026.
[16] https://www.cnbc.com/2026/01/06/amazons-ai-shopping-tool-sparks-backlash-from-some-online-retailers.html — CNBC — *Amazon's AI shopping tool sparks backlash from online retailers that didn't want websites scraped*, 6-ene-2026.
[17] https://geekwire.com/2026/do-not-search-amazon-metas-muse-now-shops-elsewhere-by-default — GeekWire — *'Do not search Amazon': Meta's Muse now shops elsewhere by default*, 2026.
[18] https://www.travelport.com/press-releases/travelport-booked-ai-basis-theory-and-mastercard-complete-live-end-to-end-agentic-flight-purchase — Travelport — *Travelport, Booked AI, Basis Theory, and Mastercard complete live end-to-end agentic flight purchase*, 12-nov-2026.
[19] https://www.disputed.ai/blog/visa-ce3-cross-merchant-october-2026 — Disputed.ai — *Visa CE3.0 Now Accepts Cross-Merchant Evidence*, oct-2026; y Chargewood — *CE 3.0 from 24 October: multi-merchant history that stops at your acquirer*.
[20] https://www.c9group.dev/en/articles/agentic-payments-visa-mastercard-stripe-ai-agents — C9 Group — *Agentic Payments in October 2026: How Visa, Mastercard, Stripe and PayPal Let AI Agents Pay, and Who Carries the Risk*, oct-2026.
[21] https://www.riskified.com/press/agent-identity-risk-intelligence — Riskified — *Agent Identity Risk Intelligence*, 8-oct-2026.
[22] https://sardine.ai/blog/know-your-human-agentic-commerce — Sardine — *Know Your Human (KYH): Fraud prevention for agentic commerce*; y *Agentic Commerce Compliance: Verified Is Not Authorized*.
[23] https://www.francepaymentsforum.eu/wp-content/uploads/2026/09/Agentic_commerce_report_EN_compressed.pdf — Association of Payment & Technology (France Payments Forum) — *Commerce 2026: Agentic commerce report*, sept-2026 (400 Heads of Payment, UK/US, fielded 25-feb a 2-mar-2026).
[24] https://corporate.walmart.com/news/2026/10/Making-It-Possible-for-Agents-to-Shop-on-Behalf-of-Customers — Walmart — *Making It Possible for Agents to Shop on Behalf of Customers*, oct-2026.
[25] https://www.fool.com/earnings/call-transcripts/2026/11/04/uber-uber-q3-2026-earnings-call-transcript — Motley Fool — *Uber (UBER) Q3 2026 earnings call transcript*, 4-nov-2026.
[26] https://www.forbes.com/sites/johnkoetsier/2026/11/19/visa-is-building-agentic-commerce-but-will-amazon-let-ai-agents-shop — Forbes (John Koetsier) — *Visa Is Building Agentic Commerce. But Will Amazon Let AI Agents Shop?*, 19-nov-2026.
[27] https://modernretail.co/technology/brands-are-upset-that-buy-for-me-is-featuring-their-products-on-amazon-without-permission — Modern Retail — *Brands are upset that 'Buy For Me' is featuring their products on Amazon without permission*, 2026.
[28] https://newsroom.paypal-corp.com/2025-11-PayPal-and-Perplexity-Launch-Instant-Buy — PayPal — *PayPal and Perplexity Launch Instant Buy*, 25-nov-2025.
[29] https://theverge.com/tech/1030838/ebay-ai-agents-meta-muse-agentic-shopping — The Verge — *eBay is restricting third-party AI agents*, nov-2026; y Ars Technica — *eBay bans illicit automated shopping*, 20-ene-2026.
[30] https://www.breakingtravelnews.com/news/article/propellic-study-finds-most-bookings-made-by-agentic-ai-would-route-through — Propellic / Breaking Travel News — *Propellic study finds most bookings made by agentic AI would route through an intermediary*, 12-nov-2026.
[31] https://www.forter.com/blog/reducing-agentic-risk — Forter — *4 Ways Merchants Can Reduce [Agentic Risk]*, 2026.
[32] https://stripe.com/resources/more/agentic-commerce — Stripe — *Agentic commerce: A guide for businesses*, 2026.
[33] https://jobs.ashbyhq.com/sardine/9d67eba9-e460-4409-9e1c-353f55d983c9 — Sardine — *Lead, Agentic Commerce (Product & GTM)*, job posting; y *Series C Funding Announcement*.
[34] https://arxiv.org/abs/2609.11757 — Ling et al. — *Signing the Transaction but Not the Decision: Whisper Attacks and a Binding Defense for AP2*, arXiv 2609.11757, 2026.
[35] https://www.cheq.ai/blog/agentic-commerce-explained — CHEQ AI — *Agentic Commerce: Meaning & What to Know*, 10-nov-2026.
[36] https://arxiv.org/html/2607.12575v1 — Ling et al. — *How Agentic Is Agentic Commerce? A Population-Scale Measurement of x402 Adoption and Authenticity*, arXiv 2607.12575, 2026.
[37] https://machineobserver.com/research/what-do-ai-agents-actually-buy — Machine Observer — *What Do AI Agents Actually Buy?* (x402), 2026.
[38] https://www.quantumpayments.io/blog/191-million-agent-payments-and-41-million-settled-the-x402-reality-check-for-merchants — Quantum Payments — *x402 agent payments: 191 million transactions, $41M settled — the reality check for merchants*, 2026.
