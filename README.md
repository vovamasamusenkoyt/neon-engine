# Neon Engine

A high-performance, modular cross-platform 3D graphics engine written in Rust with Vulkan.

Designed from the ground up for both Linux desktop and Android mobile hardware, utilizing direct Vulkan swapchains, hardware depth testing, and zero-overhead platform abstractions.

## Architecture

```
neon-engine/
├── crates/
│   ├── platform/           # Platform abstraction layer
│   │   ├── src/android.rs  # Android NativeActivity event loop & windowing
│   │   └── src/linux.rs    # Linux/Desktop winit event loop & windowing
│   ├── engine-core/        # Camera matrices, scene math (glam), frame timers
│   ├── engine-render/      # Vulkano graphics pipeline, swapchain, depth buffer, HUD
│   ├── asset-loader/       # Mesh data representations and buffer builders
│   └── neon-app/           # Executable runner (desktop binary & Android cdylib)
├── scripts/
│   └── build-android.sh    # Automated build, package, sign and deploy tool
└── AndroidManifest.xml     # NativeActivity manifest configuration
```

## Features

- **Direct Vulkan Pipeline**: Powered by `vulkano 0.35` with raw window handle integration.
- **Unified Platform Crate**: Single `platform` crate with conditional backends (`platform/src/android.rs` and `platform/src/linux.rs`).
- **Hardware Depth Testing**: Dynamic native format negotiation (`D24_UNORM_S8_UINT`, `D32_SFLOAT`, `D16_UNORM`) with correct Vulkan NDC coordinate mapping.
- **Uncapped Mailbox Swapchain**: Delivers ultra-low latency rendering (tested at 2200+ FPS / 0.45 ms per frame on Adreno 640).
- **Integrated Vector HUD**: High-frequency on-screen vector text overlay rendered without depth buffer write overhead.

## Building and Running

### Linux Desktop

```bash
cargo run -p neon-app
```

### Android (ARM64)

Prerequisites:
- Android NDK (`aarch64-linux-android`)
- Android SDK build tools (`aapt2`, `zipalign`, `apksigner`)
- Connected device via `adb`

Run the deployment script:

```bash
./scripts/build-android.sh
```

## License

MIT
