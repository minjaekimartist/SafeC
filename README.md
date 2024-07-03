# SafeC
## “Safe Embeddable Assembly”

### Warning! Currently an idea project, not implied in any compiler, interpreter, or any such yet!

### Main Features
* C Superset
* Memory-safety
* Type-safety
* Modern features(generic-like, namespace, closure, tuple, method, operator override)
* LLVM Backend

### Concept
* Safe C superset to replace low-level area of C. (Operating System, Web Browser, and Library programming)
* No semantic sugar on the language. Make your own!

### How safe pointer works?
* Safe pointer is a concrete of 3 sizeof(void) memory space on stack : address, length, and lifetime.
* Length prevents segmentation error. If index is greater than length, it breaks and gives error
* Lifetime prevents memory leak. A single set of brackets counts as a block, which has a lifetime, and safe pointer defined inside a block will be freed when block ends, meaning lifetime of the pointer is over. There’s four ways to avoid it.
    1. Return the pointer, which moves the ownership of pointer to the outer block.
    2. Move the pointer to variable of the outer block.
    3. Copy the pointer to variable of the outer block. (Which is not memcpy(), it just copies data on stack. Meaning, it copies address and length, and reassigns lifetime.)
    4. Use static syntax when declaring, which will free the pointer when program ends or free() is called.
* example
```
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
```

### What is generic-like?
* Generic in Java and C# use type-eraser, which is similar to macro in C/C++ but in safer way. But it has a cost.
* Generic in C++ and Rust generate code for every type used for zero-cost abstraction, but they sacrifice compilation time and binary size.
* Generic-like in SafeC only generates code for type that is implied in source code when building the library, but third-party user can imply their own types in their libraries and programs by given header.
* Think you are only giving the API but not the implementation.
* For C/C++ users, think you are giving a header but not a source or library.
* Library developers can give users how to imply their generic function by comments or docs.
* example
```
typedef struct
{
    generic T;
    T a;
    T b;
} Foo;

generic T foo_implement(Foo& bar)
{
    return bar.a + bar.b;
    // If generic of bar doesn't match generic T
    // or generic T doesn't have operator override add, it fails to compile.
}

generic T bar(T a, T b)
{
    return a + b;    // If generic T doesn't have operator override add, it fails to compile.
}
```

### Other Modern Features
* namespace is a named scope to separate on chord block to the other and could be access through name.
```
int bar();

namespace Foo
{
    int bar();
}

int main()
{
    bar(); // use bar() outside the namespace.
    Foo::bar(); // use bar() in the namespace.
}
```
* closure is an unnamed function inside the function for certain purpose.
```
int Foo(int a, int b)
{
    return a + int (int x, int y)
    {
        return x * y;
    };
}
```
* Tuple is unnamed struct to group a value without names.
```
{ int, int } foo = { 1, 2 };
printf("%d, %d", foo.0, foo.1);
```
* Method is a default function for struct object.
```
typedef struct foo
{
    double a;
    double b;
    double (bar*)(foo& self);
} Foo;

double Foo.bar(foo& self) default
{
    return a + b;
}

```
* Operator override is a method to use standard operators to certain type, including arthmetic operator, logic operator, type casting, indexing, etc.
```
typedef struct foo
{
    double a;
    double b;
    foo (add* override)(foo& x, foo& y);
} Foo;

double Foo.add(Foo& x, Foo& y) default
{
    return Foo { .a = x->a + y->a, .b = x->b + y->b };
}
```

### Other Safety Features
* Unless in unsafe scope, struct must have all the fields before the object is used.
* Unless in unsafe scope, switch must match out every possible value.
* Unless in unsafe scope, raw pointer and union are avoided. Instead, use safe pointer and unionum.
* Unless in unsafe scope, type casting must be avoided unless the type includes override method casting to desired type. (Primitive types can be casted.)

### Unsafe Scope
Unsafe scope(or unsafe block) exists to use C codes directly, or to optimize the performance without overhead. In unsafe scope, every features avoided in the language is allowed if it follows C standard.

### Why not Rust?
* To be honest, I use Rust on most of my personal projects which I don't inter-communicate with C/C++. But Rust has a dilemma when using with C/C++. If we’re interfacing with unsafe raw pointers, why do we have to sacrifice time for thinking in different ways than programming in a single language? That’s the first reason came out for designing SafeC project.
* When we are building the lowest level of the software like firmwares and operating systems. Without C ABI and OS native libraries, we cannot use more than half of Rust std libraries, which you have to imply your own yourself or translate existing sources written in C, which is inefficient.
* Third, enum is heavy in Rust. Enum in C is just a number, while enum in Rust is similar to union in C with enum in C, which makes enum bigger and slower.
* We got idea of “Safety” from Rust, but lots of safety in Rust depends on idea your are using an existing OS with multithreaded environment that is defined in Rust ABI. For embedded or operating system programmer, it isn’t necessary, but can struggle your way. So we moved out those limitation and gave programmer more freedom.
* “Semantic sugar” in Rust is different for that in many modern languages such as Python and JaveScript. It is handled on compilation time, so the cost of it is eliminated, which is great. The side effects are compilation time and unstable ABI. Rust ABI is very unstable, so for library developer, you have to either release in every single version of Rust, or open source code, which is not always desirable for perpetual library designer.
* But for software developers on established platforms, (i.e. Windows, macOS, Linux) we highly recommend Rust! Rust is even safer than SafeC, especially for multithreaded environments, and has “semantic sugar” that will make your development much easier. You don’t have to think about C ABI, OS API, intrinsics, or etc in the most cases.

### Types
* auto
* numbers(signed or unsigned) : char, short, int, long, float, double
* bool
* void
* enum
* union
* struct
* function(no syntax)
* raw pointer(*)
* safe pointer(&)
* unionum (enum with union like enum in Rust)

### Preprocessor syntax
* if
* elif
* else
* endif
* ifdef
* ifndef
* elifdef
* elifndef
* define
* undef
* include
* error
* warning
* pragma
* defined
### Other syntax
* //
* /* */
* static
* const
* if
* else
* switch
* case
* for
* do
* while
* goto
* continue
* break
* typedef
* sizeof
* move
* malloc
* calloc
* salloc(safe alloc)
* realloc
* free
* return
* inline
* namespace
* extern
* generic
* override
* default
* unsafe
### Operator
* &
* \*
* !=
* !
* .
* ->
* ::
* ,
* ;
* \+
* \-
* \*
* \/
* \%
* &&
* ||
* &
* |
* ^
* <<
* \>\>
* ==
* \>=
* \>
* <=
* <
* =
* \"
* \'
* \`
* []
* {}
* ()

## Examples
### Hello World
```
#include<stdio.h>

int main()
{
    printf("Hello World!\n");
    return 0;
}
```
