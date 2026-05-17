# Social Preview Image Guide

This document explains how to create and set up the social preview image for the Chazer repository.

## Dimensions

GitHub recommends:
- **Optimal**: 1280 × 640 pixels (2:1 aspect ratio)
- **Minimum**: 640 × 320 pixels
- **File size**: Under 1 MB
- **Format**: PNG, JPG, or GIF

## Current Design Elements

The social preview should include:
- Chazer logo
- Project name and tagline
- Key visual elements (code snippets, terminal output, etc.)
- Brand colors

## How to Set Up

1. Go to your repository on GitHub
2. Click **Settings**
3. Scroll to **Social preview** section
4. Click **Edit** → **Upload an image**
5. Select your 1280×640 image

## Design Suggestions

### Option 1: Terminal-Style
```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│     [Logo]  Chazer                                      │
│                                                                 │
│     Find and eliminate dead code                                │
│     in Android projects                                         │
│                                                                 │
│     $ chazer ./my-app                                   │
│     Found 12 dead code issues ✓                                 │
│                                                                 │
│     ⚡ Fast  •  🔒 Safe Delete  •  🎯 Kotlin-first             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Option 2: Code-Style
Show a before/after of dead code being detected with colorful syntax highlighting.

### Option 3: Stats-Focused
Highlight key stats:
- "Analyze 10k files in seconds"
- "51% fewer false positives"
- "Hybrid analysis: Static + Dynamic"

## Tools for Creating

- [Figma](https://figma.com) - Free design tool
- [Canva](https://canva.com) - Easy templates
- [Excalidraw](https://excalidraw.com) - Hand-drawn style
- [Carbon](https://carbon.now.sh) - Beautiful code screenshots

## Placeholder

Until a custom image is created, GitHub will auto-generate a preview from the repository content.
