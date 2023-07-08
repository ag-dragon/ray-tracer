#ifndef WORLD_H
#define WORLD_H

#include "sphere.h"
#include "ray.h"
#include "hit_record.h"

typedef enum object_id {
    INVALID,
    SPHERE, 
} object_id;

typedef union object_data {
    sphere *s;
} object_data;

typedef struct object {
    object_id id;
    object_data data;
} object;

bool object_list_collision(object *object_list, int object_count, ray *r, double t_min, double t_max, hit_record *rec);

#endif
