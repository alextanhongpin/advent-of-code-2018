# Day 12


## Part 1

A little tricky, since we need to find a way to _expand_ both the start and the end. To know when to expand the start, check the input for patterns such as :

```
....# => #
...## => #
..### => #
.#### => #
```

This means for the first pattern, you need to expand by four empty spaces, for the second pattern, three spaces and so on.


For the end, check for patterns such as:
```
#.... => #
##... => #
###.. => #
####. => #
```

When expanding, any pots to the left of the first pot has negative index. In our solution, we just expand by the max of 4 spaces front and back for each generation, and then handles the index offset by number of generation.

My initial mistake is to assume we want the count of pots of plants, but it is actually the sum of the index of pots with plants.


## Part 2


The generation is now 5 billion. Surely we do not want to run the whole thing. As usual, we need to find a O(1) solution. After repeating for n generations, a pattern emerges - the index of the pots with plants becomes constant. We know this, because ... the solution earns a star.

We first check at each generation if the delta of the index of the pots with plants is unchanged. If it is, we know we have reached the steady state. The remaining offsets can just be found by adding the delta to (generation - current generation) * delta.
