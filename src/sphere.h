#ifndef SPHERE_H
#define SPHERE_H

#include "ray.h"
#include "hit_record.h"
#include <cglm/cglm.h>
#include <stdbool.h>

typedef struct sphere {
    vec3 center;
    double radius;
} sphere;

void sphere_init(sphere *sphere, vec3 center, double radius);
bool sphere_collision(sphere *s, ray *r, double t_min, double t_max, hit_record *rec);

#endif
