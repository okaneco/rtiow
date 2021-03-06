<a name="top"></a>
# rtiow - Ray Tracing in One Weekend

![book0 cover](img/first/18-final-scene.jpg)  

![book1 cover](img/second/21-final-scene.jpg)  

![pdf for sphere](img/third/04-cornell-glass.jpg)  

Rust port of RTIOW by Peter Shirley, v3.1.2, 2020-06-03  
https://raytracing.github.io/

Completed books are tagged as a release.

- [x] - *Ray Tracing in One Weekend*
- [x] - *The Next Week*
- [x] - *The Rest of Your Life*

### Table of Contents
1. [*Ray Tracing in One Weekend*](#oneweekend)
2. [*The Next Week*](#nextweek)
3. [*The Rest of Your Life*](#restofyourlife)

### Notes

I believe I've stayed close to the spirit of the book while still writing Rust.
Multi-threading was trivially added with `rayon` for the last render of the
first book. All renders after that were multi-threaded.

<a name="oneweekend"></a>
## *Ray Tracing in One Weekend*

A primitive command line interface exists, scenes and threading need to be
adjusted by recompiling. Each flag is optional. Default values are 100 samples
and 384 pixel width, height will be calculated with an aspect ratio of 16:9 if
not specified. Arbitrary aspect ratios are supported.

```
cargo run --release -- [filename.ppm] [samples] [width] [height] [seed]
```

All images were done with 100 samples and 50 bounces.

Creating a blue to white background gradient  
![blue blend](img/first/00-blue-blend.jpg)  
Placing the first sphere in the scene  
![red sphere](img/first/01-red-sphere.jpg)  
Showing the normals of the sphere's surface  
![sphere normals](img/first/02-sphere-normals.jpg)  
Adding a ground plane with another large sphere  
![normal sphere ground](img/first/03-normal-sphere-ground.jpg)  
Multiple samples per pixel, anti-aliasing  
![multi sample](img/first/04-multi-sample.jpg)  
The first diffuse material  
![diffuse](img/first/05-diffuse.jpg)  
Gamma correcting the linear light  
![diffuse gamma correct](img/first/06-diffuse-gamma-correct.jpg)  
Improving the scattering calculation  
![diffuse random unit vector](img/first/07-diffuse-random-unit-vector.jpg)  
The first Lambert material  
![materials lambert](img/first/08-materials-lambert.jpg)  
Adding metallic materials  
![metals](img/first/09-metals.jpg)  
First pass of dielectric materials, light is only reflected  
![all refract](img/first/10-all-refract.jpg)  
Added the chance for refraction to occur  
![sometimes refract](img/first/11-sometimes-refract.jpg)  
Placed another sphere inside the dielectric to make a glass bubble effect  
![bubble](img/first/12-bubble.jpg)  
Added a Snell's law correction  
![snell](img/first/13-snell.jpg)  
Added camera controls to adjust the field of view  
![wide view](img/first/14-wide-view.jpg)  
Adjusting FOV for zooming out  
![distant view](img/first/15-distant-view.jpg)  
Zooming in with another FOV adjustment  
![zoom view](img/first/16-zoom-view.jpg)  
Depth of field blur is added  
![dof](img/first/17-dof.jpg)  
Final scene as on the cover of the book, with some personal touches added  
![final scene](img/first/18-final-scene.jpg)  

[Back to top](#top)

<a name="nextweek"></a>
## *The Next Week*

All images are 100 samples unless otherwise noted. With BVH and multi-threading,
sample count can be increased dramatically and still finish in a very tolerable
time.

I was better about saving the scenes in this, grouping the camera with it.
However, due to the nature of the book, enough incremental churn occurs that
it's not convenient to keep every camera, world, and rendering combination
pictured here.

I made an enum for the Perlin noise to allow for selection from any of the types
made over the course of the chapter: trilinear, unfiltered, net/camouflage,
smooth, and marble (with turbulence).

<a name="chapter2"></a>
**Chapter 2:** Bouncing Spheres, simulating motion blur  
![bouncing spheres](img/second/00-bouncing-spheres.jpg)  
<a name="chapter4"></a>
**Chapter 4:** Added a checker texture to the ground,
implemented bounding volume hierarchies for massive render speedup in some
scenes - 400 samples  
![checkerboard floor](img/second/01-checker-world.jpg)  
Two checker spheres  
![checkered spheres](img/second/02-checker-spheres.jpg)  
<a name="chapter5"></a>
**Chapter 5:** Hashed Perlin noise  
![perlin noise squares](img/second/03-perlin-spheres.jpg)  
Playing with the previous scene, added motion blur to one sphere  
![perlin square with motion blur](img/second/04-perlin-spheres-motion.jpg)  
<a name="5.2"></a>
**5.2:** Perlin noise with trilinear interpolation.  
![perlin noise with trilinear filter](img/second/05-trilinear.jpg)  
<a name="5.3"></a>
**5.3:** Trilinear filtering with cubic Hermite  
![perlin trilinear filter smoothed](img/second/06-hermitian-smoothing.jpg)  
<a name="5.4"></a>
**5.4:** High frequency scaling for the noise, this is a scale of 20  
![perlin noise with higher frequency](img/second/07-frequency-scale-20.jpg)  
The book example seems to be a frequency of 4, determined through trial and
error  
![perlin with frequency to match book](img/second/08-frequency-scale-04.jpg)  
<a name="5.5"></a>
**5.5** Perlin noise with random unit vectors on lattice points  
![perlin with random unit vectors](img/second/09-random-vectors-lattice-points.jpg)  
<a name="5.6"></a>
**5.6** Substituting turbulence in for the noise function, not the intended
result as in the book  
![perlin with turbluence](img/second/10-turbulence-substitution.jpg)  
Multiplying turbulence directly by the color as illustrated in the book  
![perlin with turbluence direct](img/second/11-turbulence-direct.jpg)  
<a name="5.7"></a>
**5.7** Adjusting the phase of turbulence, making a marble texture  
![perlin turbulent marble texture](img/second/12-marbled-texture.jpg)  
<a name="chapter6"></a>
**Chapter 6:** Using images as textures  
![earth on a sphere](img/second/13-earth.jpg)  
<a name="chapter7"></a>
**Chapter 7:** Turning objects into lights, small rectangle light  
![rectangle light](img/second/14-rectangle-light.jpg)  
Adding a sphere to the scene - 1,000 samples  
![sphere light](img/second/15-sphere-light.jpg)  
<a name="7.6"></a>
Empty Cornell box. Aspect ratio changed to 1:1. My result doesn't appear
to have flipped normals like in the book - 1,000 samples  
![sphere light](img/second/16-empty-box.jpg)  
Added flipped face material for less noise with axis-aligned planes - 1,000
samples  
![sphere light](img/second/17-flip-face.jpg)  
Cornell box, now with blocks but not rotated - 1,000 samples  
![sphere light](img/second/18-cornell-blocks.jpg)  
Standard Cornell box scene with rotated boxes - 1,000 samples  
![cornell box scene](img/second/19-cornell-rotated.jpg)  
Cornell with blocks of smoke - 1,000 samples  
![cornell smoke scene](img/second/20-cornell-smoke.jpg)  
Final scene, 400 samples. The scene's Perlin noise is not reproducible from
the instructions given in the book, there are some artifacts due to lack of
samples but render times are very long. I spent a few hours trying to figure out
why my result didn't look like the book. We do have nice volumetric fog and
caustics in the glass material though.  
See: https://github.com/RayTracing/raytracing.github.io/issues/425   
![testing everything](img/second/21-final-scene.jpg)  

[Back to top](#top)

<a name="restofyourlife"></a>
## *The Rest of Your Life*

This book focuses on different sampling methods and comparisons of the
Cornell box scene, so I've skipped their inclusion. Topics covered included
Monte Carlo integration and using probability density functions to sample the
lights and hittables in the scene directly.

Sampling the lights directly, finished in under a second - 10 samples  
![sample lights directly](img/third/00-sampling-lights-directly.jpg)  
Made the light one-sided, removed some noise near the light - 10 samples  
![unidirectional light](img/third/01-unidirectional-light.jpg)  
Added mixture PDF for mixing the contributions of lighting and reflection -
1,000 samples  
![mixture](img/third/02-mixture-pdf.jpg)  
Added specular to the model, metal block in the Cornell scene - 1,000 samples  
![specular metal box](img/third/03-cornell-metal.jpg)  
Sphere with new PDF functions - 1,000 samples  
![pdf for sphere](img/third/04-cornell-glass.jpg)  


[Back to top](#top)
