#include <gmp.h>
#include <stdlib.h>
#include <stdio.h>

int main() {
    mpz_t i, j, k;
    mpz_init_set_str (i, "1a", 16);
    mpz_init (j);
    mpz_init (k);
    mpz_sqrtrem (j, k, i);
    if (mpz_get_si (j) != 5 || mpz_get_si (k) != 1) abort();
    printf("%s\n", "Works as expected");
    return 0;
}
