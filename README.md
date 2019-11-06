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

[1]: https://en.wikipedia.org/wiki/100_prisoners_problem
[2]: http://rosettacode.org/wiki/100_prisoners
