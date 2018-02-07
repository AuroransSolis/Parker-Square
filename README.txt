Here's the idea: this program tries to find a magic square of perfect squares where all nine entries are distinct.

What for purposes of optimization, it uses a couple qualities of magic square entries (for instance, all squared entries must be 1 modulo 24), and the following method of producing non-squared values is used:

  x + y       x - y - z       x + z
x - y + z         x         x + y - z
  x - z       x + y + z       x - y

Then each of the values is squared and tested to see if it's a square.

If you've got a couple million years to spare, maybe this thing will finish running, but if something pops up before then, feel free to stop it running.