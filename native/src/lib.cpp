#include <string>

extern "C"
const char* hello_cpp(char* world) {
    const char* hello = "Hello";

    static char str[14];

    snprintf(str, 14, "%s, %s!", hello, world);

    return str;
}
