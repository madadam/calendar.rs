# Calendar

This is a (poor attempt at) rust reimplementation of Eric Niebler's Calendar app
(https://github.com/ericniebler/range-v3/blob/master/example/calendar.cpp).
I tried to stay as close as possible to his way, but deviated from it when it
became too much hassle for me.

The biggest hurdles for me were the lack of return type deduction and
my poor experience with the borrow checker. This was my first rust project so
it is probably quite horrible. I'm pretty sure there are more idiomatic and
elegant ways of doing this.
