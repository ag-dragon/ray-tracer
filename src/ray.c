#include "ray.h"

void ray_at(const Ray *r, double t, vec3 dest) {
    vec3 scale = {r->direction[0], r->direction[1], r->direction[2]};
    glm_vec3_scale(scale, t, scale);
    glm_vec3_add((vec3) {r->origin[0], r->origin[1], r->origin[2]}, scale, dest);
}
