# .h file generator
## Goal
This is pretty much my first project to practice writing in rust.
So if I could also make something useful. why not? :)

## Usage
```bash
hgen --path <C_FILE_PATH> [-i / --includes] [-s / --structs] [-d / --defines]
```

## Example

### Input
test.c:
```c
#include <stdio.h>

#define NUM 3

struct s {
    int d;
    float f;
}

void a()
{
    int a = 1;
}

int b(int c)
{
    return NUM;
}
```


### Output
test.h (generated file):
```c
#include <stdio.h>

#define NUM 3

struct s {
    int d;
    float f;
};


void a();
int b(int c);

```

test.c:
```c
#include "test.h"


void a()
{
    int a = 1;
}

int b(int c)
{
    return NUM;
}
```

## Todo:
1. option to specify output path
2. improve strcut regex