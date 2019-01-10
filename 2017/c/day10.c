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

  int lengths[50];
  int nlengths = read_input(lengths);

  int current = 0;
  int skip = 0;
  int iter = 0;

  // initial
  print_list(list, current);

  while (iter < nlengths) {
    int length = lengths[iter];
    reverse(list, current, length);
    current = (current + length + skip) % N;
    skip++;

    print_list(list, current);
    iter++;
  }

  printf("Part 1: %d\n", list[0]*list[1]);
}
