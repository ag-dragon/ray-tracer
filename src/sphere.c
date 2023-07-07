#include "sphere.h"
#include <math.h>

void sphere_init(sphere *sphere, const vec3 center, double radius) {
    sphere->center[0] = center[0];
    sphere->center[1] = center[1];
    sphere->center[2] = center[2];
    sphere->radius = radius;
}

bool sphere_collision(const sphere *s, const ray *r, double t_min, double t_max, hit_record *rec) {
    vec3 center_copy = {s->center[0], s->center[1], s->center[2]};
    vec3 origin = {r->origin[0], r->origin[1], r->origin[2]};
    vec3 direction = {r->direction[0], r->direction[1], r->direction[2]};
    vec3 oc;
    glm_vec3_sub(origin, center_copy, oc);
    double a = glm_vec3_norm2(direction);
    double half_b = glm_vec3_dot(oc, direction);
    double c = glm_vec3_norm(oc) - s->radius*s->radius;

    double discriminant = half_b*half_b - a*c;
    if (discriminant < 0) return false;
    double sqrtd = sqrt(discriminant);

    double root = (-half_b - sqrtd) / a;
    if (root < t_min || t_max < root) {
        root = (-half_b + sqrtd) / a;
        if (root < t_min || t_max < root) {
            return false;
        }
    }

    rec->t = root;
    ray_at(r, rec->t, rec->p);
    glm_vec3_sub(rec->p, center_copy, rec->normal);
    glm_vec3_divs(rec->normal, s->radius, rec->normal);

    return true;
}
