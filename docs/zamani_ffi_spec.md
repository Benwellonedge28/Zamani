
# Zamani Foreign Function Interface (FFI) Specification

This document outlines the conceptual design and usage of Zamani's Foreign Function Interface (FFI). The Zamani FFI is a cornerstone of its "Universal Meta-Compiler" vision, enabling seamless and safe interoperability with existing codebases, operating system APIs (beyond Nimbus's microkernel), hardware SDKs, and libraries written in other programming languages (e.g., C/C++, Rust, Python).

## 1. Goals of Zamani FFI

*   **Interoperability:** Allow Zamani code to call functions and use data structures defined in external libraries.
*   **Hardware Integration:** Provide direct access to vendor-specific hardware SDKs (e.g., QPU drivers, nano-agent control interfaces).
*   **Leverage Existing Ecosystems:** Avoid reinventing the wheel by integrating with mature libraries from other languages.
*   **Performance:** Enable zero-cost abstractions where possible, with minimal FFI overhead.
*   **Safety & Trustworthiness:** While FFI is inherently "unsafe," Zamani's FFI aims to provide mechanisms for verification and controlled access, especially when integrated with Nimbus OS.

## 2. Basic FFI Declaration Syntax

Zamani FFI declarations resemble external function or type declarations, specifying the foreign ABI and mapping.

### 2.1. `extern` Block

The `extern` keyword introduces a block for declaring foreign functions, variables, or types. The ABI (Application Binary Interface) is specified as a string literal.

```zamani
extern "C" {
    fn puts(s: *const char) -> int;
    fn malloc(size: u64) -> *mut void;
    fn free(ptr: *mut void);
}

extern "rust" {
    fn rust_add(a: i32, b: i32) -> i32;
    type RustString = string; // Mapping Rust's String to Zamani's string
}

extern "python" {
    fn python_greet(name: string);
    fn python_calc_sum(data: List<i32>) -> i32;
}

extern "qpu_ibm_qiskit" {
    fn qiskit_run_circuit(qc_handle: QCircuit, shots: i32) -> QResult;
}

extern "nano_hw_v1" {
    fn nano_motor_set_speed(agent_id: nano.AgentId, speed: float);
}
```

### 2.2. ABI Specifications

*   **`"C"`:** Standard C calling convention and data layout. Most common for OS APIs and low-level libraries.
*   **`"rust"`:** Rust's FFI-safe ABI (e.g., `#[no_mangle] extern "C"` functions). Zamani's compiler can generate specific bindings for Rust.
*   **`"python"`:** For interoperating with Python interpreters. This implies a higher-level binding layer managing GIL and Python object conversions.
*   **`"qpu_vendor_api"`:** (e.g., `"qpu_ibm_qiskit"`, `"qpu_google_cirq"`) Specific ABIs tailored for direct interaction with Quantum Processing Unit (QPU) vendor SDKs, handling complex quantum data types.
*   **`"nano_hw_api"`:** (e.g., `"nano_hw_v1"`) For direct communication with nano-agent hardware interfaces, potentially involving specialized packet formats or shared memory protocols.
*   **Custom ABI:** Zamani's meta-compiler can potentially define and target custom ABIs for highly specialized scenarios.

## 3. Type Mapping

Zamani's FFI provides conceptual mechanisms for mapping its rich type system to foreign types.

| Zamani Type                 | C Type (conceptual)          | Rust Type (conceptual)       | Python Type (conceptual)     | QPU/Nano Type (conceptual)      |
| :-------------------------- | :--------------------------- | :--------------------------- | :--------------------------- | :------------------------------ |
| `bool`                      | `_Bool` (or `int`)           | `bool`                       | `bool`                       | `bool`                          |
| `char`                      | `char`                       | `char`                       | `str` (single char)          | `char`                          |
| `i8, i16, i32, i64`         | `int8_t, int16_t, ...`       | `i8, i16, i32, i64`          | `int`                        | `int`                           |
| `u8, u16, u32, u64`         | `uint8_t, uint16_t, ...`     | `u8, u16, u32, u64`          | `int`                        | `uint`                          |
| `float, double`             | `float, double`              | `f32, f64`                   | `float`                      | `float`                         |
| `string`                    | `*const char` (C string)     | `&str` or `String`           | `str`                        | `string`                        |
| `List<T>`                   | `T*` (C array) + `size_t`    | `&[T]` or `Vec<T>`           | `list`                       | Specialized array/vector        |
| `Struct { ... }`            | `struct { ... }`             | `struct { ... }`             | `dict` (or custom object)    | Specialized struct              |
| `*mut T`, `*const T`        | `T*`, `const T*`             | `*mut T`, `*const T`         | `ctypes.POINTER(T)`          | Raw hardware address            |
| `Qubit`                     | `q_handle_t` (opaque handle) | `QubitHandle`                | `QubitObject`                | Hardware qubit ID/reference     |
| `NanoAgent`                 | `nano_id_t` (opaque ID)      | `NanoAgentId`                | `NanoAgentObject`            | Hardware agent ID/reference     |
| `MtsSlice<T>`               | `mts_handle_t`               | `MtsHandle`                  | `MtsObject`                  | `TimelineID` + snapshot pointer |
| `Effectful fn ...`          | (Not directly mappable)      | (Not directly mappable)      | (Not directly mappable)      | (Not directly mappable)         |

