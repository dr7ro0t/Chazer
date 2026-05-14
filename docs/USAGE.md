# 🔧 Usage

## Basic Analysis

```bash
# Analyze current directory
searchdeadcode .

# Analyze specific Android project
searchdeadcode ./my-android-app

# Analyze with verbose output
searchdeadcode ./app --verbose

# Quiet mode (only results)
searchdeadcode ./app --quiet
```

## Output Formats

```bash
# Terminal (default) - colored, grouped output
searchdeadcode ./app

# JSON - for programmatic use
searchdeadcode ./app --format json --output report.json

# SARIF - for GitHub Actions / CI integration
searchdeadcode ./app --format sarif --output report.sarif
```

## Hybrid Analysis (Static + Dynamic)

SearchDeadCode supports hybrid analysis by combining static code analysis with runtime coverage data. This significantly increases confidence in dead code findings and reduces false positives.

### Using Runtime Coverage

```bash
# With JaCoCo coverage from CI tests
searchdeadcode ./app --coverage build/reports/jacoco/test/jacocoTestReport.xml

# With Kover coverage (Kotlin)
searchdeadcode ./app --coverage build/reports/kover/report.xml

# With LCOV coverage
searchdeadcode ./app --coverage coverage/lcov.info

# Multiple coverage files (merged)
searchdeadcode ./app \
  --coverage build/reports/unit-test.xml \
  --coverage build/reports/integration-test.xml
```

### Confidence Levels

Each finding is assigned a confidence level:

| Level | Indicator | Description |
|-------|-----------|-------------|
| **Confirmed** | ● (green) | Runtime coverage confirms code is never executed |
| **High** | ◉ (bright green) | Private/internal code with no static references |
| **Medium** | ○ (yellow) | Default for static-only analysis |
| **Low** | ◌ (red) | May be false positive (reflection, dynamic dispatch) |

```bash
# Only show high-confidence and confirmed findings
searchdeadcode ./app --min-confidence high

# Only show runtime-confirmed findings (safest)
searchdeadcode ./app --coverage coverage.xml --runtime-only
```

### Runtime-Dead Code Detection

Find code that passes static analysis but is never executed in practice:

```bash
# Include reachable but never-executed code
searchdeadcode ./app --coverage coverage.xml --include-runtime-dead
```

This detects "zombie code" - code that exists in your codebase and appears to be used (passes static analysis) but is never actually executed during test runs.

## ProGuard/R8 Integration

Leverage ProGuard/R8's `usage.txt` for **confirmed** dead code detection. R8 performs whole-program analysis during release builds and identifies code it will remove.

### Generating usage.txt

Add to your `proguard-rules.pro`:
```
-printusage usage.txt
```

Then build your release APK:
```bash
./gradlew assembleRelease
```

The file will be at: `app/build/outputs/mapping/release/usage.txt`

### Using with SearchDeadCode

```bash
# Analyze with ProGuard data
searchdeadcode ./app --proguard-usage path/to/usage.txt

# Combine with other options
searchdeadcode ./app \
  --proguard-usage usage.txt \
  --coverage coverage.xml \
  --detect-cycles
```

### Real-World Example

```bash
# Full analysis with R8 usage.txt
./target/release/searchdeadcode /path/to/your/android-project \
  --exclude "**/build/**" \
  --exclude "**/test/**" \
  --exclude "**/Color.kt" \
  --exclude "**/Theme.kt" \
  --proguard-usage /path/to/your/android-project/app/usage.txt \
  --detect-cycles

# Output:
# 📋 ProGuard usage.txt: 106329 unused items (24593 classes, 55479 methods)
# 🧟 Zombie Code Detected: 1 dead cycle (2 declarations)
# Found 21 dead code issues:
#   ● 8 confirmed (matched with R8/ProGuard)
#   ○ 13 medium confidence
```

### Sample Output with ProGuard Integration

