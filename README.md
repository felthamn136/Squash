# Squash '76

A resizable Rust window with a fixed 4:3 game field, black background, and dim amber-orange CRT phosphor. The game scales proportionally as the window changes size; wider or taller space is letterboxed. Scanlines cover the screen, walls, and paddles, with a subtle glow/trail on the ball. Both paddles share the same vertical column; the paler second paddle starts above the original. Move the second paddle with the **Up/Down arrow keys** and the original with **W/S**. Both stay between the top and bottom dashed lines. The large, upright scoreboard at the top shows each player's points. After a paddle miss, the ball continues offscreen before awarding the point and resetting; press **Space** to serve again. The ball rebounds at an angle based on where it hits a paddle, like Pong, and rally speed increases slightly on each return up to a cap. Hold **A** on contact for a fast slice or **D** for an optional curving shot. Curves bend vertically and cannot carry the ball behind a paddle.

Run it with:

```sh
cargo run
```
