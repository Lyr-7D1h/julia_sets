# Julia Sets using Web GPU Specifications

# Requirements

- A driver supporting on of these graphics api's: Vulkan, Metal, D3D12, D3D11, and OpenGLES

# Why WGPU

I wanted to write in Rust and wgpu uses pure Rust, not some weird shotty c wrapper around opengl or similar. 

WGPU implements the Web GPU standard which is supported by a large amount of modern graphics Api's like Vulkan.


## OpenGL (OLD)

Opengl is a cross platform standard. Using opengl allows for a fairly low level implementation of a graphical application. Opengl has been around for a long time and good support on a lot of devices.

## GLFW vs SDL2

GLFW is sort of a subset of SDL2. GLFW just handles window creation and keyboard/mouse IO, whereas SDL2 does all these things but also has support for sound and a bunch of abstracted tools for drawing shapes and lines.


# Resources
https://rust-tutorials.github.io/learn-opengl
https://sotrh.github.io/learn-wgpu/
https://gpuweb.github.io/gpuweb/#intro
