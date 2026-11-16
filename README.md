# Squash '76 display

A minimal Rust window with a fixed 4:3 aspect ratio, black background, and dim amber-orange CRT phosphor. Scanlines cover the screen, walls, and paddle, with a subtle glow/trail on the ball. The right-side paddle moves with **W/S** between the top and bottom dashed lines. Press **Space** to serve; the ball rebounds off the wall and boundaries, and the paddle bats it back at an angle based on where it hits, like Pong. Ball speed increases slightly on each paddle return, up to a cap. Hold **A** on contact for a fast slice or **D** for an optional curving shot. Curves bend vertically and cannot carry the ball behind the paddle.

Run it with:

```sh
cargo run
```
