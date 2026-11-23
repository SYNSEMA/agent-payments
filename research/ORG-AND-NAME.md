# Synsema / Csibeings — Org, nombre y orden de operaciones

> 22-nov-2026. Decide la estructura de la que sale el open source de la evidencia de compras agenticas.
> Complementa [OPEN.md](OPEN.md) (qué se abre y qué no).

---

## 1. La pregunta: ¿org aparte o todo en `synsema`?

### Dónde vive el repo de Next.js

**`github.com/vercel/next.js`** — MIT, 143.603★, 32.733 forks, 3.955 releases, creado el 05-oct-2016.

Y la org `vercel` **no se llama `nextjs`**: se llama `vercel`, porque **vercel es la empresa**. La org es el
portafolio: adentro conviven `next.js`, `vercel-py`, `vercel-plugin`, `turbo`, `swc`, `ai`, `satori` — el
producto estrella y los productos secundarios.

**Tu modelo mental es exactamente ese, y es correcto:**

| Vercel | Vos |
|---|---|
| Empresa = Vercel Inc. | Empresa = **Csibeings** |
| Org GitHub = `vercel` | Org GitHub = **`synsema`** |
| Producto estrella = `next.js` (repo en la org) | Producto estrella = **`synsema`** (el lenguaje, repo en la org) |
| Producto secundario = `vercel-py`, `turbo`, `swc`… | Producto secundario = **`agentic-payments`**, `synsema-wasm`, el demo |
| Comercial = Vercel (la empresa de收费 SaaS) | Comercial = **el trust service** (nombre nuevo, bajo Csibeings) |

**No necesitás una org nueva. Y te corrijo algo que te dije antes:** dije "org neutral desde el día 1". Es
over-engineering para hoy. Una org con cero adopters es overhead (legal, banners, CLA, gobierno, mantenimiento)
y "neutral" sin implementadores no significa nada.

### Por qué tampoco hay que crear la fundación ahora

**El patrón que funcionó, siempre igual:** *el vendor lo construye en su propio repo, y lo dona a una fundación
cuando la cosa es más grande que él.*

| Caso | De dónde → dónde | Cuándo |
|---|---|---|
| gRPC | Google → CNCF | 2019 |
| Istio | Google + IBM → CNCF | 2022 |
| OpenTelemetry | Google/Microsoft → CNCF | 2019 |
| Kubernetes | Google → CNCF | 2015 |
| **Sigstore** | **Google → OpenSSF** | **2022** — y **Chainguard comprometió US$50M y 100 ingenieros** a Sigstore/SLSA/OpenSSF para sostenerlo |
| Terraform Registry | HashiCorp (el vendor queda como *one* implementer) | — |

**Sigstore es tu modelo exacto**: signatura verificable, gratis, open, neutral, y el negocio de la confianza vive
**arriba** (en tu caso: el trust service). Nadie le compra "firmar gratis"; le pagan la confianza.

Y el destino real de tu spec ya existe: **FIDO Alliance**. Google donó AP2 y Mastercard donó Verifiable Intent
(28-abr-2026) a un consorcio neutral de ~400 empresas, con el Payments TWG chaired por Mastercard y Visa.
**Tardaron 20 años. No es un movimiento de un día.**

### El detalle no obvio: el mecanismo legal tiene que permitir la donación futura

Esto es lo que separa "abrir un repo" de "construir un estándar donable":

| Mecanismo | Qué implica | Veredicto |
|---|---|---|
| **CLA unidireccional** ("asigno mis copyrights a la org") | La empresa queda dueña del código. Los legal de las empresas grandes lo odian | ❌ Google lo intentó con Chromium y **lo abandonó** |
| **Sin CLA** | Apache-2.0 pelado, máxima adopción | ❌ La donación futura a una fundación requiere el OK de cada contributor: pesadilla |
| **DCO (sign-off)** | Un `git commit -s`. Cada contributor certifica que tiene derecho a donar | ✅ **Es lo que usan el kernel de Linux, Kubernetes, CNCF y OpenSSF.** Leve, y hace la donación futura trivial |
| **CLA permisiva (inbound = outbound)** | Cada contributor conserva su copyright pero licencia al proyecto | ✅ También funciona |

