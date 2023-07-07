#ifndef RAY_H
#define RAY_H

#include <cglm/cglm.h>

typedef struct ray {
    vec3 origin;
    vec3 direction;
} ray;

void ray_at(const ray *r, double t, vec3 dest);

#endif
