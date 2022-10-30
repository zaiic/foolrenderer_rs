# Rasterization

## Old-school method: Line sweeping

```cpp
void triangle(Vec2i t0, Vec2i t1, Vec2i t2, TGAImage &image, TGAColor color) { 
    // sort the vertices, t0, t1, t2 lower−to−upper (bubblesort yay!) 
    if (t0.y>t1.y) std::swap(t0, t1); 
    if (t0.y>t2.y) std::swap(t0, t2); 
    if (t1.y>t2.y) std::swap(t1, t2); 
    int total_height = t2.y-t0.y; 
    for (int y=t0.y; y<=t1.y; y++) { 
        int segment_height = t1.y-t0.y+1; 
        float alpha = (float)(y-t0.y)/total_height; 
        float beta  = (float)(y-t0.y)/segment_height; // be careful with divisions by zero 
        Vec2i A = t0 + (t2-t0)*alpha; 
        Vec2i B = t0 + (t1-t0)*beta; 
        image.set(A.x, y, red); 
        image.set(B.x, y, green); 
    } 
}
```

Note that the `alpha` & `beta` are proportions of similar triangles.

## Better mathod for multithread processor

### Barycentric coordinate system

```
A                                      B
*--------------------------------------*
a                                      b

A         P                            B
*---------*----------------------------*
a                                      b
```

There is a `line(A, B)`, and it's vertices have different weight.

We assume they are equal to, $a$ and $b$, respectively. So, if we want to find out the barycenter
of the line. We probably know that the barycenter $P$ well may be at the side closing A.

$$
\frac{AP}{AB} + \frac{PB}{AB} = 1
$$

We assume $\frac{AP}{AB}$ equals to $i$ and $\frac{PB}{AB}$ equals to $j$.

Then $i + j = 1$.

Similarly, in the 3D space, the triangle's barycenter named $P$, then 
$P = i \overrightarrow{PA} + j\overrightarrow{PB} + k\overrightarrow{PC}$,
and $i + j + k = 1$.

If we regard $\overrightarrow{PA}$ , $\overrightarrow{PB}$ , and $\overrightarrow{PC}$ as the basis of a coordinate system, we would call system
a **barycentric coordinate system**.

Rename $(i,j,k)$ to $(w, u, v)$ and change the equation to:

$$
w = 1 - u - v
$$

Then the barycentric coordinates of point $P$ become $(1-u-v, u, v)$ .

### Calculate the value of *u* & *v*

We can find a linear system of two equations with two variables:

$$
\left\{\begin{matrix}
 u\overrightarrow{AB_x} + v\overrightarrow{AC_x} + \overrightarrow{PA_x} = 0\\
 u\overrightarrow{AB_y} + v\overrightarrow{AC_y} + \overrightarrow{PA_y} = 0\\
\end{matrix}\right.
$$

We can rewrite it in matrix form:

$$
\left\{\begin{matrix}
\begin{bmatrix}
 u & v & 1
\end{bmatrix}
\begin{bmatrix}
 \overrightarrow{AB_x}\\
 \overrightarrow{AC_x}\\
 \overrightarrow{PA_x}
\end{bmatrix} = 0\\\\
\begin{bmatrix}
 u & v & 1
\end{bmatrix}
\begin{bmatrix}
 \overrightarrow{AB_y}\\
 \overrightarrow{AC_y}\\
 \overrightarrow{PA_y}
\end{bmatrix} = 0\\
\end{matrix}\right.
$$

Let us program a new [code](https://github.com/ssloy/tinyrenderer/wiki/Lesson-2:-Triangle-rasterization-and-back-face-culling):

```cpp
#include <vector> 
#include <iostream> 
#include "geometry.h"
#include "tgaimage.h" 
 
const int width  = 200; 
const int height = 200; 
 
Vec3f barycentric(Vec2i *pts, Vec2i P) { 
    Vec3f u = Vec3f(pts[2][0]-pts[0][0], pts[1][0]-pts[0][0], pts[0][0]-P[0])^Vec3f(pts[2][1]-pts[0][1], pts[1][1]-pts[0][1], pts[0][1]-P[1]);
    /* `pts` and `P` has integer value as coordinates
       so `abs(u[2])` < 1 means `u[2]` is 0, that means
       triangle is degenerate, in this case return something with negative coordinates */
    if (std::abs(u.z)<1) return Vec3f(-1,1,1);
    return Vec3f(1.f-(u.x+u.y)/u.z, u.y/u.z, u.x/u.z); 
} 
 
void triangle(Vec2i *pts, TGAImage &image, TGAColor color) { 
    Vec2i bboxmin(image.get_width()-1,  image.get_height()-1); 
    Vec2i bboxmax(0, 0); 
    Vec2i clamp(image.get_width()-1, image.get_height()-1); 
    for (int i=0; i<3; i++) { 
        bboxmin.x = std::max(0, std::min(bboxmin.x, pts[i].x));
	bboxmin.y = std::max(0, std::min(bboxmin.y, pts[i].y));

	bboxmax.x = std::min(clamp.x, std::max(bboxmax.x, pts[i].x));
	bboxmax.y = std::min(clamp.y, std::max(bboxmax.y, pts[i].y));
    } 
    Vec2i P; 
    for (P.x=bboxmin.x; P.x<=bboxmax.x; P.x++) { 
        for (P.y=bboxmin.y; P.y<=bboxmax.y; P.y++) { 
            Vec3f bc_screen  = barycentric(pts, P); 
            if (bc_screen.x<0 || bc_screen.y<0 || bc_screen.z<0) continue; 
            image.set(P.x, P.y, color); 
        } 
    } 
} 
 
int main(int argc, char** argv) { 
    TGAImage frame(200, 200, TGAImage::RGB); 
    Vec2i pts[3] = {Vec2i(10,10), Vec2i(100, 30), Vec2i(190, 160)}; 
    triangle(pts, frame, TGAColor(255, 0, 0)); 
    frame.flip_vertically(); // to place the origin in the bottom left corner of the image 
    frame.write_tga_file("framebuffer.tga");
    return 0; 
}
```

In this code, the `barycentric` function compute the barycentric coordinates of 
Point $P$ . The cartesian coordinates of point $P$ come from the `triangle` function. To determine whether the cartesian coordinates of point $P$ are in the
triangle by determining whether the value of the barycentric coordinates of it has a negative value.

The `triangle` function also has a clipping of the bounding box with the
screen rectangle to reduce the CPU load for the triangles outside of the
screen.
