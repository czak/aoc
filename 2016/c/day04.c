#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct match {
  char ch;
  int n;
};

struct room {
  char name[128];
  int sector_id;
  char checksum[6];
};

int match_compare(const void *v1, const void *v2) {
  const struct match *m1 = v1;
  const struct match *m2 = v2;

  if (m1->n != m2->n)
    return m2->n - m1->n;
  else
    return m1->ch - m2->ch;
}

char *room_checksum(const char *name, char checksum[static 6]) {
  struct match counters[26] = {};

  for (const char *c = name; *c; c++) {
    unsigned int i = *c - 'a';
    if (i >= 26)
      continue;
    counters[i].ch = *c;
    counters[i].n++;
  }

  qsort(counters, 26, sizeof(struct match), match_compare);

  for (int i = 0; i < 5; i++) {
    checksum[i] = counters[i].ch;
  }

  checksum[5] = '\0';

  return checksum;
}

void room_parse(const char *s, struct room *room) {
  char sector[] = "";

  char *targets[] = {room->name, sector, room->checksum};
  char **p = &targets[0];

  char ch;
  while ((ch = *s++) != '\0') {
    if (ch == '-' || ch == '[' || ch == ']')
      continue;

    printf("|%c| ", ch);
  }
}

int main() {
  char cs[] = "abcde";
  assert(strcmp(room_checksum("not-a-real-room", cs), "oarel") == 0);

  struct room room = {};
  room_parse("not-a-real-room-404[oarel]", &room);
  /* assert(strcmp(room.name, "notarealroom") == 0); */
  /* assert(room.sector_id == 404); */
  /* assert(strcmp(room.checksum, "oarel") == 0); */

  FILE *f = fopen("../in/day04.in", "r");
  char buf[100];
  while (fgets(buf, 100, f) != NULL) {
    buf[strlen(buf) - 1] = '\0';
  }
}
