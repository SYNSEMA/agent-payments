# intent-proof: pricing, economics, estructura y GTM

## Tesis en dos párrafos

El verificador criptográfico es barato (margen >95%) pero **el producto no se vende solo**: hay que vender *evidencia que alguien con presupuesto ya está durmiendo mal*. Y el 24 de marzo de 2026 Mastercard y Google open-sourced **Verifiable Intent**, un framework criptográfico que hace casi exactamente lo que hace intent-proof: delegation chains con SD-JWT, constraints criptográficamente verificables, agnóstico de protocolo (mapea a AP2, ACP y UCP) y selective disclosure[38][39]. Sumale Visa Trusted Agent Protocol, que ya exige "Purchase Instructions" para evidenciar la transacción autorizada en disputas[41][83]. **El spec te lo regalaron dos de las tres redes más grandes del mundo.** Tu producto tiene que ser otra cosa, y el informe termina diciendo cuál.

Segunda tesis, y es la que decide la empresa: **un take rate del 1% no cierra**. El mercado de end-to-end agentic commerce en retail de EE.UU. es $1,44B en 2026 (eMarketer)[81][84]. El 1% de eso son $14,4M de pool de revenue para *toda* la categoría, con todos los players. Tomando 10% sos $1,44M ARR. Eso no es una empresa, es un proyecto. El take rate solo funciona sobre un pool que ya es grande (payments, no agentic commerce), y el pool grande de AI-driven commerce ($144B en 2029, 8,8% del retail e-commerce[82]) incluye discovery sin checkout agentico, donde tu evidencia no se necesita.

## 1. Benchmarks de pricing de infra de pagos

| Vendor | Modelo | Precio | Unidad real |
|---|---|---|---|
| Stripe Billing | SaaS barato + take rate | 0,7% del billing volume, sin fee recurrente; tiers $620/$1.500/$2.950/$5.750/mes | Contrato vs pay-as-you-go se cruzan en $88.571/mes de billing[1][4] |
| Stripe Payments | Procesamiento | 2,9% + 30¢ (ACH 0,8%, cap $5) | Por transacción exitosa[1] |
| Stripe Revenue Recognition | Suscripción aparte | $25/mes (hasta $10k vol) → $190/mes anual (hasta $100k) + 0,2% excedente | Cobran su propio schedule, no el pago[3][4] |
| Stripe Invoicing | Take rate | 0,4% (Starter) / 0,5% (Plus) por factura pagada | Base distinta a la de Billing[4] |
| Stripe Tax | Híbrido | 0,5% en Billing-integrado; 50¢ por txn vía API (10 calls incl., 5¢ extra) | Mismo producto, dos precios según integración[53] |
| Shopify Plus | SaaS + % GMV | $2.300–2.500/mes + 0,15% de transaction fees con third-party | Cobran por *compatibilidad*, no por Gateway[8][9] |
| Paddle | MoR all-in | 5% + 50¢, sin fee mensual | Tax compliance incluido en el take[10] |
| Lemon Squeezy | MoR all-in | 5% + 50¢ — **cierra 31/3/2027**, ya no acepta signups | Muerte del MoR pequeño[11][12] |
| Adyen | Fee fijo + método | $0,13 + Interchange++ + 0,60% (US) | Sin setup ni mensual[54] |
| Chargeflow | Éxito + por caso | **25% de lo recuperado** (solo wins) + $29/deflection + $0,10–0,20/txn scaneada; 1.000 gratis | Cero costo si no ganás[13][14] |
| Justt | Quote-only | Solo se publica "pagás cuando ganás" | Enterprise, sin lista de precios[78] |
| Sardine | Free tier + plataforma | 1.000 API calls gratis → **Growth $600+/mes** | Cobran por decisión, no por seat[15] |
| Alloy | Quote-only | Sin página de pricing; estimaciones de terceros: ACV $120k–500k/año | Todo enterprise, nada self-serve[80][79] |
| Didit | Per-check transparente | KYC bundle $0,33; AML $0,20; KYB desde $2,00; txn monitoring $0,02 | El contraejemplo: per-check con lista de precios[79] |
| Basis Theory | Suscripción con volumen | **$995/mes incluye 20.000 tokens** (=$0,0498/token), PCI AOC incluido | El benchmark más cercano a un evidence-layer[50] |
| Crossmint | Freemium por unidad | Agent Wallets 5.000 MAW gratis → $0,01/MAW; Agent Checkout 1.000 gratis → **$0,05/checkout** | Self-serve[51] |
| Stripe SPT | Per-token | **$0,15 por Shared Payment Token emitido** | El precio de lista del primitive de pago delegado[73][74] |