> **Recomendación: DCO + Apache-2.0 + grant explícito de patente.** Es el patrón exacto de CNCF/OpenSSF y el
> que hace la donación futura un trámite y no un megaproyecto legal.

### La secuencia, con los disparadores

| Fase | Qué | Disparador |
|---|---|---|
| **0 — hoy** | El spec vive en **`synsema/agentic-payments`** (o `synsema get @synsema/agentic-payments`). Apache-2.0 + patent grant + DCO + `GOVERNANCE.md` | — |
| **1 — 6-12m** | Adopción real: stars fuera de tu red, dependientes npm, PRs de terceros, FIDO y AEPombiesando la implementación | Métrica concreta, ver §4 |
| **2 — 12-24m** | **El spec se dona a un steward neutral** (CNCF / OpenSSF / LF, o una fundación nueva de pagos). Synsema queda como **una** implementación de referencia más; vos quedás como maintainer del **programa de conformance** | **≥3 implementadores independientes fuera de Synsema** |
| **3 — 24m+** | El formato no tiene dueño. El negocio es la confianza sobre un estándar neutral | — |

**Y esto es también la defensa:** si Mastercard + Google extienden Verifiable Intent para cubrir el ítem
(ventana 12-18 meses), te come el topo **si el formato era tuyo**. Si el formato es neutral y vos sos el
maintainer de la referencia y del conformance program, **el topo se vuelve tu trabajo**.

### Cuándo el spec se va a su propia org

El patrón Microsoft: `microsoft/`, `dotnet/`, `PowerShell/`, `Azure/` — una empresa, muchas orgs, y cada
proyecto grande maduro se va a la suya. **`agentic-payments` se va a su propia org cuando sea un estándar
reconocido**, no antes.

---

## 2. El nombre: tres capas, no dos

| Capa | Nombre | Por qué |
|---|---|---|
| **Empresa / legal** | **CSIBEINGS LLC** (US) | Dueña del trust service (comercial) + del repo. Ver §2.5: hay una trampa fiscal con fecha |
| **Marca del lenguaje / portafolio OSS** | **SYNSEMA** | Ya lo tenés, ya suena, ya es el dominio, ya es el npm. **No lo renuncies.** El source ya es **Apache-2.0** |
| **Receta (deployable)** | **`agent-payments`** | El vector de distribución: *"deploy one, then make it yours"*. GitHub `synsema/agent-payments` libre; **npm `agent-payments` está ocupado** → usá `@synsema/agent-payments` o el canal `synsema get` |
| **Spec (normativo)** | vive **dentro** del repo de la receta, en `spec/` | Pero **sin dependencia de Synsema** (JSON plano + SD-JWT + verificador de referencia en TS y Rust). Es la regla que permite donar el repo a una fundación neutral después y republicar la receta |
| **Producto de confianza** (lo que se vende) | **Nombre nuevo** | Porque el buyer —un risk/disputes manager— **no compra un lenguaje**. Compra evidencia |

### El problema de pronunciación, resuelto

**El nombre legal puede ser impronunciable; la marca no.** Es exactamente el caso Vercel: "Vercel" es raro
dicho en voz alta, y a nadie le molesta, porque en voz alta decís "Next.js". Con vos: en voz alta decís
"Synsema" (el lenguaje), y el otro nombre **tiene que ser decirible** porque es lo que el risk manager va a
repetir en un call y en un contrato.

