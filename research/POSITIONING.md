# intent-proof — Posicionamiento y definición

> Artefacto de posicionamiento. 21-nov-2026.
> La definición es la herramienta de venta, de recruiting y de fundraising. Todo lo demás se apoya en esto.

---

## 1. La forma de la definición (por qué esta y no otra)

El liberalismo se define por una **frontera**: qué queda protegido y bajo qué principio.
El peronismo se define por un **proyecto**: qué se quiere hacer y quién lo encarna.

Una frontera se enuncia en una frase. Un proyecto no se cierra sin programa, y por eso admite quince
respuestas. Un negocio que se define como proyecto se diluye; uno que se define como frontera se vuelve
**inconfundible y repetible**.

La nuestra es una frontera. Y tiene tres piezas, en el mismo lugar donde el liberalismo tiene las suyas
(vida / libertad individual / propiedad privada = lo que no puede ser invadido):
**lo que se declara / lo que se aprueba / lo que se comprueba.**

---

## 2. La definición

### 2.1 Versión larga (estilo liberalismo)

> **El respeto irrestricto a la decisión de quien autoriza, basado en el principio de trazabilidad de la
> autorización, en la intención declarada, la aprobación del titular y la conformidad comprobada.**

### 2.2 Versión operativa (una frase, la que repite un CEO, un comercial y un dev)

> **Que ninguna compra de un agente exista sin la prueba de que un humano la pidió y la aprobó.**

### 2.3 Versión mínima (bio, tagline, placa del video)

> **El agente propone. La prueba decide.**

### 2.4 Nombre de la postura (candidatos)

- **Consentimiento demostrable** — el más fiel al derecho y al protocolo.
- **Autorización trazable** — el más industrial.
- **Trazabilidad de la intención** — el más descriptivo, el peor para vender.

**Recomendación: "consentimiento demostrable"** para hablar de la postura, y **"prueba de intención"**
para el nombre del artefacto que entregás.

---

## 3. El antagonista (complemento peronista: contra qué estamos)

El liberalismo no nombra a su enemigo con nombre; el peronismo sí. Nosotros también podemos:

> **Contra la confianza implícita.** Alguien tiene que probar, no alcanza con que quede registrado.

Tres formas del mismo enemigo:

| Forma | Cómo se manifiesta |
|---|---|
| **El agente que se auto-autoriza** | El agente afirma que el usuario pidió; el sistema le cree porque vino firmado. |
| **El log como evidencia** | "Quedó en el log" se usa como prueba. Un log registra; no demuestra. |
| **El merchant que no puede mostrar** | La regla de Visa y Worldpay: si no mostrás qué aprobó el shopper, perdés el dispute. |

---

## 4. Las tres cosas que se prueban

| | Qué es | Cómo se prueba | Quién lo firma |
|---|---|---|---|
| **Declarar** | la intención: qué pidió, con qué límites | hash de la intención dentro del bundle firmado | el emisor + el titular |
| **Aprobar** | quién autorizó, y que fue una persona | aserción WebAuthn (passkey / biometría) contra el challenge | **el humano**, con su clave de dispositivo |
| **Comprobar** | que el cargo coincide con lo aprobado | recálculo determinista del verificador | el verificador, offline |

**Nombre operativo de las tres:** *Declarar → Aprobar → Comprobar.*

---

## 5. Traducción por audiencia (misma definición, distinta entrada)

| Audiencia | Lo primero que pregunta | La frase que lo detiene | La prueba que lo convence |
|---|---|---|---|
| **Comercial** (merchant, PSP, plataforma) | *"¿me ganás la disputa?"* | "Tus 13.3 no tienen defensa de red: CE3.0 es opcional y solo cubre el fraude. Te traigo el registro." | 1 disputa agentic real resuelta con tu bundle en el export de representment |
| **Dev / CTO** | *"¿qué integro?"* | "Un bundle firmado, un endpoint, un verificador que corre sin red." | RFC 8785 + Ed25519 hoy; alineados a AEP. Test vectors + SDK |
| **CEO / fundador** | *"¿esto es una empresa?"* | "El agente no autoriza: la autorización se prueba." | Standards adoptados + design partners, no slides |
| **Finanzas / Controller / Compliance** | *"¿esto pasa una auditoría?"* | "Cada compra deja evidencia lista para auditor, no un log." | SOX 404 / COSO (feb-2026) / PCAOB AS 2201 (15-dic-2026) / EU AI Act Art. 11 |
| **Todo el mundo** | *"¿en una frase?"* | "El agente propone. La prueba decide." | El demo de 90 segundos |

---

## 6. El post de X (thread de 4)

