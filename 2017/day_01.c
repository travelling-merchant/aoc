#include <stdio.h>
#include <string.h>

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
  printf("\n Solution part one = %u", resultOne);
  fclose(fptr);
  return 0;
}

unsigned int calcSum(char *file, unsigned int size) {
  unsigned int sum = 0;
  unsigned int lalala = 0;
  unsigned int blabla = 0;
  for (int number = 0; number < size - 1; number++) {

    if (number == 0) {
      lalala = file[number];
    }
    blabla = file[number];
    if (file[number] == file[(number + 1 % size)]) {
      sum += (file[number] - '0');
      // substracting the char 0 is the same as sum -= 48;
    }
  }
  if (blabla == lalala) {
    sum += lalala - '0';
  }
  return sum;
}
