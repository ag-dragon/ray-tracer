#include "hit_record.h"

void hit_record_init(hit_record *rec, const vec3 p, const vec3 normal, double t) {
    rec->p[0] = p[0];
    rec->p[1] = p[1];
    rec->p[2] = p[2];
    rec->normal[0] = normal[0];
    rec->normal[1] = normal[1];
    rec->normal[2] = normal[2];
    rec->t = t;
}
