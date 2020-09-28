#undef NDEBUG
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#include "types-api2.h"
void* build_all(int* size);

int main(int argc, const char* argv[]) {
  int size = 0;
  uint8_t* buff = build_all(&size);

  printf("\n------- types passed---------\n\n");
  return 0;
}
