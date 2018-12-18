#include <stdint.h>
#include <stdlib.h>
#include <string>

extern "C" {

int32_t big_function() {
    std::string str;
    // Make sure our result depends on something the compiler can't optimize so
    // that we get the linker error even when building in release mode.
    if (rand() > 5) {
      str += "hello there";
    } else {
      str += "well goodbye";
    }
    return str.size();
}

}
