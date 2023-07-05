#include <stdio.h>

int main(int argc, char *argv[]) {
    const int image_width = 256;
    const int image_height = 256;

    printf("P3\n%d %d\n255\n", image_width, image_height);

    for (int y = image_height; y >= 0; --y) {
        for (int x = 0; x < image_width; ++x) {
            double r = (double) x / (image_width-1);
            double g = (double) y / (image_height-1);
            double b = 0.25;

            int ir = (int) (255.999 * r);
            int ig = (int) (255.999 * g);
            int ib = (int) (255.999 * b);

            printf("%d %d %d\n", ir, ig, ib);
        }
    }

    return 0;
}