**El patrón que funciona en fintech B2B:** suscripción base + unidad de usage + take rate *solo cuando el vendor controla el rail*. Stripe cobra 0,7% porque Stripe es el rail. Shopify cobra 0,15% porque Shopify *es* el checkout. Agentcard cobra 1% porque emite la tarjeta[5]. Basis Theory cobra $995/mes porque el vault **es** el compliance boundary. Chargeflow cobra 25% porque el outcome —el recupero— es medible y atribuible.

Vos no controlás el rail. Entonces el take rate es la última opción, no la primera.

## 2. Qué paga un app de agentes hoy y qué margen deja

| Oferta | Precio | Consecuencia |
|---|---|---|
| Agentcard Vault | Gratis hasta 5.000 usuarios | Funnel massive, enterprise aparte[5] |
| Agentcard Issuing | $5.000/mes + **1% revenue share** + $0/tarjeta | Breakeven = **$500.000 de card spend/mes** (2.451 órdenes @ AOV $204) |
| opencommerce Standard | **1% de cada orden completada** | El take rate que sí funciona: el agente elige el canal[5] |
| Rain | Sin precio público, demo-only; Agent Vaults: "tu plataforma gana el interchange" | Programa Agentic Startup: $5.000 de crédito, cohorts de 5, 10 semanas — su canal developer-led[47][48][49] |
| Skyfire / Basis / Crossmint | Quote o per-unit | Skyfire sin lista; Basis $995; Crossmint $0,05/checkout[50][51][20] |
| OpenAI Instant Checkout | 4% reportado a merchants Shopify (ene-2026); hoy $0 tras mover el checkout al merchant[75][74] | El platform fee mutable y de un solo actor |

El AOV agentico es **$204, 2,3x el humano ($89)**[77], con ticket por plataforma entre $89 y $146[76]. Ordená eso: 1% sobre $204 da $2,04; sobre un AOV de $50, $0,50.

**El 1% no falla por margen — falla por mercado.** Costo de servir ≈ $0,0002 por bundle (firma Ed25519 ~100µs + fila Postgres ~2KB): margen bruto 99,6%. El problema es otro:

1. **Cuesta 15x más que el dolor que alivia.** El ticket promedio de un chargeback en EE.UU. es **$110**, y la tasa CNP promedio 0,65%[29][55]: eso son **$0,72 de pérdida esperada por orden**, o 0,35% de un orden de $204. Sumando el costo total de gestión (que las fuentes secundarias estiman en ~$128 all-in, con $82 de costos internos y $46 de fees de terceros[55]), el take de 1% cobra 15x más que el daño que evita. Nadie lo compra como seguro.
2. **El pool es chico** ($14,4M de revenue pool total a 1%[81]).
3. **No tenés poder.** El merchant ya paga 5,8–8,1% en total (Stripe 2,9%+30¢ + interchange 1,59%–2,92% + opencommerce 1%)[1][35][37][5]. Sumar 1% más es sumar 14–17% a su costo, sin controlás el canal.
4. **Conflicto de incentives.** El que elige el canal es el agente, no vos. El 1% de opencommerce funciona porque *es* el canal[5]. Un evidence-layer es invisible en la decisión de compra: no hay take rate defendible.

