#include "color.h"
#include "ray.h"
#include <cglm/cglm.h>
#include <math.h>
#include <stdio.h>

double sphere_collision(const vec3 center, double radius, const Ray *r) {
    vec3 center_copy = {center[0], center[1], center[2]};
    vec3 origin = {r->origin[0], r->origin[1], r->origin[2]};
    vec3 direction = {r->direction[0], r->direction[1], r->direction[2]};
    vec3 oc;
    glm_vec3_sub(origin, center_copy, oc);
    double a = glm_vec3_norm2(direction);
    double half_b = glm_vec3_dot(oc, direction);
    double c = glm_vec3_norm(oc) - radius*radius;
    double discriminant = half_b*half_b - a*c;

    if (discriminant < 0.0) {
        return -1.0;
    } else {
        return (-half_b - sqrt(discriminant)) / a;
    }
}

void ray_color(const Ray *r, vec3 dest) {
    double t = sphere_collision((vec3) {0, 0, -1}, 0.5, r);
    if (t > 0.0) {
        vec3 norm;
        ray_at(r, t, norm);
        glm_vec3_sub(norm, (vec3) {0, 0, -1}, norm);
        glm_vec3_normalize(norm);

        glm_vec3_scale((vec3) {norm[0]+1, norm[1]+1, norm[2]+1}, 0.5, dest);
        return;
    }

    vec3 unit_direction = {r->direction[0], r->direction[1], r->direction[2]};
    glm_vec3_normalize(unit_direction);
    t = 0.5*(unit_direction[1] + 1.0);

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
            Ray r;
            glm_vec3_copy(origin, r.origin);
            
            vec3 uhor, vver;
            glm_vec3_scale(horizontal, u, uhor);
            glm_vec3_scale(vertical, v, vver);
            glm_vec3_add(lower_left_corner, uhor, r.direction);
            glm_vec3_add(r.direction, vver, r.direction);
            glm_vec3_sub(r.direction, origin, r.direction);

            vec3 pixel_color;
            ray_color(&r, pixel_color);
            color_write(stdout, pixel_color);
        }
    }

    fprintf(stderr, "Done!\n");

    return 0;
}
