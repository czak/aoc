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
  char *p;

  // read name
  p = room->name;
  while (*s >= 'a' || *s == '-') {
    char ch = *s++;
    if (ch == '-')
      continue;
    *p++ = ch;
  }

  // read sector id
  char sector_id[10] = {};
  p = sector_id;
  while (*s >= '0' && *s <= '9') {
    char ch = *s++;
    *p++ = ch;
  }
  room->sector_id = atoi(sector_id);

  // read checksum
  p = room->checksum;
  while (*s != ']') {
    char ch = *s++;
    if (ch == '[')
      continue;
    *p++ = ch;
  }
}

int main() {
  char cs[] = "abcde";
  assert(strcmp(room_checksum("not-a-real-room", cs), "oarel") == 0);

  struct room room = {};
  room_parse("not-a-real-room-404[oarel]", &room);
  assert(strcmp(room.name, "notarealroom") == 0);
  assert(room.sector_id == 404);
  assert(strcmp(room.checksum, "oarel") == 0);

  FILE *f = fopen("../in/day04.in", "r");
  char buf[100];
  int sum = 0;

  while (fgets(buf, 100, f) != NULL) {
    memset(&room, 0, sizeof(struct room));
    room_parse(buf, &room);

    if (strcmp(room_checksum(room.name, cs), room.checksum) == 0) {
      sum += room.sector_id;
    }
  }

  // Part 1
  printf("Part 1: %d\n", sum);
  assert(sum == 185371);
}
