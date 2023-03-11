#include <stdio.h>

int main(int argc, char* argv[]) {
    if (argc < 2)
        return 1;

    char* fname = argv[1];
    FILE* file = fopen(fname, "r");

    while (file) {
        char character = fgetc(file);
        if (character == EOF) {
            fclose(file);
            break;
        }

        printf("%c", character);
    }

    return 0;
}
