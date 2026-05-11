
# Zenith Package Specification (`.zpkg`)

This document outlines the conceptual structure and contents of a Zenith Package (`.zpkg`) file. A `.zpkg` is the standard distribution format for Zenith libraries, applications, quantum circuits, nano-agent blueprints, and MTS simulations. It is a self-contained archive designed for easy distribution, installation, and dependency management by the `zenith-pkg` toolchain component.

## 1. Archive Format

A `.zpkg` file is conceptually a compressed archive (e.g., `.zip` or `.tar.gz`) containing a well-defined directory structure. The choice of compression format allows for efficient transfer and storage of project assets.

## 2. Top-Level Directory Structure

Upon extraction, a `.zpkg` archive will always contain a single top-level directory named after the package (e.g., `my_package-1.0.0/`). Inside this directory, the following structure is expected:

```
my_package-1.0.0/
├── Zenith.toml                  # Project manifest (REQUIRED)
├── src/                         # Source code
│   ├── main.zn
│   ├── lib.zn
│   ├── quantum/
│   │   └── circuit_lib.zq       # Quantum source files
│   ├── nano/
│   │   └── agent_blueprint.na   # Nano-agent blueprint files
│   └── ...
├── target/                      # Compiled artifacts (OPTIONAL, can be built on installation)
│   ├── nimbus-vm/               # Nimbus OS bytecode/native
│   │   └── my_package.bin
│   ├── x86_64/
│   │   └── debug/
│   │       └── my_package
│   ├── qasm/                    # Quantum Assembly Language (e.g., for IBM Qiskit)
│   │   └── quantum_circuits.qasm
│   ├── nanoasm/                 # Nano-Agent Assembly/Control Sequences
│   │   └── agent_control_code.nas
│   └── mts_bytecode/            # Multi-Timeline System specific bytecode
│       └── simulation_logic.mtsb
├── docs/                        # Generated documentation (OPTIONAL)
│   ├── index.html
│   └── ...
├── licenses/                    # License files for the package and its dependencies (OPTIONAL)
│   ├── LICENSE-APACHE
│   └── LICENSE-MIT
├── Sankofa_Metadata/            # Conceptual: Specific metadata for Sankofa memory integration (OPTIONAL)
│   ├── knowledge_schemas.json   # JSON schemas for Sasa knowledge
│   └── facts_preloaded.zam      # Pre-packaged Zamani facts
├── Nimbus_OS_Bindings/          # Conceptual: Low-level bindings or drivers for Nimbus OS (OPTIONAL)
│   └── nimbus_api_v1.zn         # Zenith declarations for Nimbus system calls
└── README.md                    # General package information (OPTIONAL)
```

## 3. Key Components Explained

### 3.1. `Zenith.toml` (REQUIRED)

This is the central manifest file, identical to the `Zenith.toml` described in the project's build specification. It contains:
*   `[package]` metadata (name, version, authors, description, license, etc.).
*   `[dependencies]` specifying other `.zpkg` files required by this package.
*   `[features]` defining conditional compilation flags.
*   `[build]` configurations for target platforms, optimization levels, etc.
*   `[nimbus.os]` and `[sankofa.memory]` specific configurations.

### 3.2. `src/` (REQUIRED for source-based packages)

Contains all source files necessary to build the package. Zenith supports various file extensions:
*   `.zn`: Standard Zenith source code.
*   `.zq`: Zenith Quantum language extensions (e.g., explicit quantum circuit definitions).
*   `.na`: Nano-Agent blueprint definitions or specific control logic.
*   `.mts`: Multi-Timeline System configuration or timeline interaction scripts.
*   Other files relevant to the project (e.g., `.cfg`, `.json` for data definitions).

### 3.3. `target/` (OPTIONAL for distributed `.zpkg`, but common for local builds)

This directory holds the output of the compilation process for various target platforms. A `.zpkg` intended for distribution might exclude this directory if it's meant to be built from source by the consumer. However, pre-compiled `.zpkg` files would include this.

### 3.4. `Sankofa_Metadata/` (OPTIONAL)

For packages that are designed to interact heavily with Sankofa memory, this directory can include:
*   **Knowledge Schemas:** Definitions of the structure and types of Sasa knowledge items that the package can produce or consume.
*   **Pre-loaded Facts:** Immutable facts that are essential for the package's operation, to be recorded in Zamani memory upon installation.

### 3.5. `Nimbus_OS_Bindings/` (OPTIONAL)

If a package requires or exposes low-level interactions with the Nimbus OS microkernel or specific hardware abstractions, relevant bindings or interface definitions would reside here. This facilitates direct access to Nimbus's unique capabilities like secure communication channels or advanced scheduling policies.

## 4. `zenith-pkg` Interaction

The `zenith-pkg` toolchain component will be responsible for:
*   **Creating `.zpkg` files:** Bundling a project's assets into the specified archive format.
*   **Extracting `.zpkg` files:** Unpacking packages and validating their structure.
*   **Resolving dependencies:** Reading `Zenith.toml` to fetch and install required packages.
*   **Building packages:** Invoking the Zenith compiler (`zenithc`) with configurations from `Zenith.toml`.
*   **Publishing packages:** Uploading `.zpkg` files to a central Zenith package registry.