Lo que sí justifica un take rate es **resultado medible y atribuible**: Chargeflow cobra 25% de lo que recupera[13]. Ahí el dolor es del cliente y vos sos el que lo resuelve.

## 3. Estructura de empresa: qué te hace falta y qué no

| Pregunta | Respuesta | Base |
|---|---|---|
| ¿Licencia bancaria? | **No**, si nunca tocás fondos ni iniciás pagos | Un processor que actúa como agente del merchant no es money transmitter[22]; la definición de FinCEN exige aceptar y *transmitir* fondos[21] |
| ¿PCI DSS? | **No**, pero necesitás opinión formal de scoping | SAQ A dice textual "no aplicable a service providers"[60]; un provider que *afecta* el CDE del cliente puede entrar igual[61][62]. Opinión escrita: $2–5k |
| ¿SOC 2 Type II? | **No en el año 1** | Exige 3–12 meses de observación + ~8 semanas de fieldwork: 5–15 meses realista[58][59]. Costo $12–20k (Drata SMB) + plataforma $7–15k/año[23][24] |
| ¿SOC 2 Type I? | **Sí, mes 4–6** | $7,5–15k[25]. Es lo que te pide un enterprise de $20k+/año y lo que Agentcard incluye en su tier Enterprise[5] |
| ¿Cyberscore / ISO 27001? | No, ruido de marketing | Basis Theory tiene SOC2 + PCI L1 + ISO 27001 y cobra $995/mes[50]: el ISO es decorativo |
| ¿FinCEN / money transmission? | No — **guardá la frontera** | Si alguna vez cobrás un % del valor de la transacción o custodias saldo, entrás[21]. Per-verification / per-case / licencia te deja fuera |
| ¿KYB del cliente? | **Sí, inevitable** | No onboards end-users, pero vendés a issuers/PSPs: KYB + beneficial owners |
| ¿Pen-test + bug bounty? | **Sí, año 1** | Pen test $5–30k (promedio $18.300); bounty $500–10.000 por hallazgo válido, críticos hasta $50k+[56][57] |

**Societario desde Argentina:** SAS, no offshore. La Ley 27.506 *Economía del Conocimiento* baja Ganancias de 35% a **14%** (micro), 21% (medianas), 28% (grandes), convierte 70% de cargas patronales en crédito fiscal, hasta dic-2029; exige ≥70% de facturación en actividades promovidas + 2 de 4 condiciones (exportar es una)[64][65]. Exportás servicios con Factura tipo "E" y no pagás derechos de exportación de servicios[66]. **No te constituiste en Delaware hasta que levantes ronda lede por EE.UU.**: con $50k no necesitás esa complexity. Facturá en USD.

Regulación local: Argentina no tiene licencia fintech omnibus; PSP y PSPCP se registran en BCRA, PSAV en CNV, y no hay sandbox formal cross-sector[63]. Si intent-proof nunca mueve fondos ni es PSP, no entrás en ninguna de esas listas — pero confirmalo por escrito antes de vender a un banco.

## 4. GTM motion: qué funciona cuando vendés una API

Los que venden infra de pagos agenticos hoy: **Rain** (demo + cohorts de 5 startups + 25+ miembros en la Agentic Payments Alliance con Visa, Mastercard, Fiserv, Circle)[16][47]; **Crossmint** (self-serve: "point your coding tool at our docs")[18]; **Basis Theory** (register self-serve + demo enterprise)[50]; **Skyfire** (KYA token, partnerships con Akamai, Okta, Forter, Mastercard)[20].

Cuatro movimientos, en orden de costo:

