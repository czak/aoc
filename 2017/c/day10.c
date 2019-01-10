#include <stdio.h>

#define N 256

void print_list(int* list, int current) {
  for (int i=0; i<N; i++) {
    if (i == current) {
      printf("[%d] ", list[i]);
    } else {
      printf("%d ", list[i]);
    }
  }
  printf("\n---\n");
}

void reverse(int* list, int current, int length) {
  int l = current;
  int r = (current + length - 1) % N;
  for (int i=0; i<length/2; i++) {
    int tmp = list[l];
    list[l] = list[r];
    list[r] = tmp;

    l++; if (l >= N) l = 0;
    r--; if (r < 0) r = N - 1;
  }
}

int read_input(int* buf) {
  int num;
  int i = 0;
  while (fscanf(stdin, "%d,", &num) != EOF) {
    buf[i] = num;
    i++;
  }
  return i;
}

int main() {
  int list[N];
  for (int i=0; i<N; i++) {
    list[i] = i;
  }

  int lengths[100];
  int nlengths = 0;

  while (1) {
    int c = fgetc(stdin);
    if (c == EOF || c == 10) break;
    lengths[nlengths++] = c;
  }

  lengths[nlengths++] = 17;
  lengths[nlengths++] = 31;
  lengths[nlengths++] = 73;
  lengths[nlengths++] = 47;
  lengths[nlengths++] = 23;

  int current = 0;
  int skip = 0;

  for (int i=0; i<64; i++) {
    int iter = 0;

    while (iter < nlengths) {
      int length = lengths[iter];
      reverse(list, current, length);
      current = (current + length + skip) % N;
      skip++;

      /* print_list(list, current); */
      iter++;
    }
  }

  printf("Part 2: ");
  for (int i=0; i<16; i++) {
    int n = 0;
    for (int j=0; j<16; j++) {
      n ^= list[16*i+j];
    }
    printf("%02x", n);
  }
  printf("\n");
}
