# Voysys egui 0.27.2 patch

This branch starts at upstream tag `0.27.2` (commit
`014327e36535deeca0839b6aca2646191f2bd2fb`). It carries the tracked-pointer
click policy previously shipped in Oden's vendored egui crate.

`Style::interaction.click_on_release` is opt-in and defaults to false. Click-only
widgets retain the widget that received the press, despite pointer motion or a
long hold. Releasing over that same enabled, unclipped widget clicks it. Releasing
elsewhere, losing the pointer, or dragging a parent cancels the click. Drag-capable
widgets retain egui's existing behavior. Pointer positions are never modified.

The behavior changes are confined to `crates/egui/src/context.rs`,
`interaction.rs` and `style.rs`. Seven regressions live in
`crates/egui/tests/click_on_release.rs`. These files are copied unchanged from
Oden's validated patch. Upstream MIT and Apache-2.0 licenses are retained.

This branch is intended for consumers patching the `egui` crate. Its `epaint`
dependency uses crates.io 0.27.2 instead of the upstream workspace path, matching
the previously vendored package and avoiding a second Git-sourced type identity
for epaint/emath/ecolor in Oden. This is not a fork of the whole rendering stack.

Oden enables the policy for Android plugin GUIs. Desktop keeps the default.
Consumers should pin a full Git commit in `[patch.crates-io]`, shared by every
workspace that exchanges egui types. Keep the patch until an equivalent upstream
policy exists; upgrading egui requires reviewing and rerunning these regressions.

Validation: `cargo test --locked -p egui --lib --test click_on_release` passes
on Rust 1.72.0 and the installed stable toolchain: 13 unit tests and all seven
tracked-pointer regressions. The rest of the upstream demo/integration workspace
is outside this consumer patch's validation scope.
