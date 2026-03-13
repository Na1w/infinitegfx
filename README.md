<div align="center">
  <img src="assets/logo.svg" alt="InfiniteGFX Logo" width="600">
</div>

# InfiniteGFX Core

[![Rust](https://github.com/Na1w/infinitegfx/actions/workflows/rust.yml/badge.svg)](https://github.com/Na1w/infinitegfx/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/infinitegfx-core.svg)](https://crates.io/crates/infinitegfx-core)
[![Documentation](https://docs.rs/infinitegfx-core/badge.svg)](https://docs.rs/infinitegfx-core)

A modular, flexible, and shader-based graphics library for Rust, built on `wgpu`. 
Designed for real-time effects, transitions, and reactive graphics.

## Features

*   **Shader-Centric Design:** Leverage the power of WGSL shaders for all effects and transitions, allowing for highly customizable and unique visual styles.
*   **WGPU-based:** Hardware-accelerated rendering with support for Vulkan, Metal, DX12, and WebGPU (WASM).
*   **Modular Architecture:** Build complex graphics pipelines using `GfxChain`.
*   **Reactive Parameter System:** All effect parameters can be static, linked to thread-safe controls (`atomics`), or driven by modulators (LFOs, Envelopes).
*   **Flexibility:** Swap shaders, chain effects, and modulate any parameter at runtime.
*   **Audio Integration:** Built-in `AudioBridge` to easily drive graphics parameters from audio signals (e.g., from `infinitedsp-core`).
*   **Cross-platform:** Designed from the start to work with WASM, Windows, Linux, and macOS.
*   **Current Effect Suite:**
    *   **Transitions:** Crossfade, Flip, Warp, Space-Bend, Curtain.
    *   **Effects:** Glitch, Haze, Sparkle, Glass, Fade, Solid Color.
    *   **Text:** Basic text rendering (hard-coded minimal font) with support for animations (explosion, gravity, wipe, modulation).
*   **Modulators:**
    *   **LFO:** RampingLfo, LinearSweep for periodic changes.
    *   **Envelope:** KickPumper for rhythmic reactivity.
    *   **Audio:** AudioBridge for connecting to audio levels.

## Project Structure

*   `src/core`: Core components like `GfxChain`, `GfxParam`, `RenderContext`, and `GfxFrameProcessor`.
*   `src/effects`: Implementations of graphical effects and transitions.
*   `src/modulators`: Modulators for controlling parameters over time or via external sources.
*   `src/backend`: Management of `wgpu` resources, devices, and surfaces.
*   `src/shaders`: WGSL shaders for all effects and transitions.

## Usage

Add `infinitegfx-core` to your dependencies.

```rust
use infinitegfx_core::core::gfx_chain::GfxChain;
use infinitegfx_core::effects::{glitch, haze, text::TextEffect};
use infinitegfx_core::modulators::lfo::RampingLfo;
use infinitegfx_core::StandardGlobals;

// Create an effect chain
let mut chain = GfxChain::new(format);

// Add a text effect with animation
let text = TextEffect::new("INFINITEGFX", 0.0, 10.0)
    .with_pos(0.5, 0.5)
    .with_explosion(RampingLfo::new(0.5, 1.0));

// Add glitch and haze
chain = chain
    .and(text)
    .and(glitch(0.2, 0.5))
    .and(haze(0.1, 0.3));

// In your render loop:
let globals = StandardGlobals {
    time,
    res_x: width as f32,
    res_y: height as f32,
    ..Default::default()
};

// Update global buffer and render
queue.write_buffer(globals_buf, 0, bytemuck::cast_slice(&[globals]));

chain.render(RenderContext {
    device,
    queue,
    encoder,
    target_view: view,
    globals_bind_group,
    globals_buf,
    time,
    input_view: None,
});
```

## Documentation

To generate and view the API documentation:

```sh
cargo doc --open
```

## AI Contribution Policy

This project encourages experimentation with AI agents for code generation and optimization. However, all AI-generated contributions must be strictly verified by a human maintainer. This verification includes:
1.  **Code Review:** Ensuring the code is idiomatic, safe, and follows project standards.
2.  **Visual Verification:** Checking the visual output to ensure correctness and graphical quality (no artifacts, correct shader behavior).
3.  **Platform Support:** Because this project targets multiple platforms, verification should be done on your native platform and ideally also via Google Chrome/WGPU.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