**No renombres la org `synsema`:** tiene 2 meses, npm `synsema`, `synsema.dev`, el `install.sh`, los release
assets en GitHub y `@synsema/wasm`. **Renombrar una org a los 2 meses es churn que rompe URLs por un beneficio
que todavía no existe.** La razón de negocio para renombrar sería: *"el lenguaje se volvió el estándar de la
industria y ya no puede ser el nombre de la empresa"* → entonces la empresa se llama Csibeings y la org queda
`synsema`. **El orden correcto es ese, y Csibeings ya está del lado correcto.**

### Candidatos para el producto de confianza

| Candidato | Lectura | Dominio a chequear |
|---|---|---|
| **Rúbrica** ⭐ | La palabra exacta: *la rúbrica es el sello que vuelve oficial un documento*. Y "rubric" en inglés. Un merchant que pierde una disputa 13.3 quiere "una rúbrica" | rubrica.ai / .com |
| **Aval** | Corto, "garantía". Poco claro qué es | — |
| **Sello** | "Seal". Genérico, difícil de registrar | — |
| **Testigo** | "Witness". Bonito pero raro en software | — |

**Criterio para elegir:** (a) se dice en español y en inglés sin explicación; (b) dice el producto (prueba,
atestación, garantía); (c) **no colisiona** con lo que ya está tomado (§ de NAMING.md); (d) dominio `.com` o
`.ai` libre.

### 2.5 🔴 CSIBEINGS LLC: tres decisiones y una trampa con fecha

**"LLC" es la forma legal; la elección fiscal es otra cosa.** Tres decisiones que hay que tomar
explícitamente, y una que hay que resolver con un contador **ahora**:

| # | Decisión | Por qué importa |
|---|---|---|
| **1** | **¿Disregarded entity o elect C-corp?** Un LLC single-member sin elección se trata como disregarded (todo pasa al dueño). Con elect de corporation, la LLC es taxpayer separada y paga 21% federal | Cambia toda la planificación. **No es un detalle que se pueda dejar en default sin consecuencias** |
| **2** | **¿Qué estado?** (Delaware, Wyoming, otro). Cada uno tiene su annual fee y su franchise tax. **Wyoming** es el estándar para LLCs pequeñas sin actividad; **Delaware** si pensás levantar ronda o tener investors/US counsel | Los annual fees se pagan aunque no tengas revenue |
| **3** | **¿Elected corporate income tax en New York?** Si el LLC tiene sede en NY, la elección de impuesto corporativo de NY es casi obligatoria en la práctica. **Wyoming o Delaware evitan eso** | Es la razón por la que la mayoría elige Delaware o Wyoming |

#### La trampa: **Form 5472** — y por qué el ITIN no es un problema

Tu LLC de Wyoming con dueño en Argentina es, para el IRS, una **"foreign-owned U.S. DE"** (entidad
ignorada con dueño foráneo). Tiene una obligación que casi nadie descubre:

> Un **Form 5472** + un **Form 1120 pro forma** por **toda transacción reportable** con el dueño foráneo.
> Y **las contribuciones de capital y las distribuciones SON reportables.** O sea: **la acción de poner plata
> en el LLC es lo que dispara la obligación.**
> Vencimiento **15 de abril** (con **Form 7004**, form code 12, se extiende a **15 de octubre**).
> **Penalización: US$25.000 por formulario por año**, y un formulario aparte por cada parte relacionada.
> **No se puede e-filear**: fax a 300 DPI+ al **855-887-7737**, o correo a *IRS, 1973 Rulon White Blvd,
> M/S 6112 Attn: PIN Unit, Ogden, UT 84201*. Se firma el 1120 pro forma **en tinta** (el 5472 no tiene línea
> de firma). Guardás el reporte de transmisión.
> Fuentes: IRS Instructions for Form 5472 (Rev. Dic-2024); commentary legal y preparadores.

**🟢 El ITIN no hace falta. Verificado, textual de las instrucciones:**

> *"4b(1) U.S. identifying number, if any. An SSN or ITIN **if you have one**; most non-resident owners do not,
> **and that is fine**. **Do not apply for an ITIN just for this line.**"*

