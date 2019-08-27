#include <stdio.h>

char* hello_c(char* world) {
    char* hello = "Hello";
    
    static char str[14];

    snprintf(str, 14, "%s, %s!", hello, world);
    
    return str;
}
