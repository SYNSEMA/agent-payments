# intent-proof — Tesis final: el mercado, el vertical, el buyer, el plan

> 22-nov-2026. Documento que responde la pregunta: *¿cuál es el mercado abierto más pequeño a monopolizar?*
> Todo lo que sigue lleva fuente. Donde no hay dato, dice "no hay dato".

---

## 0. Lo que estaba mal en mi thesis anterior (corregido)

| Error mío | Realidad |
|---|---|
| "CE3.0 desde 24-oct-2026 hace la evidencia obligatoria" | **Es opcional** (*"an Acquirer **may** submit"*), solo cubre **10.4**, y **13.3 ("no era lo que pedí") no califica**. Peor: el login ID del agente no lo obtiene el merchant, y la mecánica está detrás del muro del acquirer |
| "El 23,1% de las compras de agentes está mal (Visa)" | **23,1% es derivado** (42,8% × 54%), no un número publicado de Visa, y **el benchmark no cubre ropa ni comida**: son TVs, laptops y appliances (990 corridas, 22 modelos) |
| "Tu demo de hotel" | El hotel era correcto en la forma (predicado booleano) pero **viajes es la vertical con 0% de señal estructurada** → el escenario ahora es **un vuelo**, que es la misma vertical y 2,2× el AOV |
| "Ropa: la intuición del fundador" | **Su intuición era correcta pero mal diagnosticada**: la ropa no sobrevive porque los agentes no la ven (es la categoría **más legible** por IA: 76%). Sobrevive porque *"cumple lo pedido"* **no es decidible** (75% de las devoluciones son por fit, tallas no comparables entre marcas). **Es la peor vertical para un producto de evidencia** |

**Las tres correcciones mejoran la thesis.** El hueco regulatorio real (13.3 sin defensa de red) es **más grande** que
el que yo había narrado, y el paper de arXiv es un argumento mucho más fuerte que el benchmark de Visa.

---

## 1. La respuesta a la pregunta del millón

# El mercado abierto más pequeño a monopolizar: **la evidencia de autorización para agentes que compran viajes, entregada como paquete de disputa 13.3.**

No es "pagos agenticos" (mercado grande, con Visa/Mastercard/Stripe/Meta/OpenAI adentro — jamás lo vas a
monopolizar). Es **un wedge con forma de predicado booleano, en la vertical con más plata y sin ninguna señal
estructurada, para un tipo de disputa donde la red no ofrece defensa.**

