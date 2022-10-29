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

Similarly, in the 3D space, the triangle's barycenter named P, then 
$P = i \overrightarrow{PA} + j\overrightarrow{PB} + k\overrightarrow{PC}$,
and $i + j + k = 1$.

If we regard them as the basis of a coordinate system, we would call system
a barycentric coordinate system.
