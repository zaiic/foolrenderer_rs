# Illumination

We get zero illumination if the polygon is parallel to the vector of light
and get full illumination if it is perpendicular to the vector of light.

The intenstity of illumination equals to the scalar product of the vector of light
and the normal to the given triangle.

The normal to the given triangle can be calculated simply as the cross product of
its two sides. And, don't forget to normalize it to become
a normalized vector.

Pseudocode:

```cpp
vec3 light_dir = vec3(0, 0, -1); // light direction
vec3 points[3] = { vertices of a given triangle };

vec3 n = cross(points[1] - points[0], points[2] - points[0]);
n.normalize();

float illum = dot(light_dir, n);

if (illum > 0) {
    triangle(screen_coords, image, color(r*illum, g*illum, b*illum));
}
```

The value of `illum` can be negative. It means that the light comes from behind the polygon.
If the scene is well modeled, we can simply discard those triangles.
It is called [back-face culling](https://en.wikipedia.org/wiki/Back-face_culling).
