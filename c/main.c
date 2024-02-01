#include <stdio.h>

char* print_test()
{
    char *s = "Hello World\n";
    printf("%s", s);
}

int main()
{
    print_test();
    printf("%s", s);
}