# Day 3: No Matter How You Slice It


## Part 1

Pretty straightforward. Just store the tiles positions in a hashmap, and increment the overlapping counts.


Then count the number of tiles that has count greater or equal two.


## Part 2

Find the non-overlapping fabric. Sounds hard at first, but it's actually pretty simple. We can adapt solution one to return not only the count, but also the tile id. Since we already know the area of each tiles based on id, we just need to count each if they have that area and has only been marked once.
