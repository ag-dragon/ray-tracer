#include "color.h"
#include "ray.h"
#include "sphere.h"
#include "hit_record.h"
#include "world.h"
#include <cglm/cglm.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>

void ray_color(ray *r, object *object_list, int object_count, vec3 dest) {
    hit_record rec;
    if (object_list_collision(object_list, object_count, r, 0, 999999.0, &rec)) {
        //glm_vec3_add(rec.normal, GLM_VEC3_ONE, dest);
        //glm_vec3_scale(dest, 0.5, dest);
        //fprintf(stderr, "hi,");
        glm_vec3_copy((vec3) {1, 0, 0}, dest);
        return;
    }
    vec3 unit_direction;
    glm_vec3_copy(r->direction, unit_direction);
    glm_vec3_normalize(unit_direction);
    double t = 0.5*(unit_direction[1] + 1.0);

    vec3 color1 = GLM_VEC3_ONE_INIT;
    glm_vec3_scale(color1, (1.0-t), color1);
    vec3 color2 = {0.5, 0.7, 1.0};
    glm_vec3_scale(color2, t, color2);
    glm_vec3_add(color1, color2, dest);
}

int main(int argc, char *argv[]) {
    // Image
    const double aspect_ratio = 16.0 / 9.0;
    const int image_width     = 800;
    const int image_height    = (int) (image_width / aspect_ratio);

    // World
    int object_count = 1;
    object *object_list = malloc(sizeof(object)*object_count);
    sphere *s1 = malloc(sizeof(sphere));
    sphere_init(s1, (vec3) {0, 0, -1}, 0.5);
    object_list[0].id = SPHERE;
    object_list[0].data.s = s1;
    //sphere *s2 = malloc(sizeof(sphere));
    //sphere_init(s2, (vec3) {0, -100.5, -1}, 50);
    //object_list[0].id = SPHERE;
    //object_list[0].data.s = s2;

    // Camera
    double viewport_height = 2.0;
    double viewport_width  = aspect_ratio * viewport_height;
    double focal_length    = 1.0;

    vec3 origin     = GLM_VEC3_ZERO_INIT;
    vec3 horizontal = {viewport_width, 0, 0};
    vec3 vertical   = {0, viewport_height, 0};
    
    vec3 hdiv;
    glm_vec3_divs(horizontal, 2, hdiv);
    vec3 vdiv;
    glm_vec3_divs(vertical, 2, vdiv);
    vec3 lower_left_corner;
    glm_vec3_sub(origin, hdiv, lower_left_corner);
    glm_vec3_sub(lower_left_corner, vdiv, lower_left_corner);
    glm_vec3_sub(lower_left_corner, (vec3) {0, 0, focal_length}, lower_left_corner);
    

    printf("P3\n%d %d\n255\n", image_width, image_height);

    for (int y = image_height; y >= 0; --y) {
        //fprintf(stderr, "Scanlines remaining: %d\n", y);
        for (int x = 0; x < image_width; ++x) {
            double u = (double) x / (image_width-1);
            double v = (double) y / (image_height-1);
            ray r;
            glm_vec3_copy(origin, r.origin);
            
            vec3 uhor, vver;
            glm_vec3_scale(horizontal, u, uhor);
            glm_vec3_scale(vertical, v, vver);
            glm_vec3_add(lower_left_corner, uhor, r.direction);
            glm_vec3_add(r.direction, vver, r.direction);
            glm_vec3_sub(r.direction, origin, r.direction);

            vec3 pixel_color;
            ray_color(&r, object_list, object_count, pixel_color);
            color_write(stdout, pixel_color);
        }
    }

    fprintf(stderr, "Done!\n");

    return 0;
}
