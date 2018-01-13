#include <cstdint>
#include <cstdlib>

struct Foo;

struct String;

template<typename T>
struct Vec;

extern "C" {

void func0();

uint32_t func1(uint32_t a, uint32_t b);

Vec<int32_t> func2(size_t n);

String func3(Foo data);

const char *func4(Foo data);

} // extern "C"
