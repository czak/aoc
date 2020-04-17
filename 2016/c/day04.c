#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

struct match {
  char ch;
  int n;
};

int match_compare(const void *v1, const void *v2) {
  const struct match *m1 = v1;
  const struct match *m2 = v2;

  if (m1->n != m2->n)
    return m2->n - m1->n;
  else
    return m1->ch - m2->ch;
}

int main() {
  char name[] = "not-a-real-room";

  // count occurrences
  struct match counters[26] = {};

  for (char *c = name; *c; c++) {
    unsigned int i = *c - 'a';
    if (i >= 26)
      continue;
    counters[i].ch = *c;
    counters[i].n++;
  }

  qsort(counters, 26, sizeof(struct match), match_compare);

  for (int i = 0; i < 5; i++) {
    printf("%c = %d\n", counters[i].ch, counters[i].n);
  }
}
