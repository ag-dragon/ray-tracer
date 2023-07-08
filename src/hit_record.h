#ifndef HITTABLE_H
#define HITTABLE_H

#include "ray.h"
#include <cglm/cglm.h>

typedef struct hit_record {
    vec3 p;
    vec3 normal;
    double t;
    bool front_face;
} hit_record;

void hit_record_init(hit_record *rec, vec3 p, vec3 normal, double t, bool front_face);
void hit_record_set_face_normal(hit_record *rec, ray *r, vec3 outward_normal);

#endif
