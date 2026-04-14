#include <stdio.h>

unsigned int calcSum(char *file, unsigned int size);

int main() {
  FILE *fptr;
  fptr = fopen("day_01.txt", "r");
  if (fptr == NULL) {
    printf("You fucking Moron\n");
    return -1;
  }
  fseek(fptr, 0L, SEEK_END);
  unsigned int fSize = ftell(fptr);
  char fileContent[fSize];
  rewind(fptr);

  fgets(fileContent, fSize, fptr);
  unsigned int resultOne = calcSum(fileContent, fSize);
  printf("\n Solution part one = %u", resultOne >> 16);
  printf("\n Solution part two = %u", resultOne & 0xFFFF);
  fclose(fptr);
  return 0;
}
unsigned int calcSum(char *file, unsigned int size) {
  unsigned short sum = 0;
  unsigned short sun = 0;
  unsigned short lalala = 0;
  unsigned short blabla = 0;
  unsigned short half = size / 2;

  for (int number = 0; number < size - 1; number++) {
    unsigned char current = file[number];

    if (number < half) {
      if (current == file[number + half]) {
        sun += current - 48;
        sun += file[number + half] - 48;
      }
    }

    if (number == 0) {
      lalala = current;
    }
    blabla = current;
    if (current == file[(number + 1 % size)]) {
      sum += (current - '0');
      // substracting the char 0 is the same as sum -= 48;
    }
  }
  if (blabla == lalala) {
    sum += lalala - '0';
  }

  return ((unsigned int)sum << 16) | sun;
}
