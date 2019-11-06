# Hundred Prisoners

This is a Rust solution (and probably not-optimal) of the [100 Prisoners][1]
problem. I came across this from via [Rosetta Code][2].

There is one thing that isn't specified in the optimal solution of this problem
was to what prisoners do when either the random number generator or the box
cycle includes a box they have already seen. To play with this I implemented
both a naive and tracked version of both the random model and the optimal
model.

An earlier version of this that I wrote (you can go back one or two commits in
the solution I'm leaving up to look at it); I ommited that the prisoner started
with their own box number and I'm positively floored by how much of a
difference that has on the probability of success as opposed to starting with a
random number. This was before I performed any tracking on 'seen' boxes and I
have a sneaking suspicion that that has a lot to do with the difference in
success.

Specifically if prisoner 3 starts at box 3, it contains 1 and 1 in turn
contains 3 that prisoner has succeeded. If they start randomly this changes
significantly to where if I as prisoner 3 start at box 7 and it has 1 which
points back to 7 then I'll expend all my attempts bouncing between the two
boxes.

I went back and implemented a fifth option which does start randomly and tracks
which boxes have been seen to see if I could maybe meet in the middle but I was
still getting a 0 success. It seems like starting with your own number and
ensuring you're potentially part of your own cycle if it exists is one of the
defining traits of the solution.

## Results

I ran this with 100,000,000 iterations to see the variations in results. The
stats are real bad for any semblance of randomness and I would be surprised to
*ever* see a success with these run based on those probabilities.

This was before I added the fifth "random init" version of the optimal solution
so it's results aren't available. This run took close to half an hour on my
laptop. This could be vastly improved with threading but that was more effort
than I was willing to put into this particular problem.

```
The prisoners optimally (naive) succeeded 31187840 out of 100000000 times
The prisoners optimally (tracked) succeeded 31187840 out of 100000000 times
The prisoners randomly (naive) succeeded 0 out of 100000000 times
The prisoners randomly (tracked) succeeded 0 out of 100000000 times
```

I'm still kind of shocked that there were no cases where tracking the state of
seen boxes improved the result. I suppose in any instance where the cycle
length is less than the number of attempts you'd succeed and if it's longer
then you'd fail before it ever mattered that you'd seen the box before.

[1]: https://en.wikipedia.org/wiki/100_prisoners_problem
[2]: http://rosettacode.org/wiki/100_prisoners
