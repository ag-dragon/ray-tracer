#include "ray.h"

void ray_at(ray *r, double t, vec3 dest) {
    glm_vec3_scale(r->direction, t, dest);
    glm_vec3_add(r->origin, dest, dest);
}
