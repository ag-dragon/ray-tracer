#ifndef RAY_H
#define RAY_H

#include <cglm/cglm.h>

typedef struct Ray {
    vec3 origin;
    vec3 direction;
} Ray;

void ray_at(const Ray *r, double t, vec3 dest);

#endif
