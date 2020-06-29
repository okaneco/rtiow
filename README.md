# rtiow - Ray Tracing in One Weekend

![book cover](img/first/18-final-scene.jpg)  
Rust port of RTIOW by Peter Shirley, v3.1.2, 2020-06-03  
https://raytracing.github.io/

Completed books are tagged as a release.

- [x] - *Ray Tracing in One Weekend*
- [ ] - *The Next Week*
- [ ] - *The Rest of Your Life*

### Table of Contents
1. [*Ray Tracing in One Weekend*](#oneweekend)
2. [*The Next Week*](#nextweek)
3. [*The Rest of Your Life*](#restofyourlife)

### Notes

I've stayed close to the book so far. I may make changes to make my life easier,
including changing the architecture to suit the language or using `rayon`.

<a name="oneweekend"></a>
## *Ray Tracing in One Weekend*

A primitive command line interface exists, scenes and threading need to be adjusted by recompiling. Each flag is optional. It defaults to 100 samples and
384 pixel width, height will be calculated with an aspect ratio of 16:9 if not
specified. Arbitrary aspect ratios are supported.

```
cargo run --release -- [filename.ppm] [samples] [width] [height]
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


<a name="nextweek"></a>
## *The Next Week*

Chapter 2: Bouncing Spheres, simulating motion blur  
![bouncing spheres](img/second/00-bouncing-spheres.jpg)  


<a name="restofyourlife"></a>
## *The Rest of Your Life*
