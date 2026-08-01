---
slug: accessibility-permission-model
kind: domain
title: Accessibility permission model (rcmdb vs osascript)
description: Which binary needs macOS Accessibility (TCC) permission for which feature, and how the rcmdb binary surfaces the grant prompt.
keywords: [TCC accessibility permission rcmdb, AXIsProcessTrustedWithOptions prompt, kAXTrustedCheckOptionPrompt, rcmdb accessibility subcommand, osascript grant, cdhash re-approval, System Settings Privacy Accessibility]
created: 2026-08-01
modified: 2026-08-01
---

# Accessibility permission model (rcmdb vs osascript)

Window cycling requires Accessibility (TCC) granted to the **`rcmdb`** binary;
previously it rode on `osascript`'s grant. Center Mouse (non-cycling) still uses
`osascript`, so the two features now depend on **different binaries' grants**.

## Rules

| Feature | Binary needing Accessibility |
|---------|------------------------------|
| Window cycling (`rcmdb cycle-window`) | `rcmdb` |
| Center Mouse (non-cycling, `center-mouse.sh`) | `osascript` |

- **GOTCHA** — a CLI binary invoked from a script gets **NO automatic TCC prompt**,
  and untrusted AX calls fail **silently** (cycling just does nothing; first-press
  `open -b` focus still works, which masks the problem). The only way to surface the
  dialog is calling `AXIsProcessTrustedWithOptions` with
  `kAXTrustedCheckOptionPrompt=true` — `prompt_accessibility_if_needed()` does this
  on first press.
- New observable onboarding command **`rcmdb accessibility`** triggers the prompt and
  prints granted/not-granted (+ the binary path to add manually via System Settings →
  Privacy & Security → Accessibility → +). README + CHANGELOG document it.
- On release, the grant is keyed to the binary's **cdhash**; a new build
  (e.g. 0.6.2 universal) requires **re-approval**.
