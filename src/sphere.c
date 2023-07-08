#include "sphere.h"
#include <math.h>

void sphere_init(sphere *sphere, vec3 center, double radius) {
    glm_vec3_copy(center, sphere->center);
    sphere->radius = radius;
}

bool sphere_collision(sphere *s, ray *r, double t_min, double t_max, hit_record *rec) {
    vec3 oc;
    glm_vec3_sub(r->origin, s->center, oc);
    double a = glm_vec3_norm2(r->direction);
    double half_b = glm_vec3_dot(oc, r->direction);
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
    vec3 outward_normal;
    glm_vec3_sub(rec->p, s->center, outward_normal);
    glm_vec3_divs(rec->normal, s->radius, outward_normal);
    hit_record_set_face_normal(rec, r, outward_normal);

    return true;
}
