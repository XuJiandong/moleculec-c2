
#include "fuzzer_func.h"


int LLVMFuzzerTestOneInput(uint8_t *data, size_t size) {
  int code = setjmp(g_return_point);
  if (code == 0) {
    mol2_cursor_t block2 = mol2_make_cursor_from_memory(data, size);
    read_with_new_api(block2);
  }
  return 0;
}

#ifdef NO_FUZZER
int main(int argc, const char* argv[]) {
  fprintf(stderr, "StandaloneFuzzTargetMain: running %d inputs\n", argc - 1);
  for (int i = 1; i < argc; i++) {
    fprintf(stderr, "Running: %s\n", argv[i]);
    FILE *f = fopen(argv[i], "r");
    assert(f);
    fseek(f, 0, SEEK_END);
    size_t len = ftell(f);
    fseek(f, 0, SEEK_SET);
    unsigned char *buf = (unsigned char *)malloc(len);
    size_t n_read = fread(buf, 1, len, f);
    fclose(f);
    assert(n_read == len);
    LLVMFuzzerTestOneInput(buf, len);
    free(buf);
    fprintf(stderr, "Done:    %s: (%zd bytes)\n", argv[i], n_read);
  }
}
#endif


