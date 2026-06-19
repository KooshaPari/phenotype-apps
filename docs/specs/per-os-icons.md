# Per-OS Icon Spec — FocalPoint

**Status:** PROPOSED (no assets generated yet)
**Date:** 2026-06-04
**Author:** L1 worker (apps registry audit)

## Why

FocalPoint is iOS-first, but the Phenotype design system requires per-OS icon
variants so any cross-platform surface (macOS dock via Catalyst/UniFFI, web
favicon, Windows companion shell, Playwright visual tests) has a properly-tuned
mark. The current iOS app shell ships placeholder asset-catalog slots only.

## Base vector

Hand-authored SVG at `assets/brand/logo.svg` (path-data only, no rasters).
Concept: a focus/iris mark — concentric rings + a center "anchor dot" + connector
nubs at 4 cardinal points. The same vector is reused across all OS variants;
rasterization is deterministic from the SVG.

## Per-OS variants

| OS         | File                                                          | Size          | Notes |
|------------|---------------------------------------------------------------|---------------|-------|
| iOS        | `apps/ios/FocalPoint/Assets.xcassets/AppIcon.appiconset/icon-1024.png` | 1024x1024 | Apple App Store + on-device |
| iOS        | `apps/ios/FocalPoint/Assets.xcassets/AppIcon.appiconset/icon-180.png`  | 180x180   | iPhone @3x |
| iOS        | `apps/ios/FocalPoint/Assets.xcassets/AppIcon.appiconset/icon-120.png`  | 120x120   | iPhone @2x |
| macOS      | `assets/brand/macos/icon-1024.icns`                            | 1024x1024     | Liquid-glass material language |
| macOS      | `assets/brand/macos/icon-512.png`                              | 512x512       | Fallback PNG |
| Android    | `assets/brand/android/icon-512.png`                            | 512x512       | Adaptive foreground (legacy compat) |
| Android    | `assets/brand/android/icon-foreground-432.png`                 | 432x432       | Adaptive icon foreground (108x108 @4x) |
| Android    | `assets/brand/android/icon-background.png`                     | 432x432       | Adaptive icon background |
| Windows    | `assets/brand/windows/icon.ico`                                | 16/32/48/256  | Multi-resolution |
| Windows    | `assets/brand/windows/icon-256.png`                            | 256x256       | Mica material language |
| Web        | `apps/web/public/favicon.ico`                                  | 16/32/48      | Browser tab |
| Web        | `apps/web/public/icon-512.png`                                 | 512x512       | PWA / social |
| Cross      | `assets/brand/logo.svg`                                        | vector        | Hand-authored source-of-truth |

## Material languages (per platform_icon_materials doctrine)

- **iOS / macOS liquid-glass**: stacked colored-glass layers, frosted blur, light refraction edge. NOT flat teal.
- **Android neo-glassmorphic / geisty**: depth, 3D card-like composition, light + shadow. NOT flat glyph.
- **Windows Mica / stacked-stained-mica**: translucent layered panes with backplate tint.
- **Web**: glassmorphic card, depth shadow.

## AI-DD + renderers

- AI-CODED (hand-authored SVG, paths written by hand). Rasters = deterministic resvg export.
- ICO assembly: ImageMagick fallback, else Pillow.
- .icns: iconutil (mac) or png2icns.
- PNG sizes: 16, 32, 48, 64, 128, 256, 512, 1024 as needed.

## Out of scope (this PR)

- Generating the actual SVG / PNG / ICO / ICNS files (no asset toolchain in this PR).
- This PR adds ONLY the spec doc. A follow-up PR will land the assets.

## Open questions

- Does FocalPoint ever ship a Catalyst macOS app, or stay iOS-only? (decides whether macOS variant is needed now.)
- Is there a web frontend (`apps/web/`) in plan, or is the Go backend headless? (decides web favicon timing.)
