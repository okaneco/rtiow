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

00-blue-blend.jpg  
![blue blend](img/first/00-blue-blend.jpg)  
01-red-sphere.jpg  
![red sphere](img/first/01-red-sphere.jpg)  
02-sphere-normals.jpg  
![sphere normals](img/first/02-sphere-normals.jpg)  
03-normal-sphere-ground.jpg  
![normal sphere ground](img/first/03-normal-sphere-ground.jpg)  
04-multi-sample.jpg  
![multi sample](img/first/04-multi-sample.jpg)  
05-diffuse.jpg  
![diffuse](img/first/05-diffuse.jpg)  
06-diffuse-gamma-correct.jpg  
![diffuse gamma correct](img/first/06-diffuse-gamma-correct.jpg)  
07-diffuse-random-unit-vector.jpg  
![diffuse random unit vector](img/first/07-diffuse-random-unit-vector.jpg)  
08-materials-lambert.jpg  
![materials lambert](img/first/08-materials-lambert.jpg)  
09-metals.jpg  
![metals](img/first/09-metals.jpg)  
10-all-refract.jpg  
![all refract](img/first/10-all-refract.jpg)  
11-sometimes-refract.jpg  
![sometimes refract](img/first/11-sometimes-refract.jpg)  
12-bubble.jpg  
![bubble](img/first/12-bubble.jpg)  
13-snell.jpg  
![snell](img/first/13-snell.jpg)  
14-wide-view.jpg  
![wide view](img/first/14-wide-view.jpg)  
15-distant-view.jpg  
![distant view](img/first/15-distant-view.jpg)  
16-zoom-view.jpg  
![zoom view](img/first/16-zoom-view.jpg)  
17-dof.jpg  
![dof](img/first/17-dof.jpg)  
18-final-scene.jpg  
![final scene](img/first/18-final-scene.jpg)  


<a name="nextweek"></a>
## *The Next Week*


<a name="restofyourlife"></a>
## *The Rest of Your Life*