Lo que va en su lugar:

| Campo | Qué ponés |
|---|---|
| **4b(1)** US identifying number | **Vacío.** No tenés, no lo consigas |
| **4b(2)** Reference ID number | **Uno que inventás vos**: alfanumérico, sin espacios ni caracteres especiales, máx. 50 caracteres, **el mismo todos los años**. Sin trámite en el IRS |
| **4b(3)** FTIN | Tu **CUIT** (es un FTIN legítimo) o **"None"** |
| **4c/4d/4e** | País principal donde hacés negocio · nacionalidad · país donde presentás declaración de renta como residente (Argentina) |
| **1b** (LLC) | **Tu EIN** ← ya lo tenés |
| **Item B** del 1120 pro forma | **Tu EIN** |

**🟢 Tu EIN es exactamente lo que se necesita** (línea 1b del 5472 e Item B del 1120). Y Wyoming fue buena
elección: sin elected corporate income tax de New York, que en la práctica es obligatoria si la LLC tiene
sede ahí. Recordá: **Wyoming annual report, u$s60 al año** — pagálo, que se olvida.

**🔴 Y lo que NO tenés que hacer (y es lo más importante):**

1. **No pidas ITIN.** No lo necesitás para el 5472, y pedirlo "porque soy extranjero" no funciona: el ITIN
   requiere o una obligación de presentación o un status que acredite. **Perdés tiempo.**
2. **No elijas C-corp.** La elección a impuestos corporativos **no se puede deshacer por 5 años**. Sin
   revenue, **dejalo como disregarded entity**: es más simple y no pagás 21% federal ni una segunda
   declaración. Si algún día necesitás C-corp (ronda con investors, por ejemplo), ahí se evalúa.

**Lo que sí tenés que hacer (esta semana):**

| | |
|---|---|
| 1 | Confirmar **si hubo contribución de capital** (casi seguro) → eso es la transacción reportable. Y si hubo **pagos** del LLC a vos o de vos al LLC, también |
| 2 | Elegir tu **Reference ID** y no cambiarlo nunca (ej. `CSIBEINGS-OWNER-01`) |
| 3 | Preparar el paquete: **5472 + 1120 pro forma** ("Foreign-owned U.S. DE" escrito arriba, solo nombre/dirección + Item B y E), firmado en tinta, **fax 300 DPI+** a 855-887-7737, y **guardar el reporte de transmisión** |
| 4 | **Un contador US o un servicio de preparación** — esto es acotado y de una sola vez. Cuesta lo que cueste un par de horas de contador y te ahorra US$25.000 por año de penalización. No lo hagas solo |

**Y la pregunta de fondo:** ¿necesitás el LLC **activo**? Sin revenue y sin actividad en US, la alternativa
es **disolverlo** (Certificate of Dissolution en Wyoming) y cortar con la obligación. Pero eso te borra el EIN
y la entidad, y hay que rehacerla cuando haya ingreso. **Decisión para el contador, no para mí** — y el
argumento para **mantenerlo**: (a) es el vehículo natural para **cobrar en USD** (Stripe, chargebacks, y
un sponsorship de OSS en USD con IBAN) sin abrir una cuenta comercial en tu nombre; (b) **ring-fencing**:
el contrato con un cliente global del trust service queda en una persona jurídica que no es tu nombre; y
(c) **credibilidad** — en un estándar de pagos, que la empresa detrás sea una US LLC importa.

#### ¿Necesitás una persona jurídica US ahora?

**Lo que te compra** (todo real, todo útil):
- Stripe y chargebacks en USD, sin fricción de onboarding.
- **GitHub Sponsors / OpenCollective** pagan en USD a una entidad con IBAN — la vía más limpia para financiar un
  proyecto OSS, y es justo lo que tu misión necesita.
