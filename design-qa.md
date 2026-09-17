# Design QA

**Source visual truth:** `/home/admin/.codex/generated_images/01a0b08a-d3dd-7571-8e79-101f5a5454a2/exec-c31ddc5f-e262-4e08-94c7-453ef642b939.png`  
**Implementation screenshot:** `docs/screenshots/onboarding.png`

- Viewport and implementation pixels: 1024 × 680 at 1× density.
- Source pixels: 1536 × 1024, normalized to 1024 × 680 for comparison.
- State: onboarding lesson 1 of 5, focused window, `Super + Q` held, successful attempt.
- Interaction tested: focused-window key down, scoring, live held-state keycaps, last-attempt retention, and next-step affordance.
- Runtime errors: none. Xvfb emitted a harmless missing `XDG_RUNTIME_DIR` warning.

## Findings

No actionable P0, P1, or P2 differences remain. The implementation preserves the source hierarchy: persistent navigation, onboarding rail, shortcut instruction, live key visualization, event labels, retained last attempt, feedback, and a visible next action.

Typography uses Iced's native system-font rendering rather than the mock's display font. Colors use the existing Tokyo Night theme, with restrained borders instead of the mock's decorative glow. These are acceptable P3 differences that keep the native UI dependency-free. The mock's custom icons are intentionally omitted rather than approximated with emoji or text glyphs. Copy reflects the loaded test configuration.

Focused-region comparison was not needed: all key labels, status text, and controls are legible in the full-size 1024 × 680 capture. No raster assets are used by the application UI.

## Comparison History

1. Initial capture exposed a P1 navigation treatment mismatch and an overly bright onboarding panel. Fixed with text-style navigation, a single active item, and a dark bordered panel.
2. The first post-input capture exposed a P2 vertical overflow: adding “Last attempt” compressed the next button. Removed the duplicate expected-key row; the final capture shows live input, last attempt, feedback, and the complete next button above the fold.

## Follow-up Polish

- P3: add a purpose-built vector logo and icon set only when project-owned assets exist.
- P3: add subtle keycap elevation if native rendering remains clear across themes.

**final result: passed**
