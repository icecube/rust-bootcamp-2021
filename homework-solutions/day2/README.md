# Day 2 Homework

Write a program that automatically generates essays for you, based on the
sample text in `exercises/day2/essay_generator`

a. Using a sample text, create a directed (multi-)graph where the words of
   a text are nodes and there is a directed edge between u and v if u is
   followed by v in your sample text. Multiple occurrences lead to multiple
   edges.

b. Do a random walk on this graph: Starting from an arbitrary node choose
   a random successor. If no successor exists, choose another random node.

It may or may not help to have a word list ordered by how common words are.
Such a list has also been provided.

If you want an extra challenge, try sampling a few physics papers and
see if you get something interesting out.