- Incorporación sin notario, y **separación entre la marca OSS (Synsema) y la entidad comercial**.
- Credibilidad internacional, que en un estándar de pagos importa.

**Lo que te cuesta:** la penalización de US$25.000 por formulario por año, franchise tax del estado, y un
contador US.

**Mi recomendación:** **no llenes el LLC de plata hasta hablar con el contador.** Si el Form 5472 ya está
disparado por la contribución inicial, hacelo ahora — es barato a tiempo y carísimo tarde. Y si la estructura
no termina saliendo a cuenta, **la entidad local (S.A.S.) sigue siendo la opción para el trust service**, que
es donde va el revenue real; el LLC US sirve para el OSS.

---

## 3. Dónde vive cada cosa (el mapa)

```
Csibeings  ── empresa legal. Dueña del trust service (comercial).
   │
   ├── Synsema (org GitHub, MIT)  ── el portafolio OSS
   │     ├── synsema                     el lenguaje
   │     ├── synsema-wasm                el runtime embebido
   │     └── agentic-payments            ← EL SPEC + referencia + tests  [Apache-2.0 + patent grant + DCO]
   │
   └── <nombre nuevo>  ── el trust service: root de custodia, evidence vault,
                          attestation de conformance, exports de compliance 13.3
                          [PROPIETARIO — repo/servicio cerrado, sin nombre público hasta tener cliente]
```

**El trust service NO va dentro del repo MIT del lenguaje.** Dos razones:
1. Si la confianza vive en la marca Synsema, Synsema deja de ser "abierto y gratis" — y esa promesa es
   justamente tu distribución.
2. Son audiencias distintas: un dev no quiere un contrato de custodia de claves, y un controller no quiere
   instalar un lenguaje.

---

## 4. El orden de las cosas

### Fase 0 — Estructura y nombres (esta semana, u$s0)

| | |
|---|---|
| 1 | Confirmar: **Csibeings** = empresa, **Synsema** = marca del lenguaje + org (sin tocar) |
| 2 | Elegir el nombre del producto de confianza (criterios de §2) y **no anunciarlo** hasta tener cliente |
| 3 | Escribir el `GOVERNANCE.md` y el `PATENTS.md` del spec **antes** del primer commit público |
| 4 | Decidir **DCO** (sign-off) para el repo del spec |

### Fase 1 — El repo (semanas 1-4, u$s0)

| | |
|---|---|
| 5 | `synsema/agentic-payments`: `spec/` + `reference/` + `testvectors/` + `demo/flight.syn` |
| 6 | `LICENSE` (Apache-2.0) + `NOTICE` + `PATENTS.md` (grant explícito, como AEP) + `GOVERNANCE.md` + `DCO` |
| 7 | Reescribir el bundle a **AEP-shaped**: 8 capas, `previous_hash` **dentro** del hash input, sello Ed25519 en `authorization`, export `DisputeBundle`, `chain_complete` |
| 8 | **Verificador en ≥2 lenguajes** (Rust + TS) con salida idéntica — **portable, sin Synsema** |
| 9 | CI que corra **AP2-WhisperBench** (1.544 escenarios) + los 291 casos de mutación |
| 10 | **PR al repo `ar-agents`** — su Phase 2 pide textualmente el AP2 mandate verifier |

### Fase 2 — Distribución (semanas 4-12, u$s0)

| | |
|---|---|
| 11 | Post en X con el demo del vuelo (`REJECTED — 2 de 6`) + un blog post técnico usando el arXiv como gancho |
| 12 | **FIDO Payments TWG** y **AEP**: "implementación de referencia, portable, con benchmark de 1.544 escenarios" |
| 13 | Medir: stars fuera de tu red, dependientes npm, PRs de terceros, implementaciones |

### Fase 3 — El negocio (meses 3-6)

