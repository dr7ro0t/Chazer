# 🗺️ What's Next

Planned features and improvements for future releases:

### Completed Phases

#### Phase 5: Performance & Scale ✅

- [x] **Incremental analysis** - Cache parsed ASTs and only re-analyze changed files (`--incremental`)
- [x] **Watch mode** - Continuous analysis during development (`--watch`)
- [x] **Optimized reachability** - ~8% faster analysis on large codebases
- [ ] **Parallel graph construction** - Parallelize reference resolution phase
- [ ] **Memory optimization** - Reduce memory footprint for very large codebases (100k+ files)

#### Phase 6: Enhanced Detection ✅

- [x] **Unused function parameters** - Detect parameters that are never used in function body (`--unused-params`)
- [x] **Dead string resources** - Cross-reference `R.string.*` usage with `strings.xml` (`--unused-resources`)
- [ ] **Redundant null checks** - Detect null checks on non-nullable types
- [ ] **Unused type parameters** - Detect generic type parameters that aren't used
- [ ] **Unused Gradle dependencies** - Detect declared but unused library dependencies

#### Phase 7: CI Integration ✅ (Partial)

- [x] **Baseline support** - Ignore existing dead code, only flag new issues (`--baseline`)
- [ ] **Language Server Protocol (LSP)** - Real-time dead code highlighting in editors
- [ ] **IntelliJ/Android Studio plugin** - Native IDE integration
- [x] **GitHub Action** - Pre-built action for easy CI setup (`uses: dr7ro0t/SearchDeadCode@v0`)
- [x] **Pre-commit hook** - Block commits introducing dead code (`scripts/pre-commit-hook.sh`)

#### Phase 9: Write-Only Detection ✅ (Mostly Complete)

- [x] **Write-only variables** - Variables assigned but never read (`--write-only`)
- [x] **Write-only SharedPreferences** - prefs.putString() without getString() (`--write-only-prefs`)
- [x] **Write-only database tables** - @Insert without @Query consumers (`--write-only-dao`)
- [ ] **Write-only cache** - Cache writes that are never read

#### Phase 10: Sealed Class & Override Analysis ✅

- [x] **Unused sealed variants** - Sealed class cases never instantiated (`--sealed-variants`)
- [x] **Redundant overrides** - Override methods that only call super (`--redundant-overrides`)

#### Phase 11: Intent & Data Flow ✅ (Partial)

- [ ] **Ignored return values** - `list.map{}` without capturing result
- [x] **Unused intent extras** - putExtra() without getExtra() (`--unused-extras`)
- [ ] **Redundant null checks** - Double null checks after safe calls

### Upcoming Phases

#### Phase 8: Multi-Platform

- [ ] **iOS/Swift support** - Extend to Swift/Objective-C projects
- [ ] **React Native** - Analyze both native and JavaScript layers
- [ ] **Flutter/Dart** - Support Dart language analysis
- [ ] **KMP (Kotlin Multiplatform)** - Proper shared code analysis

#### Phase 12: Observable State ⭐⭐

- [ ] **Dead feature flags** - Flags always true/false
- [ ] **Unobserved StateFlow/LiveData** - State never collected in UI

#### Phase 13: Advanced Flow Analysis ⭐

- [ ] **Partially dead code** - Code computed on all paths but used on some
- [ ] **Recalculation detection** - Redundant recomputation of available values
- [ ] **Event bus orphans** - Events posted without subscribers

### Contributing to Future Development

Want to help? Here are good first issues:

1. **Add new annotation support** - Easy: add annotation names to `entry_points.rs`
2. **Improve XML parsing** - Medium: add support for more XML attributes
3. **Write tests** - Medium: add test cases for edge cases
4. **Performance profiling** - Advanced: identify and fix bottlenecks
5. **LSP implementation** - Advanced: implement language server protocol

See [CONTRIBUTING.md](../CONTRIBUTING.md) for development setup and guidelines.