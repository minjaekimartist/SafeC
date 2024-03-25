#include <stdlib.h>
#include <salloc.h>

void pointer1()
{
    void& safe_pointer = (void&)salloc(sizeof(void) * 4); // safe pointer allocated
    return;
} // safe pointer freed

void& pointer2()
{
    void& safe_pointer = (void&)salloc(sizeof(void) * 4); // safe pointer allocated
    return safe_pointer; // moves out ownership of safe_pointer
}

int main()
{
    pointer1(); // safe pointer allocated and deallocated.
    void& pointer = pointer2(); // safe pointer allocated and assigned to variable "pointer"
    return 0;
} // safe pointer in variable "pointer" freed.
