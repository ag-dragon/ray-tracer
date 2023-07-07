#ifndef HITTABLE_H
#define HITTABLE_H

#include <cglm/cglm.h>

typedef struct hit_record {
    vec3 p;
    vec3 normal;
    double t;
} hit_record;

void hit_record_init(hit_record *rec, const vec3 p, const vec3 normal, double t);

#endif