1. **Self-serve con free tier generoso.** Crossmint y Agentcard usan free tiers de 5.000 unidades[51][5]. El tuyo: 5.000 verificaciones/mes gratis. Costo de servirlo: <$5. Es marketing con ROI positivo.
2. **Design partners pagadores.** 3 a $2–5k/mes, con un entregable concreto por socio (un bundle que el issuer pueda presentar en un dispute real).
3. **Vender al que ya tiene el presupuesto del dolor.** Chargeflow vive de chargebacks; Justt, Sardine, Forter de fraud; Basis Theory de compliance. Si tu bundle es la evidencia que les falta, entrás por presupuesto existente.
4. **OEM / compliance license.** Basis Theory embebe para Crossmint[19]; vos cobrás por año, no por orden.

Lo que **no** funciona: vender "confianza" a developers en abstracto. El developer no tiene el problema; el merchant, el issuer y el chargeback vendor sí.

## 5. Costos reales de servir y de confianza

| Ítem | Costo | Nota |
|---|---|---|
| Servidor Synsema (1 binario) | €11,99/mes (Hetzner CPX12) | El más barato medido[69] |
| App + Postgres 10GB | $20,15/mes (DigitalOcean) | Railway $20, Render $35, Fly $46, Koyeb $59[68] |
| Postgres administrado | $38/mes Fly Basic + $0,28/GB; $15,15 DO 1GB | En Fly, 89% del bill es DB[67][68] |
| Storage de bundles | 1M bundles/mes ≈ 1,9 GB ≈ **$0,53/mes** | Prácticamente gratis |
| Firma Ed25519 | ~100µs | 1M firmas = 100s de CPU en un vCPU |
| Pen-test | $5–30k | Desde $5.085/test (Synack)[56][57] |
| SOC 2 | $12–20k (Type II) + $7–15k/año plataforma | [23][24][25] |
| Bug bounty | $3–10k/año | [56] |
| QSA scoping opinion | $2–5k | Evita una surprise de $30k+ |

**El costo variable es ~$0,0002 por bundle. Todo lo demás es confianza, y la confianza no escala linealmente.** Margen bruto >99% desde el primer bundle — y ese es el problema: no necesitás volumen para ser rentable, necesitás *una razón para que te paguen*. Un producto con 99% de margen y cero switching cost es un producto que el cliente te reemplaza cuando se agrees con Mastercard.

## 6. Los seis modelos: cuál es defendible

| Modelo | Precio | ¿Defendible? | Por qué |
|---|---|---|---|
| (a) SaaS por-seat | $X/usuario | **No** | Un agente no tiene seat. Sardine cobra $600+/mes, no por cabeza[15] |
| (b) Take rate por orden | 1% del GMV | **No** | 15x el dolor que alivia, pool de $14,4M, y no controlás el canal[81][29][55] |
| (c) Per-verification | $0,05/bundle (SPT $0,15; Basis $0,0498; Crossmint $0,05) | **Sí, base** | Costo marginal $0,0002 → margen >99%. Es el lenguaje de la infra real[50][51][73] |
| (d) White-label OEM | $15–40k/año | **Sí, complementario** | Basis Theory vive embebido en Crossmint[19][50] |
| (e) Licencia de compliance | $24–60k/año a issuer/PSP | **Sí, el mejor contrato** | Budget de compliance existente, dolor medido ($110 ticket promedio US, hasta $120 en viajes)[29], ciclo justificable a ese ticket |
| (f) Cobrarle al merchant | 0,15–0,5% | **No** | Ya paga 5,8–8,1% y no tenés poder[1][35][37] |

**Veredicto: (c) como unidad de consumo + (e) como contrato grande + (d) como canal.**

Estructura concreta:

- **Community**: $0, 5.000 bundles/mes, verificador CLI offline, sin SLA.
- **Pro**: $399/mes incluye 50.000 bundles; $0,01/bundle excedente.
- **Scale**: $1.990/mes incluye 500.000 bundles; $0,004 excedente.
- **Enterprise / Compliance License**: $24k–60k/año, anual, SLA, SOC 2, AOC, key management, SSO.
- **Dispute package**: $29/case (anclado en Chargeflow[13]) + 20% de lo recuperado si vos defendés el caso.

