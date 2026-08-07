# Zamani Type System

## Primitive Types

- `Int` — 64-bit signed integer
- `Float` — 64-bit IEEE 754 float
- `Bool` — boolean (true/false)
- `Str` — UTF-8 string
- `Void` — unit/void return type
- `Never` — bottom type (diverging functions)

## Composite Types

- `Array<T>` — fixed-size homogeneous array
- `List<T>` — dynamic list
- `Map<K, V>` — hash map
- `Tuple<T...>` — heterogeneous tuple
- `Option<T>` — nullable/optional value
- `Result<T, E>` — fallible computation

## Advanced Types

- `Π(x:T) U` — dependent function (Pi type)
- `Σ(x:T) U` — dependent pair (Sigma type)
- `Id(T, a, b)` — identity/equality type
- `linear T` — linear type (must be used exactly once)
- `affine T` — affine type (used at most once)
- `Qubit` — quantum bit
- `QReg[N]` — N-qubit quantum register
- `Superposition<T>` — superposed value
- `Entangled<A, B>` — entangled pair

## Type Inference

Zamani uses bidirectional type inference. Type annotations are optional for local bindings but required for function signatures.

## Subtyping

Zamani uses structural subtyping for interfaces and nominal subtyping for classes.
