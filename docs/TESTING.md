# SearchDeadCode Testing Guide

## Running Tests

### All Tests
```bash
cargo test
```

### Unit Tests Only
```bash
cargo test --lib
```

### Integration Tests Only
```bash
cargo test --test integration
```

### Specific Detector Tests
```bash
cargo test sealed_variant
cargo test write_only
cargo test unused_param
cargo test unused_intent_extra
```

### With Verbose Output
```bash
cargo test -- --nocapture
```

## Test Structure

```
tests/
├── fixtures/           # Test fixtures (sample code)
│   ├── kotlin/
│   │   ├── dead_code.kt      # Various dead code patterns
│   │   └── all_used.kt       # Clean code (no dead code)
│   ├── java/
│   │   └── DeadCode.java     # Java dead code patterns
│   └── android/
│       └── MainActivity.kt   # Android-specific patterns
└── integration/
    └── analysis_test.rs      # Integration tests
```

## TDD Workflow

### 1. Write a Failing Test First

```rust
#[test]
fn test_new_detection_feature() {
    let fixture = fixtures_path().join("kotlin/new_feature.kt");
    let graph = build_graph_from_file(&fixture);
    let detector = NewFeatureDetector::new();
    let issues = detector.detect(&graph);

    // Assert expected behavior
    assert!(!issues.is_empty(), "Should detect the new pattern");
    assert_eq!(issues[0].declaration.name, "expectedDeadCode");
}
```

### 2. Create Test Fixture

Create a Kotlin/Java file in `tests/fixtures/` that demonstrates the pattern:

```kotlin
// tests/fixtures/kotlin/new_feature.kt
class NewFeatureExample {
    // Pattern to detect goes here
}
```

### 3. Implement the Detector

Implement the minimum code to make the test pass.

### 4. Refactor

Refactor while keeping tests green.

## Test Categories

### Unit Tests (in `src/`)

Each detector module has `#[cfg(test)] mod tests`:

- **sealed_variant**: Tests sealed class detection
- **write_only**: Tests write-only variable detection
- **unused_param**: Tests unused parameter detection
- **unused_intent_extra**: Tests Android Intent extra detection
- **ignored_return**: Tests pure function detection
- **redundant_override**: Tests override detection

### Integration Tests

Located in `tests/integration/analysis_test.rs`:

- `test_dead_code_kotlin_fixture`: Parses Kotlin with dead code
- `test_all_used_kotlin_fixture`: Parses clean Kotlin code
- `test_java_fixture`: Parses Java files
- `test_write_only_detector_on_fixture`: Runs write-only detector
- `test_sealed_variant_detector_on_fixture`: Runs sealed detector
- `test_reachability_analysis`: Tests reachability analysis
- `test_multiple_files`: Tests multi-file analysis

## Adding New Tests

### 1. Unit Test for a Detector Method

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_name() {
        let detector = MyDetector::new();
        // Create test data
        // Assert expected behavior
    }
}
```

### 2. Integration Test for Full Pipeline

```rust
#[test]
fn test_feature_integration() {
    let fixture = fixtures_path().join("kotlin/feature.kt");
    if !fixture.exists() {
        return;
    }

    let graph = build_graph_from_file(&fixture);
    // Run analysis
    // Assert results
}
```

### 3. Fixture-Based Test

1. Add fixture file to `tests/fixtures/`
2. Write test that loads and analyzes it
3. Assert expected findings

## Coverage

To generate coverage reports:

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Generate coverage
cargo llvm-cov

# Generate HTML report
cargo llvm-cov --html
```

## Benchmarks

```bash
# Run benchmarks
cargo bench

# Benchmark specific feature
cargo bench parsing
```

## Continuous Integration

Tests run automatically on:
- Push to main branch
- Pull requests
- Release builds

## Best Practices

1. **Write tests first** - TDD ensures better design
2. **Keep fixtures small** - Minimal code to test the pattern
3. **Test edge cases** - Empty files, malformed code, etc.
4. **Use descriptive names** - `test_skip_enum_classes` not `test1`
5. **Assert specific behavior** - Check names, locations, not just counts
6. **Keep tests fast** - Unit tests should complete in milliseconds