> **1/4**
> Firman la transacción. No la decisión.
>
> Un paper de seguridad (arXiv, sept-2026) encontró que los protocolos de pago agentico —AP2, Verifiable Intent— **producen firmas criptográficamente válidas de compras que no coinciden con lo que el usuario pidió**.
> Una sola frase del merchant movió al agente del producto más barato al más caro en el **73,3%** de los intentos. Y los controles de binding **rechazaron 0 de 68** carritos: no dejan rastro estructural.
>
> **2/4**
> Y la red no cubre el hueco.
>
> Visa Compelling Evidence 3.0 (vigente 24-oct-2026) es **opcional** y solo aplica al reason code 10.4 — el fraude.
> La disputa que realmente está apareciendo es **13.3, "no era lo que pedí"**. No califica. **No hay defensa de red.**
>
> **3/4**
> Nosotros hacemos esa prueba. Por cada compra, un bundle firmado con:
> • qué se pidió, y con qué límites
> • quién lo aprobó (una persona, con passkey)
> • si el cargo coincide con lo aprobado
>
> Se verifica **sin conexión**, con solo claves públicas. Bundle en JSON firmado: sin lock-in.
>
> **4/4**
> Que ninguna compra de un agente exista sin la prueba de que un humano la pidió y la aprobó.
> El agente propone. La prueba decide.
> [link]

### 6.1 Post único (≤280)

> Firman la transacción, no la decisión.
>
> Un paper (arXiv, sept-2026): los protocolos de pago agentico producen firmas válidas de compras que **no coinciden con lo pedido**. El ataque funcionó el 73,3% de las veces y 0 de 68 carritos fueron rechazados.
>
> Falta la prueba. Bundle firmado, verificable sin red.

### 6.2 Post único, versión "agente" (sin jerga de redes)

> Un agente de IA te compra algo que no era lo que pediste. El pago es válido. El banco te cree. Nadie puede probar qué pediste.
>
> Nosotros hacemos esa prueba: un bundle firmado con qué se pidió, quién lo aprobó (passkey) y si el cargo coincide. Verificable sin red.

---

## 7. Los outputs de "qué somos"

### 7.1 Tagline (bio de X, placa del video, firma de mail)

> El agente propone. La prueba decide.

### 7.2 Una línea (para pedir meetings, pitch de 10 segundos)

> Evidencia firmada y verificable de las compras que hacen los agentes de IA: qué pidió el usuario, qué aprobó y si el pago cumple lo aprobado.

### 7.3 Descripción corta (hero de landing, primer párrafo del README)

> Cuando un agente de IA compra, el banco ve un pago válido, la tienda ve un pago válido, y nadie puede demostrar qué pidió realmente el usuario.
>
> intent-proof genera, por cada compra, un bundle firmado que registra la intención, la aprobación del titular con passkey, y si el cargo coincide con lo aprobado. Se verifica sin conexión, con solo claves públicas, ante un tercero que no confía en la plataforma que lo hizo.

### 7.4 Elevator pitch (30 segundos, hablado)

> Cuando un agente de IA compra algo, el banco ve un pago válido, la tienda ve un pago válido, y nadie puede demostrar qué pidió realmente el usuario. Un paper de seguridad (arXiv, sept-2026) mostró que los protocolos de pago agentico producen firmas válidas de compras que **no coinciden con lo pedido**: el ataque funcionó el 73,3% de las veces y los controles de binding rechazaron 0 de 68 carritos. Nosotros construimos la pieza que falta: un bundle firmado que registra qué se pidió, quién lo aprobó con su passkey, y si el cargo coincide con lo aprobado. Se verifica sin conexión, con solo claves públicas. Es JSON firmado: no te ata a ninguna plataforma.

### 7.5 Tres verbos (slide)

> **Declarar** la intención. **Aprobar** como titular. **Comprobar** que el cargo coincide.

### 7.6 Tres bullets (slide de "qué hacemos")

> - Un bundle firmado por compra
> - Verificable sin red, por terceros
> - JSON y claves públicas: sin lock-in

### 7.7 La definición (línea citable)

> Que ninguna compra de un agente exista sin la prueba de que un humano la pidió y la aprobó.

### 7.8 Qué NO somos (FAQ, reunión con inversor, último slide)

| No somos | Somos |
|---|---|
| Un sistema de pagos | Una capa de evidencia, arriba del checkout que sea |
| El checkout | Un integrador: Agentcard, Stripe, opencommerce, AEP |
| Identidad de agentes (KYA / TAP) | Verificación de **autorización**, no de identidad |
| Un control que previene | Una **prueba** que demuestra |
| La raíz de confianza | **El notario**: verificamos la cadena de otro (el L1 de Verifiable Intent lo firma un banco o una red) |

### 7.9 Estado real de compatibilidad (NO prometas lo que no está)