## 7. Qué haría yo con $50k y 12 meses

Siendo escéptico: **no abriría el producto horizontal**. "Capa de confianza para compras agenticas" ya laphobicaron Mastercard y Google, gratis, con más legitimidad que vos[38][39]. Cualquier VC que sepa leer el estado del arte te va a preguntar qué hacés que Verifiable Intent no hace. Tenés una respuesta técnica genuina: **verificación offline, sin red, sin la clave del emisor en el verificador, deny-by-default** — el spec de Mastercard te da un formato de exchange que requiere el issuer online[41]. Pero eso es una feature, no un negocio.

**Lo que haría, en orden:**

**1. (Meses 1–2) Cambiar el producto de "confianza" a "defesa de disputas".** El mismo bundle, otro comprador, otro budget. Y hay tres hechos que vuelven esto urgente:

- El benchmark de Visa del 5/11/2026: 22 modelos, 57,2% de las corridas pasaron, y **~23% de todas las corridas terminan en una compra equivocada o prematura**; solo 34,2% pasaron las tres tentativas[26]. Los agentes compran mal a escala.
- Sardine documenta el hueco: al agente le falta el device signal del cliente, y una compra no reconocida te pega **doble** en el ratio de VAMP (TC40 + TC15)[52].
- **Las redes no tienen regla.** Ni Visa ni Mastercard van a publicar una regla vinculante de disputas agentic antes del release cycle de abril de 2027; las releases de octubre 2026 son solo identity, token y auth plumbing. Las disputas agentic de BFCM 2026 se van a adjudicar bajo reglas escritas para un humano apretando "comprar"[34].

El dolor ya está medido y es ajeno: 0,65% de chargebacks con ticket promedio $110 en EE.UU.[29][55]. **Vendé a quien ya tiene la línea presupuestaria: Justt, Chargeflow, Sardine, Forter, y en LATAM a Getnet, Kushki, Pomelo, Dock, Neivor** — estos últimos ya hicieron transacciones agenticas en vivo con Mastercard Agent Pay desde marzo de 2026, con Banco Nación Argentina, Galicia, Itaú y Santander en el circuito[44][45][46][72].

Ese último punto es tu ventana: **es el vacío regulatorio, y hay que ocuparlo antes que Mastercard.**

**2. (Meses 2–4) Nicho: viajes y e-commerce de alto AOV con agentes; y procurement B2B.** Viajes tiene la tasa de disputa más alta entre verticales mainstream (0,89–2,00%)[55] y el **ticket promedio de chargeback más alto de todas las verticales: $120 vs $110 promedio EE.UU.**[29]. Sumado al AOV agentico 2,3x el humano[77], el reclamo central —"el agente compró fuera de lo que autoricé"— es exactamente tu evidencia. En procurement B2B el comprador es el controller, con presupuesto de compliance y necesidad de audit trail. Arrancá con **3 design partners pagadores a $2–5k/mes** (no gratis) y un entregable por socio.

**3. (Meses 3–5) Self-serve con free tier + open-source del verificador.** Publicá el verificador offline (Synsema, un binario) como MIT, con binding a Verifiable Intent y TAP. Cada bundle emitido trae el adopter. El free tier de 5.000 verificaciones cuesta <$5 y trae developers que hoy no tienen forma de evaluar un proof sin red. Publicar no te mata: Mastercard ya regaló el spec; tu ventaja es ser *la implementación de referencia* que funciona sin issuer online.

**4. (Meses 4–9) Confianza, secuenciada.** (a) **QSA scoping opinion** $2–5k — es el blocker número uno en ventas B2B de pagos; (b) **pen-test** acotado $7k; (c) **bug bounty** $4k; (d) **SOC 2 Type I** $12k en mes 5–6. El **Type II lo hacés en el mes 12–15**, cuando los controles ya llevan 3 meses corriendo y el revenue lo justifica. No lo compres en el mes 1: es plata quemada antes de tener un cliente.

