# Julia Sets using OpenGL

# Requirements

- glfw

# Why OpenGL

Opengl is a cross platform standard. Using opengl allows for a fairly low level implementation of a graphical application. We don't need any fancy sound, ui or things like this. We just need to draw our art!

# GLFW vs SDL2 

GLFW is sort of a subset of SDL2. GLFW just handles window creation and keyboard/mouse IO, whereas SDL2 does all these things but also has support for sound and a bunch of abstracted tools for drawing shapes and lines.

That's why I decided on using GLFW as we only need minimal functionality.

# Resources
https://rust-tutorials.github.io/learn-opengl/basics/000-creating-a-window.html