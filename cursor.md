# TPC-C Playground - Development Guidelines

This document contains coding standards and architectural decisions for the TPC-C playground project.

## Module Organization

### Preferred Pattern: Named Module Files

For organizing modules, prefer the **named module file** pattern over `mod.rs` files.

#### ✅ Preferred (handlers pattern):
```
src/
├── handlers.rs          # Module declarations and re-exports
└── handlers/
    ├── warehouses.rs
    ├── stock_level.rs
    └── order_status.rs
```

**handlers.rs content:**
```rust
pub mod warehouses;
pub mod stock_level;
pub mod order_status;

pub use warehouses::*;
pub use stock_level::*;
pub use order_status::*;
```

#### ❌ Avoid (mod.rs pattern):
```
src/
└── handlers/
    ├── mod.rs           # Avoid this pattern
    ├── warehouses.rs
    ├── stock_level.rs
    └── order_status.rs
```

### Rationale

1. **Clearer Intent**: The module name is explicit in the filename (`handlers.rs` vs generic `mod.rs`)
2. **Easier Navigation**: IDE navigation shows meaningful names in tabs
3. **Better File Management**: Reduces confusion when multiple `mod.rs` files are open
4. **Consistent Pattern**: Follows the same approach across the entire codebase

### Implementation

When adding new module groups:
1. Create `src/{module_name}.rs` for module declarations
2. Create `src/{module_name}/` directory for individual module files
3. In `{module_name}.rs`, declare all submodules and re-export public items
4. Keep individual module files focused and cohesive

## TPC-C Specific Guidelines

### Database Table Naming
- Tables follow the pattern: `{table_name}1` (e.g., `warehouse1`, `customer1`)
- This matches the sysbench TPC-C schema convention

### Handler Structure
- Use query parameters for TPC-C transaction inputs
- Return structured JSON responses with clear field names
- Include proper error handling (404 for not found, 500 for database errors)
- Follow TPC-C transaction specifications for data access patterns

### Time Handling
- Use `chrono::NaiveDateTime` for PostgreSQL `timestamp` columns
- SQLx with `chrono` feature enabled for proper timestamp support
- Avoid `DateTime<Utc>` unless specifically needed for timezone-aware operations

### Response Structure
- Group related data in nested objects (e.g., customer info, order details)
- Use Option<T> for nullable database fields
- Include all relevant TPC-C transaction outputs in responses

## Development Workflow

### Code Quality Tools

#### ✅ Preferred: Use `cargo clippy` over `cargo check`

Always use `cargo clippy` instead of `cargo check` for code validation:

```bash
cargo clippy                    # Check for issues and linting
cargo clippy --all-targets     # Check all targets (lib, bins, tests, etc.)
cargo clippy --fix             # Auto-fix simple issues
```

#### Rationale

1. **Comprehensive Analysis**: Clippy includes all the checks that `cargo check` does, plus hundreds of additional lints
2. **Code Quality**: Catches potential bugs, performance issues, and style problems
3. **Best Practices**: Suggests more idiomatic Rust code
4. **Learning Tool**: Helps developers write better Rust code over time

#### Development Loop

```bash
# Standard development workflow:
cargo clippy                    # Validate code quality
cargo test                      # Run tests  
cargo run                       # Test the application
```

#### CI/CD Integration

For automated builds, use clippy with strict settings:
```bash
cargo clippy -- -D warnings     # Treat clippy warnings as errors
```

---

*Last updated: 2025-08-13*