**5. Estructura:** SAS en Argentina, inscrita en Economía del Conocimiento apenas tengás exportaciones reales (el certificado de no-retención de IVA solo se pide una vez, con exportaciones previas)[64][66]. Facturá USD. Delaware cuando levantes ronda. KYB de clientes emisores desde el día 1.

**Presupuesto año 1:**

| Línea | Monto |
|---|---|
| Infra (Hetzner + Postgres, 12 meses) | $300 |
| Dominio, correo, monitoring, backups | $180 |
| Legal: QSA scoping, ToS/DPA, KYB, contratos enterprise | $8.000 |
| SOC 2 Type I + plataforma | $12.000 |
| Pen-test | $7.000 |
| Bug bounty + reserva | $4.000 |
| Distribución: SDK, docs, landing, samples | $5.000 |
| Runway fundador (viajes, eventos, demos, legal extra) | $9.500 |
| **Total** | **$45.980** (quedan $4.020) |

**Target realista a 12 meses:** 3 design partners a $3.500/mes + 8 clientes Pro/Scale + 1 licencia enterprise → MRR ~$16.500, **ARR ~$198.000**. Eso no es Serie A; es una company que levanta pre-seed con design partners y MRR real. Si a los 12 meses no tenés **un issuer o PSP en LATAM pagándote por compliance**, el pivote es vender evidencia a **todo** el e-commerce (no solo agentic) como capa anti-disputa: pool 100x más grande, producto idéntico, pero perdés el halo agentic y entrás en competencia con Chargeflow y Justt, que están financiados y tienen el canal.

Un dato más para calibrar el pesimismo: solo el 31% de los consumidores dice que dejaría a un agente completar una compra sin revisarla, y el 67% dice que es poco probable[84]. La adopción no está trabada por confianza en la UX; está trabada por confianza en el pago. Ahí hay producto — y hay presupuesto.

**La pregunta que te tenés que hacer antes de escribir una línea más de código:** ¿quién tiene hoy el presupuesto que estás tratando de capturar, y por qué te lo va a dar a vos y no a Mastercard? Si la respuesta es "porque somos más neutrales y más fáciles de auditar offline", tenés un producto. Si es "porque la evidencia criptográfica es mejor", no la tenés.

## Sources

