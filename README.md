# Squash '76

A resizable Rust window with a fixed 4:3 game field, black background, and dim amber-orange CRT phosphor. 

The attraction screen presents the game court as a faint watermark. Use the sliders to choose player count, game speed, paddle size (**Small** or **Normal**), rebound angle (**Shallow 20°** or **Steep 40°**), automatic or manual serves, and scanlines, then select **Start Game**. One-player mode has a single paddle; each missed ball adds one to the miss counter, and the game ends after 15 misses. Press **Escape** during a game to return to the attraction screen.

The game scales proportionally as the window changes size; wider or taller space is letterboxed. 

When enabled, scanlines cover the screen, walls, and paddles, with a subtle glow/trail on the ball. In two-player mode, both paddles share the same vertical column; the paler second paddle starts above the original. Move the second paddle with the **Up/Down arrow keys** and the original with **W/S**. Both stay between the top and bottom dashed lines.

The large, upright scoreboard at the top shows each player's points in two-player mode and missed balls in one-player mode. When a two-player miss passes the right edge, the opponent scores; the first player to 15 points wins. After a miss, the ball continues offscreen before the score advances and it resets. Automatic serve mode serves again automatically; in manual mode press **Space** to serve. The ball rebounds at the selected angle based on where it hits a paddle, like Pong, and rally speed increases slightly on each return up to a cap. Hold **A** on contact for a fast slice or **D** for an optional curving shot. Curves bend vertically and cannot carry the ball behind a paddle.

Run it with:

```sh
cargo run
```