Verificado contra el código el 21-nov-2026. **El riesgo de sobre-prometer es nuestro, no del cliente final: una incompatibilidad que se descubre en la integración te quema la relación.**

| | Estado real | Gap |
|---|---|---|
| **RFC 8785 (JCS)** canonicalización | **Sí** — `canonical_json` | — |
| **Ed25519 / did:key** | **Sí** | — |
| **Verificación offline** (solo claves públicas, sin red) | **Sí** — `synsema run --cap-set "stdout,file.read"` | — |
| **AEP — cadena hash de 8 capas** | **No** | Hoy el bundle tiene 4 claves (`intent`, `proposal`, `signed_statement`, `human_assertion`), no la cadena `discovery→…→authorization→fulfillment` |
| **AEP — sello en el punto de authorization** | **Parcial** | Firmamos el statement; falta encadenar los artefactos y sellar ahí |
| **Verifiable Intent — SD-JWT / `cnf` / `sd_hash`** | **No** | Hoy es W3C VC + did:key |
| **Verifiable Intent — ES256** | **No** | Emitimos Ed25519. VI exige solo ES256; AP2 exige ECDSA en `checkout_jwt`. **Cambio chico y barato** (Fase 0, paso 2 del plan) |

**Frase honesta para una venta hoy:** *"Somos RFC 8785 + Ed25519 con verificación offline. Estamos
alineados a AEP: la cadena de 8 capas y el sello en authorization son trabajo de un trimestre."*
**No digas "compatible con AEP" ni "compatible con Verifiable Intent" hasta que los tests lo demuestren.**

---

## 8. Reglas de uso del post (y lo que NO se puede decir)

### 8.1 Los tres claims que sí podés hacer

| Claim | Fuente | Cómo citarlo |
|---|---|---|
| *"Los protocolos de pago agentico producen firmas válidas de compras que no coinciden con lo pedido"* | arXiv:2609.11757 (10-sep-2026) | "un paper de seguridad (arXiv, sept-2026)" |
| *"El ataque funcionó el 73,3% de las veces; los controles de binding rechazaron 0 de 68 carritos"* | arXiv:2609.11757 | Contexto: **en un benchmark**, sobre modelos que el sample de AP2 usa por defecto |
| *"La disputa 13.3 ('no era lo que pedí') no califica para CE3.0; la red no ofrece defensa"* | Visa Core Rules 18-abr-2026 §11.7.5.3/§11.7.5.6 + commentary legal | "CE3.0 es opcional y solo cubre 10.4" |

### 8.2 Los tres claims que NO podés hacer (yo los hice, y los corregí)

| ❌ No digas | ✅ Decí | Por qué |
|---|---|---|
| "El 23,1% de las compras de agentes está mal" | "En el benchmark de Visa (990 corridas, 22 modelos, TVs/laptops/appliances), el 42,8% de las corridas falló y en el 54% de esos fallos el agente compró igual" | El 23,1% es **derivado**, no publicado. Y el benchmark **no cubre ropa ni comida** — no es el universo |
| "Desde el 24-oct-2026 la evidencia es obligatoria" | "CE3.0 es **opcional** y solo aplica a 10.4" | *"An Acquirer **may** submit"*. Y el login ID del agente no lo obtiene el merchant |
| "Somos compatibles con AEP / Verifiable Intent" | "Somos RFC 8785 + Ed25519 con verificación offline; estamos alineados a AEP" | La cadena de 8 capas de AEP y el SD-JWT/ES256 de VI **no están implementados** (ver §7.9) |

### 8.3 Un solo post, tres audiencias

| Audiencia | Versión |
|---|---|
| **Tech / pagos** | §6.1 — arXiv + CE3.0 |
| **General** | §6.2 — el agente compró otra cosa, el banco te cree, nadie puede probar qué pediste |
| **Video / demo** | El guion del vuelo (§3 de THESIS.md) con el `REJECTED — 2 de 6` en pantalla |

---

## 8. Lo que la definición NO promete (guardá esto a mano)

- **No previene** la compra incorrecta. La hace **demostrable**. (PredictAP: *"an audit trail is a log, not
  a control"*.)
- **No es un sistema de pagos.** No mueve plata, no custodia tarjetas, no emite instrumentos.
- **No autentica al agente.** Eso lo hacen KYA/TAP. Verificamos **autorización**, no identidad.
- **No es el checkout.** Se integra arriba: Agentcard, Stripe, opencommerce, AEP, el que sea.
- **No es una plataforma.** Sin licencia bancaria no podés ser la raíz de confianza (VI exige que el emisor
  del L1 sea un banco o una red). Sos **el notario**, no el emisor. Decilo así, y es una posición defendible.
