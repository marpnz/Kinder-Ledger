# 🏠 Kindred Ledger: Inmutable Child Welfare Records

Kindred Ledger es una solución descentralizada construida en **Solana** diseñada para casas hogar y centros de asistencia social. Su objetivo es proporcionar un registro digital permanente, seguro e inalterable de los menores bajo cuidado, eliminando el riesgo de pérdida de documentos físicos y garantizando la transparencia administrativa.

---

## 🌟 Visión del Proyecto
A diferencia de los sistemas comerciales, Kindred Ledger se basa en el principio de **preservación histórica**. En la blockchain, la información es inmutable, lo que asegura que:
* **Sin Borrado:** No existe una función para eliminar expedientes; la historia del niño permanece siempre.
* **Trazabilidad:** Registro automático del administrador responsable y la fecha exacta de ingreso.
* **Seguridad:** Los expedientes están protegidos por PDAs (Program Derived Addresses) vinculadas al ID del menor.

---

## 🏗️ Arquitectura de Datos (Account Space)

Cada expediente se almacena en una cuenta con la siguiente estructura técnica:

| Campo | Tipo | Tamaño (Bytes) | Descripción |
| :--- | :--- | :--- | :--- |
| `admin` | Pubkey | 32 | Administrador que creó el registro. |
| `id_expediente` | String | 24 | Identificador único (ej: "EXP-2026-001"). |
| `name` | String | 44 | Nombre completo del menor. |
| `blood_type` | String | 8 | Tipo de sangre (O+, AB-, etc.). |
| `medical_notes` | String | 204 | Historial médico y observaciones. |
| `age` | u8 | 1 | Edad actual (actualizable). |
| `admission_date`| i64 | 8 | Timestamp de entrada (Unix). |
| `exit_date` | i64 | 8 | Timestamp de salida (0 si está activo). |
| `is_active` | bool | 1 | Estado actual en la institución. |

---

## 🛠️ Instrucciones del Programa

### 1. `register_child`
Crea el expediente inicial. Se requiere el ID único, nombre, edad, tipo de sangre y notas médicas iniciales. La fecha de admisión se genera automáticamente usando el reloj de la red.

### 2. `update_vitals`
Permite al administrador actualizar la edad y las notas conforme el menor crece o recibe nuevos diagnósticos. Solo el administrador original tiene permiso.

### 3. `register_exit`
Registra el egreso formal del niño. Marca el expediente como inactivo (`is_active: false`) y registra la fecha de salida. Una vez ejecutado, el expediente queda bloqueado para futuras modificaciones para preservar la integridad del historial final.

---

## 🚀 Cómo empezar (Solana Playground)

1. **Compilación:** Pega el código en `lib.rs` y presiona el botón de **Build**.
2. **Despliegue:** Ve a la pestaña **Deploy** y confirma la transacción en la red de prueba (Devnet).
3. **Pruebas:** - Usa `register_child` con un ID único como "CASA-101".
   - Verifica la creación en el explorador de Solana.
   - Intenta actualizar los datos con `update_vitals`.

---

## 🔐 Seguridad y Errores
El programa incluye validaciones para evitar que personas no autorizadas modifiquen los datos (`UnauthorizedAccess`) y para asegurar que no se editen expedientes de niños que ya han egresado de la institución (`RecordAlreadyInactive`).