| | |
|---|---|
| 14 | Trust service privado, sin nombre público. Root de custodia, evidence vault, attestation, exports de compliance |
| 15 | **15 llamadas.** El pitch cambia: *"adoptá el primitivo gratis; la confianza y el compliance se cobran"* |
| 16 | Programa de conformance: certificación de implementación |

### Fase 4 — Neutralidad (meses 12-24)

| | |
|---|---|
| 17 | Con **≥3 implementadores independientes fuera de Synsema** → donar el spec a un steward neutral |
| 18 | Synsema queda como una implementación de referencia más; vos, como maintainer del conformance program |

---

## 5. El riesgo grande de esta estrategia (sin endulzar)

**Un repo MIT de un lenguaje con 43 descargas semanales de npm no es un canal de distribución todavía.**
Synsema tiene ~2 meses, y 1-2 meses desde que tiene la potencia para decir "esto es un lenguaje". Estás
**apostando a que Synsema crece** y arrastra al estándar con él.

**Mitigación (no negociable): el spec tiene que ser usable SIN Synsema.**
- Formato: JSON plano + SD-JWT. Referencia en TS y Python.
- Un dev tiene que poder adoptarlo **sin instalar Synsema**.
- Synsema es **la implementación de primer nivel**, no el único camino.

> La diferencia entre *"el lenguaje tiene un módulo de pagos"* (pobre) y *"hay un estándar de pagos que Synsema
> implementa primero"* (rico) **es exactamente esa portabilidad.**

Y el otro riesgo, ya nombrado: **la licencia del core no cambia nunca.** El error HashiCorp/OpenTofu es el
único escenario que mata esta estrategia, y se previene con una línea en `GOVERNANCE.md`.

---

## 6. Los patrones, en una tabla

| Patrón | Ejemplo | Qué te enseñó |
|---|---|---|
| **Org = empresa, producto estrella = un repo** | `vercel/next.js` (143.603★) + `vercel-py`, `turbo`, `swc`, `ai` | **Tu caso exacto.** Synsema = org, el lenguaje = repo estrella, agentic-payments = otro repo. **No necesitás org nueva** |
| **Empresa con muchas orgs** | Microsoft: `microsoft/`, `dotnet/`, `PowerShell/`, `Azure/` | El destino de `agentic-payments` cuando madure |
| **Vendor-neutral, gratis, sostenido por sponsors** | **Sigstore** → OpenSSF (Google donó en 2022; **Chainguard: US$50M y 100 ingenieros**) | **El modelo exacto que proponés.** El formato es gratis y neutral; la confianza se cobra arriba |
| **Vendor construye, la fundación adopta cuando es más grande que el vendor** | gRPC, Istio, OpenTelemetry, Kubernetes → CNCF; Sigstore → OpenSSF | **No se crea una fundación el día 1.** Se dona cuando hay implementers de otros |
| **Neutralidad de industria, de verdad** | **FIDO Alliance** — Google donó AP2 y Mastercard donó VI (abr-2026); ~400 miembros | Tu destino. **Tardaron 20 años** |
| **Legal raro, marca renowned** | Vercel Inc. / Next.js · HashiCorp / Terraform · Automattic / WordPress | **El legal puede ser impronunciable; la marca no.** Tenés Synsema |
| **El contraejemplo** | HashiCorp: BSL → propietario → fork (OpenTofu) → pérdida del proyecto | **La licencia del core no cambia nunca.** Lo único innegociable |
| **El mecanismo de donación** | Linux kernel, Kubernetes, CNCF, OpenSSF: **DCO (sign-off)** | Lo que hace la donación futura trivial. Usá DCO |

---

## 7. La frase

> **Pedir, autorizar y cobrar con un agente debería ser auditable por cualquiera.**
> El formato es libre y siempre lo va a ser. La confianza es un negocio — pero no tiene por qué ser un monopolio.

*Relacionado: [OPEN.md](OPEN.md) · [THESIS.md](THESIS.md) · [NAMING.md](NAMING.md) · [POSITIONING.md](POSITIONING.md)*
