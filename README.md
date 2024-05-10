# `cursive-hjkl`

Transparently wraps any [cursive](https://github.com/gyscos/cursive) `View` with hjkl controls.

Specifically, the wrapper listens for HJKL keys, and if the inner view doesn't respond to them, it re-sends an event with arrow keys instead.
Because it spoofs arrow keys, it should Just Work on a lot of pre-existing views.

I wrote this for [ttyloop](https://github.com/gamma-delta/ttyloop).
