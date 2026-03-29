#include <stdio.h>

int main() {
  FILE *fptr;
  fptr = fopen("day_01.txt", "r");
  if (fptr == NULL) {
    printf("You fucking Moron\n");
    return -1;
  }
  fseek(fptr, 0L, SEEK_END);
  long int fSize = ftell(fptr);
  rewind(fptr);
  char fileContent[fSize];
  fgets(fileContent, fSize, fptr);
  printf("%s", fileContent);
  printf("%d", fSize);
  fclose(fptr);
  return 0;
}


