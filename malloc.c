#include <stdlib.h>
#include <stdio.h>
#include <string.h>

int main() {
    char* name = malloc(100);
    strcpy(name, "John Wick");
    printf("Person is %s\n", name);
    free(name);
    printf("Person is %s\n", name);
    free(name);


}