```
📋 ProGuard usage.txt: 106329 unused items (24593 classes, 55479 methods)

Found 21 dead code issues:

Confidence Legend:
  ● Confirmed (runtime) ◉ High
  ○ Medium ◌ Low

/app/src/main/java/com/example/app/admin/ui/SingleLiveEvent.kt
  ● 22:1 warning [DC001] class 'SingleLiveEvent' is never used (confirmed by R8/ProGuard)
    → class 'SingleLiveEvent'

/base/src/main/java/com/example/common/text/HtmlFormatterHelper.kt
  ● 7:1 warning [DC001] class 'HtmlFormatterHelper' is never used (confirmed by R8/ProGuard)
    → class 'HtmlFormatterHelper'

────────────────────────────────────────────────────────────
Summary: 21 warnings

By Confidence:
  ● 8 confirmed (0 runtime-confirmed)
  ○ 13 medium confidence
```

### What This Provides

| Benefit | Description |
|---------|-------------|
| **Confirmed findings** | Items in usage.txt are marked as `● Confirmed` |
| **Cross-validation** | Static analysis + R8 agreement = high confidence |
| **Library dead code** | R8 sees unused library code we can't analyze |
| **False positive detection** | `const val` objects may appear unused but are inlined |

### Important Notes

- **`const val` inlining**: Kotlin constants are inlined at compile time. The `Events` object may show as "unused" in usage.txt because only its values (not the object) are accessed at runtime. This is NOT dead code.
- **Build variants**: usage.txt is specific to release builds. Debug-only code won't appear.
- **Generated code**: Filter out `_Factory`, `_Impl`, `Dagger*`, `Hilt_*` classes.

## Zombie Code / Cycle Detection

Detect mutually dependent dead code - code that only references itself:

```bash
# Enable zombie code cycle detection
searchdeadcode ./app --detect-cycles
```

This finds patterns like:
- Class A uses Class B
- Class B uses Class A
- Neither A nor B is used by anything else

Example output:
```
🧟 Zombie Code Detected:
  2 dead cycles found (15 declarations)
  Largest cycle: 8 mutually dependent declarations
  3 zombie pairs (A↔B mutual references)

  Cycle #1 (8 items):
    • class 'LegacyHelper'
    • class 'LegacyProcessor'
    • method 'process'
    • method 'handle'
    ... and 4 more
```

## Unused Function Parameters Detection

Detect function parameters that are declared but never used within the function body:

```bash
# Enable unused parameter detection
searchdeadcode ./app --unused-params
```

This detector is conservative to minimize false positives:
- **Skips underscore-prefixed parameters** (`_unused`) - Kotlin convention for intentionally unused params
- **Skips override methods** - Parameters may be required by the interface
- **Skips abstract/interface methods** - No body to analyze
- **Skips @Composable functions** - Parameters used for recomposition
- **Skips constructors** - Parameters often used for property initialization
- **Skips callbacks/listeners** - `onXxx`, `*Listener`, `*Callback` patterns

## Unused Android Resources Detection

Detect Android resources (strings, colors, dimensions, styles, etc.) that are defined but never referenced in code or XML:

```bash
# Enable unused resource detection
searchdeadcode ./app --unused-resources
```

### How It Works

1. **Parses resource definitions** from all `res/values/*.xml` files:
    - `strings.xml` → `R.string.*`
    - `colors.xml` → `R.color.*`
    - `dimens.xml` → `R.dimen.*`
    - `styles.xml` → `R.style.*`
    - `attrs.xml` → `R.attr.*`

2. **Scans for references** in all Kotlin, Java, and XML files:
    - Code references: `R.string.app_name`, `R.color.primary`
    - XML references: `@string/app_name`, `@color/primary`

3. **Reports unused resources** with file location and resource type

### Example Output

```bash
$ searchdeadcode ./my-android-app --unused-resources

📦 Unused Android Resources:
  ○ app/src/main/res/values/strings.xml:21 - string 'unused_feature_text'
  ○ app/src/main/res/values/strings.xml:45 - string 'legacy_error_message'
  ○ app/src/main/res/values/colors.xml:12 - color 'deprecated_accent'
  ○ app/src/main/res/values/dimens.xml:8 - dimen 'old_margin_large'
  ○ app/src/main/res/values/styles.xml:15 - style 'LegacyButton'
  ○ base/src/main/res/values/attrs.xml:3 - attr 'customAttribute'

Found 6 unused resources (150 total defined, 320 referenced)
```

### Real-World Results

