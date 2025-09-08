# Set Server

El ejercicio de la clase de hoy consiste en la implementación de una aplicación cliente-servidor utilizando sockets.

## Comportamiento del programa

El servidor se comporta como un set centralizado, que permite a múltiples clientes conectarse simultáneamente y realizar operaciones sobre un conjunto compartido. Cada operación modifica o consulta el estado actual del servidor.

## Servidor

El servidor debe contar con las siguientes características:

- Mantener un conjunto (Set) de elementos en memoria
- Aceptar conexiones concurrentes de múltiples clientes
  - Manejar a cada cliente en un hilo separado.
- Procesar las siguientes operaciones:
    - **INSERT <number>**: Agregar un elemento al conjunto
    - **REMOVE <number>**: Eliminar un elemento del conjunto
    - **CONTAINS <number>**: Verificar si un elemento existe en el conjunto
    - **GET**: Obtener todos los elementos del conjunto
- Enviar respuestas apropiadas a cada cliente indicando el resultado de la operación
- Manejar la concurrencia de forma segura para evitar condiciones de carrera

El servidor se debe poder levantar con el siguiente comando:

```bash
cargo run --bin server -- <puerto>
```

### Cliente

El cliente se debe poder ejecutar de la siguiente forma:

```bash
cargo run --bin client -- <host:puerto> <operacion>
```

Las operaciones se pueden pasar entre comillas dobles o simples, por ejemplo:

```bash
cargo run --bin client -- "localhost:3000" "INSERT 42"
```

## Protocolo de comunicación

- El protocolo seguirá la siguiente forma, en notación BNF:

```
<mensaje-cliente> ::= <operacion> <eom>
<operacion> ::= <insert> | <remove> | <contains> | <get>

<insert> ::= "INSERT" <ws> <numero>
<remove> ::= "REMOVE" <ws> <numero>
<contains> ::= "CONTAINS" <ws> <numero>
<get> ::= "GET"

<numero> ::= número de 8 bits (0-255)
<ws> ::= whitespace
<eom> ::= "\n"
```
