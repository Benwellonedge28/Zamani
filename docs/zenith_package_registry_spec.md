
# Zenith Package Registry Specification

This document outlines the conceptual design and API for the official Zenith Package Registry. The registry serves as a central repository for discovering, sharing, and distributing `.zpkg` (Zenith Package) files. It is an integral part of the Zenith ecosystem, working in conjunction with the `zenith-pkg` package manager.

## 1. Registry Principles

*   **Trust and Verification:** Packages undergo automated formal verification and Nimbus OS sandbox policy checks upon submission.
*   **Multi-Paradigm Support:** Designed to host packages for classical, quantum, nano-agent, MTS, and Sankofa domains, with rich metadata for each.
*   **Version Control:** Strict semantic versioning enforcement for packages.
*   **Security:** Leverages Nimbus OS's secure environment for registry operations and package hosting. All packages are scanned for vulnerabilities before publication.
*   **Decentralization (Future):** While starting as a central registry, the architecture will be designed to allow for federated or decentralized registry instances.

## 2. Package Identity and Naming

*   **Unique Name:** Each package has a globally unique name (e.g., `zenith_linalg`, `quantum_simulator_lib`, `nano_swarm_utils`).
*   **Semantic Versioning:** Packages follow `MAJOR.MINOR.PATCH` versioning (e.g., `1.2.3`). Pre-release versions (`1.2.3-alpha.1`) and build metadata (`1.2.3+build.123`) are supported.
*   **Qualified Names:** When needed, packages can be referred to by a fully qualified name, e.g., `registry.zenith-lang.org/my_org/my_package@1.0.0`.

## 3. Registry API (Conceptual RESTful API)

The Zenith Package Registry exposes a RESTful API for interaction by `zenith-pkg` and other tools.

### 3.1. Authentication

*   All write operations (publish, yank, delete) require authentication (e.g., OAuth2, API tokens linked to Zenith developer accounts).
*   Read operations (search, fetch metadata, download) can be unauthenticated.

### 3.2. Endpoints

#### `GET /packages`

*   **Description:** Lists all available packages in the registry.
*   **Query Params:**
    *   `q`: Search query (e.g., package name, description keywords).
    *   `limit`, `offset`: Pagination.
    *   `sort`: Sorting criteria (e.g., `downloads`, `last_updated`, `name`).
    *   `paradigm`: Filter by primary paradigm (`classical`, `quantum`, `nano`, `mts`, `sankofa`).
    *   `target`: Filter by supported build target (`nimbus-vm`, `x86_64`, `qpu_ibm`, etc.).
*   **Response:** `200 OK` with a list of `PackageSummary` objects.

#### `GET /packages/{package_name}`

*   **Description:** Retrieves metadata for a specific package.
*   **Response:** `200 OK` with a `PackageDetails` object including all versions.

#### `GET /packages/{package_name}/{version}`

*   **Description:** Retrieves detailed metadata for a specific package version.
*   **Response:** `200 OK` with a `PackageVersionDetails` object. This includes the parsed `Zenith.toml`, dependency tree, verified properties, and download URL.

#### `GET /packages/{package_name}/{version}/download`

*   **Description:** Downloads the `.zpkg` archive for a specific package version.
*   **Response:** `200 OK` with `application/octet-stream` (the `.zpkg` file).

#### `POST /packages/publish`

*   **Description:** Publishes a new package version to the registry.
*   **Headers:** `Authorization: Bearer <token>`, `Content-Type: application/octet-stream`
*   **Body:** The `.zpkg` file content.
*   **Process:**
    1.  The registry extracts the `Zenith.toml` from the `.zpkg`.
    2.  Validates the `Zenith.toml` structure and version.
    3.  Performs security scans (`E.V.A.S.`) on the package content.
    4.  Submits the package to a conceptual **Formal Verification Service** (leveraging `zenith-fv`).
    5.  If all checks pass, the package is stored, metadata is indexed, and a `PackageVersionDetails` is returned.
*   **Response:** `201 Created` with `PackageVersionDetails` or `400 Bad Request`, `401 Unauthorized`, `409 Conflict` (if version exists), `422 Unprocessable Entity` (if validation/verification fails).

#### `POST /packages/{package_name}/{version}/yank`

*   **Description:** Marks a package version as "yanked", meaning it will no longer be resolved by `zenith-pkg` for new projects, but existing projects can still access it.
*   **Response:** `204 No Content`.

#### `DELETE /packages/{package_name}/{version}` (Admin only)

*   **Description:** Permanently deletes a package version (rarely used, only for severe issues).
*   **Response:** `204 No Content`.

## 4. Package Metadata (Conceptual)

Beyond the basic `Zenith.toml` contents, the registry stores additional metadata:

*   **Verification Status:** `Proved`, `Disproved`, `Unproven` (from formal verification).
*   **Security Score:** `0-100` score from `E.V.A.S.` scanner.
*   **Performance Benchmarks:** Aggregate benchmark data for key functions across targets.
*   **API Documentation URL:** Link to auto-generated documentation.
*   **Paradigm Tags:** Explicit tags like `classical`, `quantum-measurement`, `nano-swarm`, `mts-speculative`, `sankofa-temporal`.
*   **Nimbus OS Compatibility:** Specific Nimbus OS kernel versions or security policies required.

## 5. Formal Verification Service Integration

Upon package submission, the registry triggers a separate Formal Verification Service:

1.  Extracts Zenith source from `.zpkg`.
2.  Runs `zenithc` with `zenith-fv` integration, applying a predefined set of verification properties (e.g., `MemorySafety`, `CausalConsistency`, `EntanglementPurity`).
3.  Verification results (proofs, counter-examples) are stored with the package metadata. This status is prominently displayed to users.

## 6. `zenith-pkg` Interaction Flow (Conceptual)

1.  **`zenith-pkg install <package>`:**
    *   Queries `GET /packages/{package_name}/{version}` (or latest).
    *   Downloads `.zpkg` from `/download`.
    *   Extracts `.zpkg` to local cache.
    *   Recursively resolves and installs dependencies.
    *   Builds the package from source if necessary (or uses pre-compiled artifacts).
2.  **`zenith-pkg publish`:**
    *   Bundles current project into `.zpkg`.
    *   Calls `POST /packages/publish`.
    *   Awaits verification results before marking as fully `published`.

This conceptual specification provides a detailed vision for a robust, secure, and multi-paradigm-aware package registry for Zenith.
