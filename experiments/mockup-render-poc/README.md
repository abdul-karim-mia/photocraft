# Mockup rendering proof of concept

Status: experimental; no Photoshop parity or browser performance claim yet.

## Verified source entry points

- `crates/engine/src/smart_cmds.rs`: `replace_contents_bytes(session, layer_id, file_name, bytes)` decodes first, replaces shared Smart Object source, and refreshes instances while retaining their transforms and filter stacks.
- `crates/psd/src/header.rs`: PSD/PSB header versions.
- `crates/automation/src/headless.rs`: UI-free session and file operations.
- `apps/photocraft-web`: wasm32 browser application; **not** proof of a standalone JS rendering API.

## Minimal adapter contract (proposed; not implemented)

```ts
type RenderRequest = {
  template: Uint8Array;
  layerName: string;
  design: Uint8Array;
  designFileName: string;
  resizeMode: "fit" | "fill" | "none" | "stretch";
  position: "center" | "top" | "bottom" | "left" | "right";
  previewQuality?: "low" | "medium" | "high";
};
```

## Implementation sequence

1. Load template bytes through existing import/session code; reject malformed PSD/PSB.
2. Walk document layers and match exact layer name; fail on zero or ambiguous matches; require Smart Object for first milestone.
3. Call `smart_cmds::replace_contents_bytes` on matched layer, retaining existing transform. Determine resize policy relative to original Smart Object source dimensions **before** implementing fit/fill.
4. Composite document and encode PNG in memory. No filesystem dependency in browser adapter.
5. Run fixture tests with one simple PSD, one warped Smart Object PSD, one PSB, and one unsupported Photoshop filter. Record output dimensions, errors and visual diff against Photoshop.
6. Compile wasm32 separately; test browser memory, cold-start, rendering latency and repeated render behavior.

## Acceptance gates

- Replacement selects the intended layer by exact name and fails safely on duplicates.
- Original template remains unchanged; replacement produces a PNG.
- Perspective/warp, masks, effects, clipping and color fidelity are assessed against Photoshop references.
- Unsupported filters are surfaced as warnings or errors, not silently represented as equivalent.
- WASM compile and actual browser run pass on realistic file sizes.
- Third-party files never leave the browser during rendering unless integrator explicitly opts in.

## Known risks

- `replace_contents_bytes` already exists, but may detach preserved Photoshop metadata and re-render unsupported effects differently.
- Embedded and linked Smart Object sources have different availability/security behavior.
- Large PSB files can exhaust browser memory.
- License/NOTICE obligations must be reviewed before distributing the SDK.

Do not merge this experiment into `main` until actual fixtures and measurements exist.
