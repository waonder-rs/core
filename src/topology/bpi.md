# Recursive Sierpiński triangle indexing.

## Recursive Binary Partition Index (RBPI) Structure

A struture (N, <, %) where < is a comparison operator, and % a combination operator.

0 < 4 < 2 < 6 < 1 < 5 < 3 < 7

```
  |--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
  0  8  4 12  2 10  6 14  1  9  5 13  3 11  7 15

  |-----------------------|-----------------------|
  0                       1

  |-----------|-----------|-----------|-----------|
              2                       3

  |-----|-----|-----|-----|-----|-----|-----|-----|
        4           6           5           7

  |--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
     8    12    10    14     9    13    11    15
```

## Normalized RBPI Structure

Give an index to the last position on our scale.
Every index is incremented.
Last index is 0.
In this struture, 1 is the smallest element, 0 the greatest.

```
  |--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
  1  9  5 13  3 11  7 15  2 10  6 14  4 12  8 16  0
```

## RBPI2 Struture

For 2d coordinates.
