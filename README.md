# 💫 Proyecto Soroban – Clase 3 | Código Futura 2025  

> **Contrato Inteligente “Hello Tiburona”** desarrollado en **Rust + Soroban SDK**, desplegado sobre la red de prueba **Stellar Testnet**.  
> Este proyecto forma parte del programa **Código Futura**, impulsado por **Buen Día Builders**, **BAF Network** y el **Stellar Community Fund**, con el propósito de formar desarrolladoras en tecnologías Web3 y blockchain en Latinoamérica.  

---

## 👩‍💻 Contexto  

Durante esta clase se exploraron los **fundamentos de Soroban**, el entorno de contratos inteligentes de Stellar.  
El objetivo fue aprender a construir, validar y ejecutar un **smart contract completo**, aplicando buenas prácticas de seguridad, manejo de errores, almacenamiento persistente y control de acceso.  

A través de la creación del contrato **“Hello Tiburona”**, se desarrollaron **tres retos adicionales** que simulan escenarios reales de producción.  

---

## 🎯 Objetivo  

Diseñar y desplegar un **contrato inteligente profesional** en Soroban, aplicando validaciones, autorización por dirección (`Address`), administración de estado (`Instance` y `Persistent Storage`) y una estructura modular preparada para entornos reales.  

---

## ⚙️ Tecnologías y Herramientas  

| Categoría | Herramientas |
|:-----------|:-------------|
| 🦀 Lenguaje | Rust |
| 🧩 Framework | Soroban SDK |
| 🧱 Entorno | Soroban CLI · Cargo · WASM32 |
| 🌐 Blockchain | Stellar Testnet |
| 💻 Editor | Cursor / Visual Studio Code |
| 🔧 Control de versiones | Git & GitHub |

---

## 🚀 Funcionalidades del Contrato  

| Tipo | Descripción |
|------|--------------|
| 🏗️ **initialize()** | Define el administrador inicial y configura el almacenamiento del contrato. |
| 💬 **hello()** | Valida el nombre, comprueba la autorización y guarda el último saludo. |
| 📊 **get_contador()** | Devuelve el número total de saludos registrados. |
| 👤 **get_contador_usuario()** | Devuelve cuántos saludos ha realizado cada usuario. |
| 🔐 **transfer_admin()** | Permite transferir el rol de administrador a otro usuario. |
| 🧹 **reset_contador()** | Reinicia el contador global (solo admin). |
| 📏 **set_limite()** | Permite establecer un límite máximo de caracteres configurables para los nombres. |

---

## 🧩 Retos Adicionales Implementados  

### 🔹 Reto 1 — Estadísticas por Usuario  
📈 Se agregó `ContadorPorUsuario(Address)` a `DataKey` para registrar cuántas veces saluda cada cuenta.  
✅ Nueva función `get_contador_usuario()` para consultar el contador individual.  
🔁 `hello()` fue modificado para incrementar y guardar el contador de cada usuario.  

---

### 🔹 Reto 2 — Transferencia de Administrador  
🔐 Se añadió la función `transfer_admin()` que verifica que el caller sea el admin actual antes de transferir el control a una nueva dirección.  
💡 Esto simula el traspaso seguro de poder en entornos descentralizados.  

---

### 🔹 Reto 3 — Límite de Longitud Configurable  
📏 Se implementó `LimiteCaracteres` como nueva clave en `DataKey`.  
🧠 El admin puede modificar el límite de caracteres permitido mediante `set_limite()`.  
🚫 `hello()` valida que el nombre no exceda ese límite dinámico (por defecto 32).  

---

🧠 Aprendizajes Clave

✨ Comprender la diferencia entre Instance y Persistent Storage.
✨ Manejar errores con #[contracterror] y Result.
✨ Validar inputs antes de modificar el estado del contrato.
✨ Aplicar control de acceso mediante Address.
✨ Configurar límites y parámetros dinámicos.
✨ Compilar y ejecutar contratos Soroban en la red Testnet.

🌊 Reflexión Final

“Este desafío marcó un antes y un después: no solo logré que el código funcionara, logré entenderlo, dominarlo y transformarlo en algo mío.
Hoy no solo compila, brilla.” 🌟

Construir este contrato fue más que un ejercicio técnico: fue un paso hacia el desarrollo profesional de soluciones blockchain reales, aplicando lógica, seguridad y diseño limpio dentro del ecosistema Stellar.

🌐 Dennise Alvarado

Código Futura 2025 · Buen Día Builders x Stellar Community Fund
✨ Building the future, one contract at a time.