```bash
$ searchdeadcode /path/to/android-project --unused-resources

📦 Unused Android Resources:
  ○ app/src/main/res/values/admin_strings.xml:21 - string 'admin_apiMockAddressSaved'
  ○ app/src/main/res/values/appboy.xml:3 - string 'com_braze_api_key'
  ○ app/src/main/res/values/dimens.xml:8 - dimen 'card_sticky_audio_bottom_margin'
  ○ app/src/main/res/values/styles.xml:2 - style 'AppTheme.AppBarOverlay'
  ○ base/src/main/res/values/base_strings.xml:46 - string 'donation_button_text'
  ○ component-feed/src/main/res/values/feed_colors.xml:31 - color 'dates_light'
  ... and 47 more

Found 53 unused resources (672 total defined, 1142 referenced)
```

### Common False Positives to Ignore

Some resources may appear unused but are actually required:
- **Braze/Firebase SDK configs** (`com_braze_*`, `google_*`) - Read via reflection
- **Theme attributes** - May be referenced by parent themes
- **Build variant resources** - Only used in specific flavors

Use `--exclude` patterns or add to your config file:
```yaml
exclude:
  - "**/appboy.xml"
  - "**/google-services.xml"
```

## Deep Analysis Mode

For more aggressive dead code detection that analyzes individual members within classes:

```bash
# Enable deep analysis
searchdeadcode ./app --deep
```

### Terminal Output Example

```
Dead Code Analysis Results
==========================

com/example/app/utils/DeadHelper.kt
  ├─ class DeadHelper (line 5)
  │  Never instantiated or referenced
  └─ function unusedFunction (line 12)
     Never called

com/example/app/models/LegacyModel.kt
  └─ property debugFlag (line 8)
     Assigned but never read

Summary: 3 issues found
  - 1 unused class
  - 1 unused function
  - 1 assign-only property
```

### JSON Output Format

```json
{
  "version": "1.1",
  "total_issues": 21,
  "issues": [
    {
      "code": "DC001",
      "severity": "warning",
      "confidence": "confirmed",
      "confidence_score": 1.0,
      "runtime_confirmed": true,
      "message": "class 'DeadHelper' is never used (confirmed by R8/ProGuard)",
      "file": "com/example/app/utils/DeadHelper.kt",
      "line": 5,
      "column": 1,
      "declaration": {
        "name": "DeadHelper",
        "kind": "class",
        "fully_qualified_name": "com.example.app.utils.DeadHelper"
      }
    }
  ],
  "summary": {
    "errors": 0,
    "warnings": 21,
    "infos": 0,
    "by_confidence": {
      "confirmed": 8,
      "high": 0,
      "medium": 13,
      "low": 0
    },
    "runtime_confirmed_count": 8
  }
}
```

| Field | Description |
|-------|-------------|
| `code` | Issue code (DC001-DC007) |
| `confidence` | low, medium, high, confirmed |
| `confidence_score` | 0.25 to 1.0 for sorting |
| `runtime_confirmed` | True if coverage data confirms unused |
| `fully_qualified_name` | Package path when available |

## Filtering

```bash
# Exclude patterns (glob syntax)
searchdeadcode ./app --exclude "**/test/**" --exclude "**/generated/**"

# Retain patterns (never report as dead)
searchdeadcode ./app --retain "*Activity" --retain "*ViewModel"

# Combine multiple filters
searchdeadcode ./app \
  --exclude "**/build/**" \
  --exclude "**/*Test.kt" \
  --retain "*Repository" \
  --retain "*UseCase"
```

## Safe Delete

```bash
# Interactive deletion (confirm each item)
searchdeadcode ./app --delete --interactive

# Batch deletion (select from list, confirm once)
searchdeadcode ./app --delete

# Dry run (preview only, no changes)
searchdeadcode ./app --delete --dry-run

# Generate undo script for recovery
searchdeadcode ./app --delete --undo-script restore.sh
```

### Dry-Run Output Example

```
Dry run - would delete:
  class DeadHelper at com/example/utils/DeadHelper.kt:5
  function unusedMethod at com/example/Service.kt:42
  property debugFlag at com/example/Config.kt:8

Total: 3 items would be deleted
```