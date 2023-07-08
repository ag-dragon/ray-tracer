#include "hit_record.h"

void hit_record_init(hit_record *rec, vec3 p, vec3 normal, double t, bool front_face) {
    glm_vec3_copy(p, rec->p);
    glm_vec3_copy(normal, rec->normal);
    rec->t = t;
    rec->front_face = front_face;
}

void hit_record_set_face_normal(hit_record *rec, ray *r, vec3 outward_normal) {
    rec->front_face = glm_vec3_dot(r->direction, outward_normal) < 0;
    
    if (rec->front_face) {
        glm_vec3_copy(outward_normal, rec->normal);
    } else {
        glm_vec3_scale(outward_normal, -1, rec->normal);
    }
}
