#include "world.h"
#include <string.h>

bool object_list_collision(object *object_list, int object_count, ray *r, double t_min, double t_max, hit_record *rec) {
    hit_record temp_rec;
    bool hit_anything = false;
    double closest_so_far = t_max;

    for (int i=0; i<object_count; ++i) {
        switch (object_list[i].id) {
            case SPHERE:
                if (sphere_collision((sphere*) &object_list[i].data, r, t_min, closest_so_far, &temp_rec)) {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                    memcpy(rec, &temp_rec, sizeof(hit_record));
                }
                break;
            default:
                break;
        }
    }

    return hit_anything;
}