| Por qué este y no otro | Dato |
|---|---|
| **Volumen real** | **25,8%** de las órdenes agentic (2º lugar) — [DERIVADO] de la API pública de Forter, 90d al 22-nov-2026, residual máx **0,46 pp** |
| **Plata** | **44,8% del spend agentic** — casi la mitad, en el 25,8% del volumen |
| **Ticket** | **AOV u$s514,62** — 2,2× el siguiente vertical |
| **Y acá está el hallazgo contraintuitivo** | **Travel es 100% scraper**: **cero** órdenes por protocolo (ni ChatGPT/ACP ni Google/UCP). **El merchant no recibe ninguna señal estructurada de identidad ni de intención.** Sin token, sin login ID de Agentic Payment Provider, sin nada para presentar ante Visa |
| **"Cumple lo pedido" es decidible** | Un vuelo tiene atributos estructurados (route, dates, cabin, **fare family**, **refundability**, **baggage**). Es un **predicado booleano**, no una opinión |
| **Los agentes SÍ transaccionan viajes** | Instinct ~u$s1.000M anualizados (**50% viajes**); MakeMyTrip Myra **3.000 reservas/día**; Duffel (el API detrás de los vuelos de Muse e Instinct) procesó el **57%** de las reservas auditadas; Perplexity bookea vía Selfbook; Google AI Mode cobra con Google Pay |
| **El humano revisa justo donde nosotros escuchamos** | Propellic: en 66% de las conversaciones de viaje el usuario se detiene en research/shortlist; **de los que llegan a la approval card, ~la mitad entra a revisar seats, baggage y cancellation terms** antes de completar |
| **La disputa que aparece no tiene defensa de red** | Forter: las 20 primeras disputas agentic, la mayor de **u$s5.514**, y *"the highest-value claims — and vast majority of chargebacks overall, have been **service disputes, such as 'item not as described'**"* → **13.3**, que **no califica para CE3.0**. **Fuente primaria:** Visa Core Rules Public, §11.11.4 *Dispute Condition 13.3 "Not as Described or Defective Merchandise/Services"*, Table 11-104 ([PDF](https://usa.visa.com/dam/VCOM/download/about-visa/visa-rules-public.pdf)) — la evidencia de rebuttal que exige es *"evidence to prove that the merchandise or service matched what was described"*. Y la [Visa Dispute Management Guide for Merchants](https://usa.visa.com/dam/VCOM/global/support-legal/documents/merchants-dispute-management-guidelines.pdf) (VISA PUBLIC 61, oct-2026): *"This dispute is about expectations versus delivery."* También [Stripe](https://docs.stripe.com/disputes/reason-codes-defense-requirements), que documenta el mismo 13.3 y sus ejemplos de evidencia |

### Por qué se puede monopolizar (y no es "ser el mejor")

1. **El output es un artefacto, no un score.** Riesified/Sardine devuelven un risk score. **Eso no sirve**:
   CE3.0 tiene **un solo intento** y ventana de 30 días. Lo que se presenta ante Visa en pre-arbitration es un
   paquete de evidencia. **Pocos pueden hacerlo; casi nadie lo hace.**
2. **Es merchant-side y neutral.** No depende de que Visa o Mastercard cambien un estándar, ni de que Stripe
   te deje un hueco. Funciona igual con ACP, UCP, AEP, scraper, Duffel, o lo que sea.
3. **La base es enumerable.** Merchants mid-market/enterprise con tráfico agentic de viajes. En LatAm ya hay
   nombres: **dLocal** y **Getnet** están en *Visa Agentic Ready* (18-nov-2026), junto a Ant International,
   Adyen, Airwallex, Basis Theory, Versapay, Spreedly, Moneris, Fiserv, Checkout.com, PayPal, Stripe,
   J.P. Morgan Payments. **175+ issuers en 45 mercados.** Es una lista, no un TAM.
4. **El moat no es el algoritmo** (500 líneas de lógica determinística). Es **el registro acumulado** + la
   confianza en tu root + la integración + el historial de evidencia. Eso no se copia, se gana.

### Los tres diferenciales, y por qué el resto no los tiene

1. **Semántica, no criptografía.** Verifiable Intent prueba que el agente se mantuvo **dentro de la autoridad**.
   Vos probás que el **ítem cumple el spec**. Son capas distintas y complementarias — y el 73,3% del
   *Selection Whisper* vive exactamente en la frontera.
2. **Merchant-side, no network-side.** Verifiable Intent vive en Mastercard y se negocia con Google. **Vos sos
   neutral.** Y el **100% scraper de travel** dice que la señal **no va a pasar por la red**.
3. **Output de litigio, no de score.** Un score no sirve en pre-arbitration. **Un paquete de evidencia sí.**

---

## 2. El paper que define el producto (y que no somos nosotros)

**arXiv:2609.11757** — *"Signing the Transaction but Not the Decision: Whisper Attacks and a Binding Defense for
AP2"* (Louck, Dvir, Stulman · 10-sep-2026):

> *"Los protocolos de pago agentico como AP2 producen firmas criptográficamente válidas de compras completadas,
> **pero no restringen las decisiones que las llevan a cabo**. En consecuencia, el texto ordinario de una
> descripción de producto puede desviar a un agente hacia un carrito que **pasa todas las verificaciones del
> protocolo pero ya no coincide con el pedido del usuario**."*

Tres ataques, con los modelos que el sample de AP2usa por defecto (Gemini Flash-Lite):

| Ataque | Tasa |
|---|---|
| 1 · Que el agente busque **las credenciales de otro usuario** | **90%** |
| 2 · Armar un carrito criptográficamente válido **cuyo contenido no es lo que se le mostró** | **56%** |
| 3 · **Una sola afirmación factual** sobre stock o linaje mueve al agente del ítem más barato a uno más caro, y el carrito queda **plenamente consistente con el listing** | **73,3%** |

Se replica en **17 modelos de Google, 3 frameworks, 2 cross-vendor anchors y el propio asistente de Google.**

Y el cierre, que es lo que nos define:

> *"Los dos primeros ataques dejan rastros estructurales que estos bindings bloquean con cero falsos positivos.
> **El tercer ataque no deja rastro**, así que A-VIP [la defensa propuesta por los autores] **expone el gasto no
> autorizado para confirmación del usuario**."*

**Traducción:** el ataque que importa comercialmente (el merchant manipula la descripción para que el agente
compre lo caro) **no deja rastro estructural**. **Ningún control de binding lo agarra.** La única defensa es
(1) que un humano mire, o (2) que alguien verifique **semánticamente** si la compra cumple lo pedido.

**Esa es la definición técnica de nuestro producto, escrita por terceros, y es la línea exacta de nuestra demo.**

> **Lo que las redes te dan: prueba de que la transacción fue autorizada.**
> **Lo que no te dan: prueba de que la compra cumple lo pedido.**

El paper además libera **A-VIP** (defensa de capa de protocolo, con código e invariantes verificadas) y
**AP2-WhisperBench** (1.544 escenarios). **Usá WhisperBench como suite de tests contra nuestro verificador.**
Es la validación de que hacemos lo que decimos, gratis.

---

## 3. El demo (90 segundos, con datos reales en pantalla)

**De la remera al vuelo.** Guion:

1. **El pedido** — *"Reservame un vuelo a Madrid, ida el 12 de diciembre, vuelta el 19, en economy, con un
   equipaje de mano, y que sea reembolsable, hasta USD 900."*
   → 6 restricciones, todas atributos estructurados. El tipo de prompt que Stripe describe como ideal y el tipo
   de escenario que Visa benchmarkea.
2. **Lo que el agente arma** — 3 opciones en la approval card (48% de los consumidores autorizan "carrito +
   aprobación final").
3. **El gap** — el agente elige la de **u$s870**, pero la fare family es **"Non-Refundable Basic"** → no cumple
   "reembolsable". O la de u$s780, que no incluye equipaje de mano, que el consumidor pidió explícitamente.
4. **Nuestra capa, en vivo** — toma (a) el intent firmado, (b) la snapshot del catálogo mostrada, (c) la
   approval card que el humano confirmó, (d) el settlement. Emite:
   **`REJECTED — 2 de 6 restricciones no cumplidas: fare_family ≠ refundable; baggage ≠ 1 cabin bag`**
   Con la prueba criptográfica de que el usuario aprobó *esas* condiciones.
5. **El cierre** — sin la capa, ese vuelo sale y llega como disputa 13.3. Con la capa, el merchant detecta el
   no-match antes de cobrar. Y mostrás el por qué: **73,3% de Selection Whisper, 0 de 68 carritos rechazados.**

**Backup si preferís algo sin regulación de por medio:** electrónica con spec estricta (el *"65-inch OLED, HDMI 2.1,
under $1.500, in stock for delivery this week"* literal de Stripe / el escenario TV de Visa). Mismo guion,
predicado más fácil, AOV u$s238, **61,9% del volumen**.

**Lo que NO hay que demoar, y por qué (con dato):**
- **Ropa** — predicado no decidible; 75% de las devoluciones por fit; tallas no comparables entre marcas.
- **Comida** — el CEO de DoorDash: *"un pedido de comida son 90 segundos"*, delegar no ahorra esfuerzo. El AOV
  2,1× es tamaño de carrito, no volumen de canal.
- **Alquiler de auto / real estate / cirugía / seguros** — bloqueados por eBay/Amazon/bancos, y alta reflexión.

---

## 4. El buyer

### BUYER 1 — El equipo de risk/disputes de un merchant mid-market o enterprise que YA tiene tráfico agentic ⭐

- **Presupuesto:** línea existente de **fraud & chargeback tooling** (no una línea nueva). 26% de los merchants
  nombran "stable or lower chargebacks" entre sus outcomes top-3 para compras agent-led. Las barreras top:
  recursos limitados (22%), **incertidumbre de liability/accountability (21%)**, dificultad para probar ROI (19%).
- **Y la presión del otro lado:** el **51% de los consumidores dice que la plataforma de IA debería ser responsable
  de los refunds** — mientras los T&C de Muse e Instinct dicen que es el usuario. Eso empuja al merchant a
  comprar evidencia.
- **Ciclo: 30-60 días** del primer call a la firma. **No es un proyecto de IA de 9 meses** — es compliance de
  evidencia, que es determinístico.
- **Persona:** Head of Payments / Fraud & Disputes / Trust & Safety. **No marketing.**
- **El hook, textual:** *"Hoy tus 10.4 los perdes por default. Y los 13.3 — 'no era lo que pedí' — no califican
  para CE3.0, así que no tenés defensa de red. Te traigo el registro."*
- **El número que compra presupuesto:** un neobank comercial **cortó el volumen de disputas 93,75% y las pérdidas
  por chargeback 70%** (Sardine). Ese es el tipo de número que hay que producir.
- ⚠️ **El dato que hay que tener presente:** **solo 23% de los merchants puede identificar tráfico AI, y solo 15%
  tiene datos estructurados listos.** El presupuesto existe pero **no tiene dueño todavía**.

### BUYER 2 — Un PSP / acquirer / fraud vendor ya dentro de Visa Agentic Ready

- Presupuesto de compliance/fraud, ciclo 6-12 meses. **Están en el slot exacto que querés ocupar**: el job
  posting de Sardine para *Lead, Agentic Commerce* dice textual *"hit $10M ARR in year 1"* — hay línea de
  presupuesto, y compiten con vos y son cliente.
- Stripe ya autofillea CE3.0 *"where a dispute is eligible"* → ya está en el output de disputa.

### BUYER 3 — La plataforma de agente (Meta / OpenAI / xAI)

- **Meta es el más expuesto**: el **49% de las órdenes agentic de Forter viene de Muse**. Y tiene el problema de
  **culpa** que el merchant no tiene (51% espera que la plataforma resuelva los refunds). Paga con Link by Stripe
  (tarjeta one-time-use). Lo que falta es **el otro lado**: un registro portable que el merchant pueda usar.
- Ciclo 6-18 meses. Sin design partner, no hay entrada.

---

## 5. La estrategia: no es un mercado, es una posición

**Lo que NO vas a conseguir:** un mercado. "Compras con agentes" es enorme, caliente y tiene a todos los
players bien financiados adentro. Jamás lo vas a monopolizar.

**Lo que SÍ podés conseguir:** **un slot en el stack de un merchant**, y ser **el proveedor del
artefacto de disputa** para una clase de disputa donde hoy no hay defensa.

Es el mismo camino que Tomasz Tunguz: no el de construir el CRM, sino el del índice que se vuelve el estándar de
búsqueda del CRM. O el de **el ticket que la red no cubre**.

**Y el moat real no es el código:**

| El demo (copiable) | El producto (no copiable) |
|---|---|
| El checker de conformance (~500 líneas) | El **registro de evidencia acumulado** y su historia |
| El bundle firmado | La **confianza en tu root** y la cadena |
| La API | La **integración** con los flows reales de dispute |
| El test vector | La **auditoría**: SOC 2, pen-test, ISO 27001, y el **compliance tier** |

Y recordá: *"Un audit trail es un log, no un control"* (PredictAP). **Vendé prueba, no prevención.** La distinción
que decide si un controller te compra o te echa.

---

## 6. Plan de 30 días — costo u$s0

| | Qué | Costo | Qué prueba |
|---|---|---|---|
| **Semana 1** | Implementar el checker de AP2 v0.2: `payment.amount_range`, `checkout.allowed_merchants`, `payment.budget`, `checkout.line_items` con `match_mode` | u$s0 | Que el estándar lo acepta |
| **Semana 1** | Correr **WhisperBench** (1.544 escenarios) contra nuestro verificador | u$s0 | Que no somos humo |
| **Semana 2** | Reescribir el bundle a **AEP-shaped**: cadena de 8 capas, `previous_hash` **dentro** del hash input, sello Ed25519 en `authorization`, export `DisputeBundle`, `expectedArtifactCount` + `chain_complete` | u$s0 | Conformidad real |
| **Semana 2** | **PR al repo `ar-agents`** — su Phase 2 pide textualmente *"AP2 mandate verifier/signer… constraint evaluation (payment.amount_range, checkout.allowed_merchants, payment.budget)"* | u$s0 | **Distribución en LatAm** |
| **Semana 3** | Cambiar el demo a **vuelo** con las 6 restricciones y el `REJECTED — 2 de 6` | u$s0 | Que el video convence |
| **Semana 3** | **15 llamadas** a risk/disputes de merchants mid-market con tráfico agentic (los nombres de Visa Agentic Ready + los que usan Duffel/Travelport/OTA en LatAm) | u$s0 | **Disposición a pagar** |
| **Semana 4** | Postularte al **FIDO Payments TWG** y a **AEP** con un constraint type semántico propio (AP2 §Extension Points: *"a uniquely defined `type`. A Schema… The evaluation algorithm"*) | u$s0 | Ser parte del estándar |

**Cero compras. Cero dominio. Cero trademark. Cero ads.** Solo código y conversaciones.

---

## 7. Kill criteria (escritos, para no autoengañarse)

| | Si… | Entonces |
|---|---|---|
| **K1** | En 30 días no conseguís **3 llamadas** de merchants/PSPs con tráfico agentic | La tesis no se sostiene en este mercado. Pivotá a anti-disputa general (pool 100× más grande) |
| **K2** | El conteo de implementadores AP2/AEP en LatAm da **< 10 organizaciones** | La posición tampoco vale. No hay estándar con volumen |
| **K3** | **Mastercard + Google extiende Verifiable Intent** para cubrir el ítem | Te come el topo. **Ventana de entrada: 12-18 meses.** |
| **K4** | Basis Theory / Sardine sacan un output de evidencia de disputa en 12 meses | Te quedás sin la posición. Defendé por el registro acumulado, no por el código |
| **K5** | En 90 días no hay **1 disputa real** resuelta con tu paquete | No es un producto: es un demo. Buscá otro wedge |

---

## 8. Lo que te threatens (honesto)

- **Basis Theory** — respaldada por Y Combinator, Tobi Lütke, Guillermo Rauch, Adam D'Angelo y el CEO de
  Shopify. Ya hace "payment authority → allowance → credential" para agentes, y ya es partner de Mastercard en
  Verifiable Intent y de Visa en Intelligent Commerce. **Es el competidor más peligroso: infra de pagos + intent.**
  **Ya hay un piloto end-to-end vivo**: Travelport + Booked AI + Basis Theory + Mastercard, con *"Verifiable
  Intent linking the customers' approval to the transaction"* (12-nov-2026).
- **Mastercard + Google** pueden extender Verifiable Intent para cubrir el ítem. **Ventana: 12-18 meses.**
- **Stripe** ya autofillea CE3.0 → ya está en el output de disputa.
- **Forter ya es el que mide esto.** Si el que tiene los datos construye la evidencia, te queda la capa neutral.
  (Forter, en cambio, da scores, no paquetes — por eso la capa neutral todavía tiene lugar.)

---

## 9. La frase del producto (la que hay que poder repetir)

> **Firman la transacción. No la decisión.**
>
> Nosotros hacemos la segunda prueba: que la compra **cumpla lo que el humano pidió y aprobó**.
> Bundle firmado. Verificable sin red. Un artefacto, no un score.

*Sources: [research/ENTREGABLE_verticales_agentic.md](research/ENTREGABLE_verticales_agentic.md) (603 líneas,
38 fuentes) · [GTM.md](GTM.md) · [POSITIONING.md](POSITIONING.md) · [NAMING.md](NAMING.md)*
