#include "color.h"
#include <cglm/cglm.h>

void color_write(FILE *stream, vec3 pixel_color) {
    fprintf(stream, "%d %d %d\n", (int) (255.999 * pixel_color[0]),
                                  (int) (255.999 * pixel_color[1]),
                                  (int) (255.999 * pixel_color[2]));
}
