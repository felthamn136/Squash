# Squash '76

A resizable Rust window with a fixed 4:3 game field, black background, and selectable green, amber, or white CRT phosphor.

The attraction screen presents the game court as a faint watermark. Use the sliders to choose player count, game speed, paddle size (**Small** or **Normal**), rebound angle (**Shallow 20°** or **Steep 40°**), automatic or manual serves, scanlines, game color (**Green**, **Amber**, or **White**), and **Keyboard** or **Joystick** controls, then select **Start Game**. One-player mode has a single paddle; each missed ball adds one to the miss counter, and the game ends after 15 misses. Press **Escape** (or the joystick's east face button) during a game to return to the attraction screen.

The game scales proportionally as the window changes size; wider or taller space is letterboxed. 

When enabled, scanlines cover the screen, walls, and paddles, with a subtle glow/trail on the ball. Side-wall rebounds play a crisp high beep, while front-wall and racket impacts use a double-pitched click. Scoring plays a descending tone; game over and replay use a low descending buzz. In two-player mode, both paddles share the same vertical column; the paler second paddle starts above the original. Keyboard mode uses **W/S** and **Up/Down** for the paddles. Joystick mode uses the left and right sticks, respectively; the D-pad also controls the second paddle and serves as a one-player movement alternative. Both paddles stay between the top and bottom dashed lines.

The large, upright scoreboard at the top shows each player's points in two-player mode and missed balls in one-player mode. When a two-player miss passes the right edge, the opponent scores; the first player to 15 points wins. After a miss, the ball continues offscreen before the score advances and it resets. Automatic serve mode serves again automatically; in manual mode press **Space** or the joystick's south face button to serve. At game over, choose **Play Again**, **Attraction Screen**, or **Quit**. Keyboard shortcuts **R**, **Escape**, and **Q** perform those actions, respectively. Keyboard mode uses **A** for a fast slice and **D** for a curving shot; joystick mode uses the west and north face buttons, respectively. Curves bend vertically and cannot carry the ball behind a paddle.

Run it with:

```sh
cargo run
```