**Pointers:** Zamani distinguishes between raw pointers (`*mut T`, `*const T`) and safe references. FFI operations heavily rely on raw pointers for direct memory access.

**Complex Types:** For complex types like `List<T>` or custom `Struct`s, Zamani's FFI compiler automatically generates marshalling code to convert between Zamani's internal representation and the foreign ABI's layout.

## 4. Calling Conventions

The `extern` block can optionally specify a calling convention for traditional ABIs:

*   **`"C"` (default for C FFI):** Standard C calling convention (`cdecl` on x86, platform-specific for others).
*   **`"stdcall"` (Windows API):** For Windows API calls.
*   **`"Zamani"`:** Zamani's native calling convention (highly optimized, often inlined, may involve passing contexts or capabilities).

```zamani
extern "C", calling_convention = "stdcall" {
    fn MessageBoxA(hWnd: int, lpText: *const char, lpCaption: *const char, uType: u32) -> int;
}
```

## 5. Error Handling Across FFI Boundaries

FFI calls can fail. Zamani provides mechanisms to handle foreign errors:

*   **Return Codes:** Standard in C. Zamani FFI can automatically translate C-style return codes into Zamani's `Result<T, E>` or `Option<T>` types.
*   **Exceptions/Panics:** When interfacing with languages that use exceptions (e.g., C++ exceptions, Rust panics, Python exceptions), Zamani FFI generates conceptual wrappers to catch these and convert them into Zamani's `Result<T, E>` or raise specific Zamani effects.
*   **Zamani Effects:** A foreign function can conceptually `perform` a Zamani effect if it encounters a specific error condition. This allows Zamani's effect handlers to manage FFI failures.

## 6. Resource Management and Ownership

Managing memory and other resources across FFI boundaries is critical for safety and preventing leaks.

*   **Manual Management:** For `extern "C"` functions, Zamani defaults to manual memory management. If a foreign function returns a pointer to memory it allocated, Zamani requires explicit `free` calls (or custom deallocators).
*   **Linear/Affine Types:** Zamani's FFI can extend linear/affine type tracking across boundaries. For instance, a linear type passed to a C function implies the C function "consumes" ownership, and Zamani's runtime ensures no further use.
*   **Garbage Collection Integration:** For Python FFI, Zamani's GC might interact with Python's reference counting to ensure proper object lifetime management.
*   **Nimbus OS Secure Regions:** FFI calls accessing memory via Nimbus OS `secure_alloc` or shared memory regions (`allocate_shared_memory`) would have their ownership and access policies managed by the Nimbus microkernel.

## 7. Safety and `unsafe` Blocks

FFI operations are generally considered `unsafe` in Zamani because the compiler cannot guarantee memory safety or type correctness across foreign boundaries.

*   **`unsafe` Keyword:** All direct FFI calls within Zamani must be wrapped in an `unsafe` block.
    ```zamani
    fn main() {
        unsafe {
            let message = "Hello from Zamani FFI!\0"; // C string requires null terminator
            extern "C" { fn puts(s: *const char) -> int; }
            puts(message as *const char);
        }
    }
    ```
*   **Formal Verification Integration:** For critical FFI bindings, Zamani's formal verification tools can be used to prove properties about the safety and correctness of the interaction, even when the foreign code itself is not formally verified. `Zamani.toml` can specify verification policies for FFI modules.

## 8. Example: Calling a Rust Library

A Rust library might expose:

```rust
// Rust: my_rust_lib/src/lib.rs
#[no_mangle]
pub extern "C" fn process_data(data: *mut u8, len: usize) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts_mut(data, len) };
    // Process data...
    0 // Success
}
```

Zamani code would call this:

```zamani
// Zamani: main.zn
extern "C" { // Even if Rust, C ABI is often used for FFI
    fn process_data(data: *mut u8, len: u64) -> i32;
}

fn main() {
    let mut my_data: List<u8> = List::new();
    my_data.push(1);
    my_data.push(2);
    my_data.push(3);

    unsafe {
        // Conceptual: Get raw pointer and length from Zamani List
        let data_ptr = my_data.as_mut_ptr(); // Assumes List has as_mut_ptr method
        let data_len = my_data.len();
        let result = process_data(data_ptr, data_len as u64); // Cast len to u64
        if result != 0 {
            stdlib.core.println("Error processing data in Rust!");
        }
    }
}
```

This conceptual specification provides a detailed vision for Zamani's FFI, enabling its universal interoperability goals.
