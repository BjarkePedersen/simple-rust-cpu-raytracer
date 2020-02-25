# Simple Rust CPU raytracer
A simple homebrewed, CPU-powered path tracer. It's not meant to be good path tracer by any means, It's meant to teach me Rust. Feel free to contribute.

## Controls
Movement: W A S D LShift Space

Look: Mouse

Zoom: Z X

Focus distance: J L

Aperture size: I M

Toggle autofocus: N

Toggle overlays: U

Toggle depth pass: Enter

## Preview
![alt text](https://i.imgur.com/Y5f9IJl.png)

## Missing features / To do / wishlist
  #### Optimization:
  - Inlining
  - Cache optimization
  #### UI:
  - Options menu
  - Improve Bresenham implementation
  - Possibly switch framebuffer
  #### Raytracing features:
  - Spectral rendering
  - Lens effects:
    - Logitudinal achromatic / chromatic aberration
    - Lateral chromatic abberation
    - Spherical aberration
    - Customizable bokeh shapes
  - GPU post-processing effects:
    - Bloom
  - Scrambled Sobol
  - Multiple importance sampling
  - Bidirectional path tracing
  - Refraction
  #### Stretch goals:
  - Metropolis light transport
    - (Automatic parameter control)
  - Customizable reflectance distribution functions
  - Polygon support
  - Translucency
  - Subsurface Scattering
  - Proper PBR implementation
  - Textures
  - Normal mapping
