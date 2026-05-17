#!/bin/bash
# Chazer Pre-Commit Hook
# Blocks commits that introduce new dead code
#
# Installation:
#   cp scripts/pre-commit-hook.sh .git/hooks/pre-commit
#   chmod +x .git/hooks/pre-commit
#
# Or use with pre-commit framework (https://pre-commit.com):
#   Add to .pre-commit-config.yaml:
#   - repo: local
#     hooks:
#       - id: chazer
#         name: Chazer
#         entry: scripts/pre-commit-hook.sh
#         language: script
#         pass_filenames: false

set -e

# Configuration
MIN_CONFIDENCE="${CHAZER_MIN_CONFIDENCE:-high}"
FAIL_ON_FINDINGS="${CHAZER_FAIL_ON_FINDINGS:-true}"
BASELINE_FILE="${CHAZER_BASELINE:-.chazer-baseline.json}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Running Chazer...${NC}"

# Check if chazer is installed
if ! command -v chazer &> /dev/null; then
    echo -e "${RED}Error: chazer is not installed${NC}"
    echo "Install it with: brew tap dr7ro0t/tap && brew install chazer"
    echo "Or: cargo install chazer"
    exit 1
fi

# Build command
CMD="chazer . --min-confidence $MIN_CONFIDENCE --format json"

# Use baseline if it exists
if [ -f "$BASELINE_FILE" ]; then
    CMD="$CMD --baseline $BASELINE_FILE"
    echo "Using baseline: $BASELINE_FILE"
fi

# Run analysis
OUTPUT=$(eval "$CMD" 2>/dev/null || true)

# Count findings
FINDINGS=$(echo "$OUTPUT" | jq 'length' 2>/dev/null || echo "0")

if [ "$FINDINGS" -gt 0 ]; then
    echo -e "${RED}Found $FINDINGS dead code issue(s):${NC}"
    echo ""

    # Show summary of findings
    echo "$OUTPUT" | jq -r '.[] | "  \(.file):\(.line) - \(.name) (\(.issue_type))"' 2>/dev/null || echo "$OUTPUT"

    echo ""
    echo -e "${YELLOW}To see full details, run:${NC}"
    echo "  chazer . --min-confidence $MIN_CONFIDENCE"
    echo ""
    echo -e "${YELLOW}To generate a baseline (ignore existing issues):${NC}"
    echo "  chazer . --generate-baseline $BASELINE_FILE"
    echo ""

    if [ "$FAIL_ON_FINDINGS" = "true" ]; then
        echo -e "${RED}Commit blocked. Fix the issues above or update the baseline.${NC}"
        exit 1
    else
        echo -e "${YELLOW}Warning: Dead code found but CHAZER_FAIL_ON_FINDINGS=false${NC}"
    fi
else
    echo -e "${GREEN}No new dead code found.${NC}"
fi

exit 0