[1] https://stripe.com/billing/pricing
[3] https://stripe.com/revenue-recognition/pricing
[4] https://cfopresso.com/blog/stripe-billing-pricing
[5] https://agentcard.com/pricing
[8] https://www.shopify.com/pricing
[9] https://help.shopify.com/en/manual/intro-to-shopify/pricing-plans/plans-features/shopify-plus-plan
[10] https://www.paddle.com/pricing
[11] https://www.lemonsqueezy.com/pricing
[12] https://www.lemonsqueezy.com/blog/saying-goodbye-to-lemon-squeezy
[13] https://www.chargeflow.io/pricing
[14] https://chargeflow.io/blog/chargeflow-vs-kount
[15] https://sardine.ai/pricing
[16] https://www.rain.xyz/resources/rain-launches-the-agentic-payments-alliance
[18] https://www.crossmint.com/announcement/agent-commerce-toolkit-launch
[19] https://www.crossmint.com/announcement/agentic-cards-api-launch-visa-basistheory
[20] https://paywithlocus.com/guides/agentic-payment-platforms
[21] https://www.fincen.gov/resources/statutes-regulations/administrative-rulings/definition-money-transmitter-merchant-payment
[22] https://www.fincen.gov/resources/statutes-regulations/administrative-rulings/whether-authorized-agent-receipt-utility
[23] https://drata.com/learn/soc-2/cost
[24] https://soc2auditors.io/resources/drata-vs-vanta
[25] https://justinmckelvey.com/blog/vanta-vs-drata
[26] https://tokenstrom.com/posts/nearly-1-in-4-test-purchases-went-wrong-who-pays-when-your-ai-agent-bu
[29] https://www.mastercard.com/content/dam/mccom/shared/news-and-trends/insights/2025/2025-global-chargebacks-outlook/pdf/2025-state-of-chargebacks-report.pdf
[34] https://shopappy.com/payments/card-networks/agent-chargeback-rules-holiday-2026
[35] https://nilsonreport.com/merchant-processing-fees-industry-investment
[37] https://merchantspaymentscoalition.com/credit-and-debit-card-swipe-fees-totaled-quarter-trillion-dollars-2025-far-higher-earlier-reports
[38] https://verifiableintent.dev
[39] https://mastercard.com/us/en/news-and-trends/stories/2026/verifiable-intent.html
[41] https://developer.visa.com/docs/trusted-agent-protocol/trusted-agent-protocol-specifications
[44] https://www.mastercard.com/news/latin-america/en/newsroom/press-releases/pr-en/2026/march/mastercard-advances-agentic-payments-in-latin-america-and-the-caribbean-with-live-transactions-completed-across-the-region
[45] https://www.santander.com/en/press-room/press-releases/2026/03/santander-and-visa-deliver-latin-americas-first-end-to-end-payments-powered-by-ai-agents
[46] https://www.santander.com/en/press-room/press-releases/2026/06/getnet-develops-infrastructure-that-enables-businesses-to-accept-ai-agent-initiated-payments
[47] https://www.rain.xyz/agentic-startup-program
[48] https://www.rain.xyz/product/agent-vaults
[49] https://www.rain.xyz/resources/introducing-the-agent-control-layer
[50] https://basistheory.com/pricing
[51] https://www.crossmint.com/pricing
[52] https://www.sardine.ai/blog/know-your-human-agentic-commerce
[53] https://stripe.com/tax/pricing
[54] https://www.adyen.com/pricing
[55] https://eightx.co/blog/average-chargeback-rate-by-vertical
[56] https://www.synack.com/blog/penetration-testing-cost
[57] https://pentestingcost.com
[58] https://www.vanta.com/collection/soc-2/soc-2-audit-timeline
[59] https://ysecurity.io/blog/how-long-does-a-soc-2-audit-take
[60] https://www.pcisecuritystandards.org/documents/PCI-DSS-v4-0-SAQ-A.pdf
[61] https://www.pcisecuritystandards.org/faqs/1580
[62] https://deployopen.com/certifications/pci-dss/guides/pci-dss-for-service-providers
[63] https://www.legal500.com/guides/chapter/argentina-fintech?export-pdf=
[64] https://wade.thorque.com.ar/blog/economia-del-conocimiento-2026
[65] https://ghostcrew.io/guides/knowledge-economy
[66] https://www.arca.gob.ar/monotributo/exportacion-servicios
[67] https://fly.io/pricing
[68] https://devtoolpicks.com/blog/railway-alternatives-priced
[69] https://www.hetzner.com/cloud
[72] https://paymentreview.com/reviews/kushki
[73] https://www.aissist.io/insights/best-agentic-commerce-platforms
[74] https://swell.is/content/agentic-commerce-protocol-explained
[75] https://radlof.com/articles/chatgpt-instant-checkout
[76] https://firstpagesage.com/seo-blog/agentic-commerce-statistics
[77] https://forter.com/blog/ai-agent-order-data
[78] https://justt.ai
[79] https://didit.me/blog/didit-vs-alloy-kyc-comparison
[80] https://aifintechindex.com/vendors/alloy
[81] https://digiday.com/marketing/in-graphic-detail-welcome-to-the-agentic-commerce-quarter
[82] https://www.emarketer.com/content/ai-commerce-2026
[83] https://developer.visa.com/docs/visa-intelligent-commerce/concepts
[84] https://martsignal.com/articles/agentic-commerce-hits-q4-openai-google-and-walmart-ship-agent-checkout
