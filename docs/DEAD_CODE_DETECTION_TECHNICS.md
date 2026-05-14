# Dead Code Detection Paradigms & Research

This section documents the various paradigms and techniques used for dead code detection, based on research across
industry tools and academic literature.

## Overview of Detection Techniques

According to systematic literature reviews, there are two main approaches for automating dead code detection:

| Approach                   | Description                                                                  | Tools                                  |
|----------------------------|------------------------------------------------------------------------------|----------------------------------------|
| **Accessibility Analysis** | Build dependency graph, traverse from entry points, mark unreachable as dead | Periphery, SearchDeadCode, R8/ProGuard |
| **Data Flow Analysis**     | Track how data flows through program, identify unused computations           | Compilers (DCE), Static analyzers      |

### 1. Graph-Based Reachability Analysis

This is the approach used by SearchDeadCode, inspired by [Periphery](https://github.com/peripheryapp/periphery):

```
Entry Points → Build Dependency Graph → DFS/BFS Traversal → Mark Reachable → Report Unreachable
```

**How Periphery works:**

1. Build project to generate the "index store" with declaration/reference info
2. Build in-memory graph of relational structure
3. Mutate graph to mark entry points
4. Traverse graph from roots to identify unreferenced declarations

**Key insight**: The index store contains detailed information about declarations and their references, enabling
accurate cross-file analysis.

### 2. Static + Dynamic Hybrid Analysis (Meta's SCARF)

[Meta's SCARF system](https://engineering.fb.com/2023/10/24/data-infrastructure/automating-dead-code-cleanup/) combines
multiple analysis techniques:

**Capabilities:**

- **Multi-language support**: Java, Objective-C, JavaScript, Hack, Python
- **Symbol-level analysis**: Analyzes individual variables, not just files/classes
- **Static analysis via Glean**: Indexed, standardized format for static facts
- **Runtime monitoring**: Observes actual code execution in production
- **Cycle detection**: Detects mutually dependent dead code subgraphs

**Impact at Meta:**

- Deleted 104+ million lines of code
- Removed petabytes of deprecated data
- 370,000+ automated change requests

**Key technique**: SCARF tracks two metrics - static usage (code that appears to use data) and runtime usage (actual
access patterns in production).

### 3. Tree Shaking (JavaScript Bundlers)

[Webpack](https://webpack.js.org/guides/tree-shaking/) and [Rollup](https://rollupjs.org/) popularized tree shaking:

> "Start with what you need, and work outwards" vs "Start with everything, and work backwards"

**Algorithm:**

1. Build dependency graph from entry points
2. Identify all exports in modules
3. Trace which exports are actually imported/used
4. Eliminate code not reached during traversal

**Requirements:**

- ES6 module syntax (`import`/`export`) - static structure required
- CommonJS (`require`) cannot be tree-shaken due to dynamic nature

**Webpack's implementation:**

- `usedExports` optimization marks unused exports
- Terser performs final dead code elimination
- Works at module boundary level

### 4. Compiler-Based Dead Code Elimination

[R8/ProGuard](https://blog.logrocket.com/r8-code-shrinking-android-guide/) for Android:

**Process:**

1. Entry points declared in ProGuard config
2. Search for all reachable code from entry points
3. Build list of reachable tokens
4. Strip anything not in the list

**R8 advantages over ProGuard:**

- Faster (single-pass: shrink + optimize + dex)
- Better Kotlin support
- More aggressive inlining and class merging
- ~10% size reduction vs ProGuard's ~8.5%

### 5. Scope & Namespace Tracking

Tools
like [ReSharper](https://www.jetbrains.com/help/resharper/Code_Analysis__Solution-Wide_Analysis__Solution-Wide_Code_Inspections.html)
use solution-wide analysis:

**Capabilities:**

- Detect unused non-private members (requires whole-solution analysis)
- Track namespace imports across files
- Identify redundant type casts and unused variables
- Real-time analysis during development

**Key insight**: Some dead code can only be detected at solution/project scope, not file scope.

### 6. Transitive Dependency Analysis

Tools like [deptry](https://github.com/fpgmaas/deptry) (Python) and [Knip](https://knip.dev/) (TypeScript):

**Detects:**

- Unused dependencies (declared but not imported)
- Missing dependencies (imported but not declared)
- Transitive dependencies (used but only available through other packages)

**Multi-module support:**

- Analyze relationships between workspaces
- Understand monorepo dependency structure
- Detect cross-module dead code

### 7. Compiler Optimization Techniques

From compiler theory ([Wikipedia - Dead Code Elimination](https://en.wikipedia.org/wiki/Dead-code_elimination)):

**Data Flow Analysis:**

- Build Control Flow Graph (CFG)
- Perform liveness analysis
- Identify variables written but never read
- Remove unreachable basic blocks

**Escape Analysis:**

- Determine dynamic scope of pointers
- Enable stack allocation for non-escaping objects
- Remove synchronization for thread-local objects

**SSA-based DCE:**

- Static Single Assignment form simplifies analysis
- Each variable assigned exactly once
- Dead assignments easily identified

### 8. Incremental Analysis (Large Codebases)

For large codebases, incremental analysis is essential:

**Techniques:**

- **Caching**: Store cryptographic hashes of analysis results
- **Memoization**: Reuse unchanged computation results
- **Dependency tracking**: Only re-analyze affected code
- **Index stores**: Pre-computed declaration/reference indexes

**Tools using incremental analysis:**

- [Glean](https://glean.software/) (Meta) - Incremental indexing
- [Roslyn](https://github.com/dotnet/roslyn) - Incremental generators with aggressive caching
- Periphery - Index store from compiler

### Comparison of Approaches

| Paradigm           | Accuracy | Speed            | Scope          | Best For             |
|--------------------|----------|------------------|----------------|----------------------|
| Graph Reachability | High     | Fast             | Project        | General dead code    |
| Static + Dynamic   | Highest  | Slow             | Organization   | Production code      |
| Tree Shaking       | High     | Fast             | Bundle         | JavaScript modules   |
| Compiler DCE       | Highest  | Build-time       | Binary         | Release builds       |
| Scope Analysis     | Medium   | Real-time        | IDE            | Development feedback |
| Coverage-based     | Medium   | Requires runtime | Executed paths | Test coverage gaps   |

### Challenges & Limitations

1. **Halting Problem**: Theoretically impossible to find ALL dead code deterministically
2. **Reflection**: Dynamically invoked code cannot be detected statically
3. **Polymorphism**: Must know all possible types for method resolution
4. **Configuration**: Code referenced in XML, properties files, etc.
5. **Dynamic Languages**: Less static structure = harder analysis

### Future Improvements for SearchDeadCode

Based on this research, potential enhancements include:

| Feature                   | Description                                       | Inspiration   | Status                    |
|---------------------------|---------------------------------------------------|---------------|---------------------------|
| **Symbol-level analysis** | Track individual variables, not just declarations | Meta SCARF    | ✅ Done (v0.3.0 deep mode) |
| **Cycle detection**       | Find mutually dependent dead code                 | Meta SCARF    | ✅ Done (v0.2.0)           |
| **Coverage integration**  | Augment static analysis with runtime data         | Hybrid tools  | ✅ Done (v0.2.0)           |
| **Incremental mode**      | Cache results, only re-analyze changes            | Glean, Roslyn | Planned                   |
| **Transitive tracking**   | Track full reference chains                       | deptry, Knip  | Partial                   |
| **Cross-module analysis** | Analyze multi-module projects holistically        | Knip          | Planned                   |

## Advanced Dead Code Patterns - Prioritized Implementation Roadmap

This section documents advanced dead code patterns beyond traditional "unreferenced code" detection. These patterns
represent **code that executes but serves no purpose** - a more insidious form of technical debt.

Based on analysis of real-world Android codebases (1800+ files), we've prioritized these patterns by:

- **Detectability**: How accurately can static analysis find this? (High/Medium/Low)
- **Frequency**: How common is this pattern? (Based on real-world codebase analysis)
- **Impact**: How much wasted code/resources? (High/Medium/Low)

### Priority Tier 1: High Impact, High Detectability ⭐⭐⭐

These patterns are common, easy to detect, and represent significant waste.

| #      | Pattern                                   | Detectability | Frequency         | Description                                                            |
|--------|-------------------------------------------|---------------|-------------------|------------------------------------------------------------------------|
| **1**  | **Write-Only Variables**                  | High          | 58+ occurrences   | Variables assigned but never read (`private var x = 0` without reads)  |
| **2**  | **Unused Sealed Class Variants**          | High          | 73 sealed classes | Sealed class/interface cases that are never instantiated               |
| **3**  | **Override Methods That Only Call Super** | High          | 284 overrides     | `override fun onCreate() { super.onCreate() }` - adds no value         |
| **4**  | **Ignored Return Values**                 | High          | Common            | `list.map { transform(it) }` without using the result                  |
| **5**  | **Empty Catch Blocks**                    | High          | Common            | `catch (e: Exception) { }` - swallowed errors                          |
| **6**  | **Unused Intent Extras**                  | High          | 90 putExtra calls | `intent.putExtra("key", value)` where "key" is never read              |
| **7**  | **Write-Only SharedPreferences**          | High          | Medium            | `prefs.edit().putString("x", y).apply()` where "x" is never read       |
| **8**  | **Write-Only Database Tables**            | High          | 16 DAOs           | `@Insert` without corresponding `@Query` usage                         |
| **9**  | **Redundant Null Checks**                 | High          | Common            | `user?.let { if (it != null) }` - double null check                    |
| **10** | **Dead Feature Flags**                    | Medium        | 388 isEnabled     | `if (RemoteConfig.isFeatureEnabled())` where flag is always true/false |

### Priority Tier 2: Medium Impact, High Detectability ⭐⭐

Detectable patterns with moderate frequency.

| #      | Pattern                             | Detectability | Frequency     | Description                                                     |
|--------|-------------------------------------|---------------|---------------|-----------------------------------------------------------------|
| **11** | **Unobserved LiveData/StateFlow**   | Medium        | 64 collectors | `_state.value = x` where `_state` is never observed in UI       |
| **12** | **Unused Constructor Parameters**   | High          | Medium        | Parameters passed to constructor but never used                 |
| **13** | **Middle-Man Classes**              | Medium        | Low           | Classes that only delegate to other classes with no added logic |
| **14** | **Lazy Classes**                    | Medium        | Low           | Classes with minimal logic that could be inlined                |
| **15** | **Invariants Always True/False**    | High          | Common        | `if (list.size >= 0)` - always true                             |
| **16** | **Cache Write Without Read**        | Medium        | Medium        | `cache.save(data)` but always fetching from network             |
| **17** | **Analytics Events Never Analyzed** | Low           | 253 log calls | Events tracked but no dashboard configured                      |
| **18** | **Unused Type Parameters**          | High          | Low           | `class Foo<T>` where T is never used in the class               |
| **19** | **Dead Migrations**                 | Medium        | Low           | Database migrations for versions no user has anymore            |
| **20** | **Listeners Never Triggered**       | Medium        | Medium        | `view.setOnClickListener { }` on views that can't be clicked    |

### Priority Tier 3: High Impact, Lower Detectability ⭐

High-value patterns that require more sophisticated analysis.

| #      | Pattern                                           | Detectability | Frequency | Description                                                     |
|--------|---------------------------------------------------|---------------|-----------|-----------------------------------------------------------------|
| **21** | **Dormant Code Reactivated** (Knight Capital Bug) | Low           | Rare      | Old code accidentally enabled by feature flags                  |
| **22** | **Defensive Copies Never Modified**               | Medium        | Low       | `val copy = list.toMutableList()` but copy never mutated        |
| **23** | **Calculations Overwritten Immediately**          | Medium        | Low       | `var x = expensiveCalc(); x = otherValue`                       |
| **24** | **Partially Dead Code**                           | Medium        | Medium    | Code only used on some branches but computed on all             |
| **25** | **Recalculation of Available Values**             | Medium        | Low       | `val h1 = data.hash(); ... val h2 = data.hash()`                |
| **26** | **Audit Logs Never Queried**                      | Low           | Low       | `auditDao.insert(log)` with no read methods                     |
| **27** | **Breadcrumbs Without Consumer**                  | Low           | Low       | Navigation history saved but never displayed                    |
| **28** | **Event Bus Without Subscribers**                 | Medium        | Low       | `eventBus.post(event)` with no `@Subscribe` for that event type |
| **29** | **Coroutines Launched Then Cancelled**            | Low           | Medium    | Jobs cancelled before completing meaningful work                |
| **30** | **Workers That Produce Unused Output**            | Low           | Low       | WorkManager jobs whose results are never consumed               |

### Priority Tier 4: Specialized Patterns ⭐

Domain-specific or less common patterns.

| #      | Pattern                                | Detectability | Frequency        | Description                                      |
|--------|----------------------------------------|---------------|------------------|--------------------------------------------------|
| **31** | **Annotations Without Effect**         | Medium        | Low              | `@Keep` when ProGuard isn't configured to use it |
| **32** | **Validation After The Fact**          | Medium        | Low              | `db.insert(x); require(x.isValid)` - too late    |
| **33** | **Unused Debug Logging**               | High          | 253 Timber calls | Logs in production that output to nowhere        |
| **34** | **Semi-Dead Classes**                  | Medium        | Low              | Classes used as types but never instantiated     |
| **35** | **Test-Only Code in Production**       | High          | Medium           | Code only referenced by tests, never production  |
| **36** | **Timestamps Never Used**              | Medium        | Low              | `updatedAt` field maintained but never queried   |
| **37** | **Serializable Without Serialization** | Medium        | Low              | `@Serializable` on classes never serialized      |
| **38** | **Crashlytics Keys Never Filtered**    | Low           | Low              | Custom keys set but never used in dashboard      |
| **39** | **Threads Spawned Without Work**       | Low           | Rare             | Executor pools with empty task queues            |
| **40** | **Configuration Values Never Read**    | Medium        | Medium           | Properties defined but never accessed            |

### Implementation Phases

Based on the priority analysis, here's the recommended implementation order:

#### Phase 9: Write-Only Detection (Highest ROI)

```
Priority: ⭐⭐⭐⭐⭐
Patterns: #1, #7, #8, #26
Estimated dead code found: 15-25% increase
```

**Detectors to implement:**

- `WriteOnlyVariableDetector` - Variables assigned but never read
- `WriteOnlyPreferenceDetector` - SharedPreferences written but never read
- `WriteOnlyDatabaseDetector` - DAO methods with @Insert but no @Query callers

**Algorithm:**

1. For each variable/property, track all assignments (writes)
2. Track all reads (usages that don't assign)
3. If writes > 0 && reads == 0, report as write-only

#### Phase 10: Sealed Class & Override Analysis

```
Priority: ⭐⭐⭐⭐
Patterns: #2, #3
Estimated dead code found: 10-15% increase
```

**Detectors to implement:**

- `UnusedSealedVariantDetector` - Sealed subclasses never instantiated
- `RedundantOverrideDetector` - Overrides that only call super

**Algorithm for sealed variants:**

1. Find all sealed class/interface definitions
2. Find all subclasses/implementations
3. For each subclass, check if it's ever instantiated (constructor called)
4. Report never-instantiated subclasses

#### Phase 11: Intent & Data Flow

```
Priority: ⭐⭐⭐
Patterns: #4, #6, #9
Estimated dead code found: 5-10% increase
```

**Detectors to implement:**

- `IgnoredReturnValueDetector` - Function results not captured
- `UnusedIntentExtraDetector` - putExtra without getExtra
- `RedundantNullCheckDetector` - Double null checks

#### Phase 12: Observable State Analysis

```
Priority: ⭐⭐
Patterns: #10, #11, #16
Estimated dead code found: 5-8% increase
```

**Detectors to implement:**

- `DeadFeatureFlagDetector` - Flags always true/false
- `UnobservedStateDetector` - StateFlow/LiveData never collected
- `WriteOnlyCacheDetector` - Cache writes without reads

#### Phase 13: Advanced Flow Analysis

```
Priority: ⭐
Patterns: #21-30
Estimated dead code found: 2-5% increase
```

**Detectors to implement:**

- `PartiallyDeadCodeDetector` - Code used only on some paths
- `RecalculationDetector` - Redundant recomputation
- `EventBusOrphanDetector` - Events without subscribers

### Pattern Detection Examples

#### Write-Only Variable (#1)

```kotlin
class Analytics {
    private var lastEventTime: Long = 0  // DEAD: never read

    fun track(event: Event) {
        lastEventTime = System.currentTimeMillis()  // write-only
        send(event)
    }
}
```

#### Unused Sealed Variant (#2)

```kotlin
sealed class UiState {
    object Loading : UiState()          // Used
    data class Success(val data: Data) : UiState()  // Used
    data class Error(val msg: String) : UiState()   // Used
    object Empty : UiState()            // DEAD: never emitted
}
```

#### Override Only Calling Super (#3)

```kotlin
override fun onCreateView(...): View {
    return super.onCreateView(inflater, container, savedInstanceState)
    // DEAD: If this is all it does, the override is unnecessary
}
```

#### Write-Only Database (#8)

```kotlin
@Dao
interface ReadHistoryDao {
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun saveReadArticle(history: ReadHistory)  // Called

    @Query("SELECT * FROM read_history ORDER BY timestamp DESC")
    fun getReadHistory(): Flow<List<ReadHistory>>  // DEAD: never called!
}
```

#### Ignored Return Value (#4)

```kotlin
// DEAD: The sorted list is never used
articles.sortedByDescending { it.date }
adapter.submitList(articles)  // Still the original unsorted list!
```

#### Dead Feature Flag (#10)

```kotlin
// The flag has been true for 2 years
if (RemoteConfig.isNewPlayerEnabled()) {  // Always true
    playWithExoPlayer()
} else {
    playWithMediaPlayer()  // DEAD: never executed
}
```

### Codebase Analysis Results

From our analysis of a real-world Android project (1806 files):

| Pattern Category         | Occurrences | Potential Dead Code             |
|--------------------------|-------------|---------------------------------|
| Timber/Log calls         | 253         | ~50% may be production-silent   |
| Override methods         | 284         | ~10-20% may only call super     |
| Intent extras (putExtra) | 90          | ~30% may be unread              |
| Sealed classes           | 73          | ~5-10% may have unused variants |
| Feature flags            | 388         | ~20% may be dead branches       |
| Flow collectors          | 64          | ~10% may be unobserved          |
| Map operations           | 72          | ~5% may have ignored results    |
| Private vars             | 58          | ~20% may be write-only          |
| DAO @Insert methods      | 16          | ~10% may be write-only tables   |
| DAO @Query methods       | 49          | (Need cross-reference analysis) |

**Estimated additional dead code**: Using these advanced detectors could identify **30-50% more dead code** beyond
current detection.

### Manual Investigation Results - Verified Examples

Through thorough manual investigation of a real-world Android codebase, we verified the following concrete examples:

#### Confirmed Write-Only Variables (Pattern #1)

**Example 1: `feedStartUpdatingTimestamp` in NewsToolbarController.kt:65**

```kotlin
private var feedStartUpdatingTimestamp = 0L  // Line 65

// Only written, never read:
feedStartUpdatingTimestamp = timeService.now().toInstant().toEpochMilli()  // Line 102
```

**File**: `feature-news/src/main/java/com/example/feed/news/toolbar/NewsToolbarController.kt`

**Example 2: Same pattern in ShowcaseToolbarController.kt:50**

```kotlin
private var feedStartUpdatingTimestamp = 0L  // Line 50

// Only written, never read:
feedStartUpdatingTimestamp = timeService.now().toInstant().toEpochMilli()  // Line 124
```

**File**: `feature-showcase/src/main/java/com/example/feed/showcase/ui/toolbar/ShowcaseToolbarController.kt`

**Impact**: 2 confirmed write-only variables that store timestamps but never use them.

#### Confirmed Empty Override Methods (Pattern #3)

Found 20+ empty override methods that add no value:

| File                               | Line          | Method                                                                              |
|------------------------------------|---------------|-------------------------------------------------------------------------------------|
| `ShowcaseToolbarController.kt`     | 137           | `override fun onFragmentViewDestroyed() {}`                                         |
| `ListViewsFactory.kt`              | 30            | `override fun onCreate() {}`                                                        |
| `ListViewsFactory.kt`              | 46            | `override fun onDestroy() {}`                                                       |
| `StartupAdController.kt`           | 248           | `override fun onActivityStarted(activity: Activity) {}`                             |
| `StartupAdController.kt`           | 249           | `override fun onActivityPaused(activity: Activity) {}`                              |
| `StartupAdController.kt`           | 250           | `override fun onActivityStopped(activity: Activity) {}`                             |
| `StartupAdController.kt`           | 251           | `override fun onActivitySaveInstanceState(activity: Activity, outState: Bundle) {}` |
| `TimeViewHolder.kt`                | 76            | `override fun unbind() {}`                                                          |
| `MenuFeedDataSource.kt`            | 27            | `override fun onAdapterViewBinded(position: Int) {}`                                |
| `SingleScrollDirectionEnforcer.kt` | 44            | `override fun onTouchEvent(rv: RecyclerView, e: MotionEvent) {}`                    |
| `SingleScrollDirectionEnforcer.kt` | 46            | `override fun onRequestDisallowInterceptTouchEvent(disallowIntercept: Boolean) {}`  |
| `MultipleCardFragment.kt`          | 139, 149, 151 | Empty animation listener methods                                                    |

**Impact**: These are interface requirements but represent code that does nothing.

#### Patterns NOT Found (False Positives Avoided)

During investigation, these patterns were verified as **properly used** (NOT dead code):

1. **GlucheStatusDao.get()** - Initially looked write-only but is called via
   `GlucheRepositoryImpl.getGluchePostStatus()`
2. **BannerDao.exists()** - Called via `BannerRepository.isDismissed()`
3. **beNotificationID/Secret preferences** - Both written and read in `BackEndNotificationService.kt`
4. **intervalCheckInMilliseconds** - Assigned in `init` and read in `scheduleVerifyIfServerHasNewPosts()`
5. **newDeepLinkIntent** - Both getter and setter are used across multiple files

This validates that our detection algorithm must follow the full call chain through repositories and services.

### Detection Algorithm Requirements

Based on the investigation, the Write-Only Variable detector must:

1. **Track all assignments** to private variables
2. **Track all reads** (usages that don't assign)
3. **Exclude reads inside the assignment expression** (`x = x + 1` counts `x` as read)
4. **Handle property delegates** (`by lazy`, `by BooleanPreferenceDelegate`)
5. **Handle backing fields** with custom getters/setters
6. **Report if**: writes > 0 && reads == 0

The Empty Override detector must:

1. **Find all `override fun`** declarations
2. **Check if body is empty** or only contains `super.method()`
3. **Exclude**: Abstract implementations where empty is intentional (e.g., `LifecycleObserver`)
4. **Report with confidence level** based on interface type

## Research Sources

- [Meta - Automating Dead Code Cleanup](https://engineering.fb.com/2023/10/24/data-infrastructure/automating-dead-code-cleanup/)
- [Periphery - Swift Dead Code Detection](https://github.com/peripheryapp/periphery)
- [Webpack - Tree Shaking Guide](https://webpack.js.org/guides/tree-shaking/)
- [Tree Shaking Reference Guide - Smashing Magazine](https://www.smashingmagazine.com/2021/05/tree-shaking-reference-guide/)
- [Vulture - Python Dead Code](https://github.com/jendrikseipp/vulture)
- [R8 Code Shrinking - LogRocket](https://blog.logrocket.com/r8-code-shrinking-android-guide/)
- [ReSharper Solution-Wide Analysis](https://www.jetbrains.com/help/resharper/Code_Analysis__Solution-Wide_Analysis__Solution-Wide_Code_Inspections.html)
- [deptry - Python Dependencies](https://github.com/fpgmaas/deptry)
- [Knip - TypeScript Unused Dependencies](https://knip.dev/typescript/unused-dependencies)
- [Dead Code Detection Techniques - Aivosto](https://www.aivosto.com/articles/deadcode.html)
- [Call Graphs - Wikipedia](https://en.wikipedia.org/wiki/Call_graph)
- [Dead Code Elimination - Wikipedia](https://en.wikipedia.org/wiki/Dead-code_elimination)
- [Dead Code Removal at Meta - ACM](https://dl.acm.org/doi/10.1145/3611643.3613871)