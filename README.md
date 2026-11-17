# Squash '76

## Project Overview

This project forms part of an ongoing series of experiments exploring the capabilities of the free\-tier version of Microsoft Copilot AI in the design and development of fully functional recreations of classic 1970s video games. The objective is to evaluate how effectively AI\-assisted development can be used to generate modern implementations of vintage electronic sports and arcade\-style titles while preserving the gameplay principles, visual style, and technical limitations that defined the originals.

The project will deliver a squash\-inspired video game built in Rust. It features a resizable application window with a fixed 4:3 gameplay area, CRT\-inspired graphics, optional scanline effects, configurable gameplay settings, keyboard and joystick support, and synthesised retro sound effects. The game is designed to capture the look and feel of late\-1970s video entertainment systems while introducing additional gameplay options that enhance replayability.

## Attraction Screen and Configuration

When the game launches, players are presented with an attraction screen displaying a faint watermark of the game court in the background. From this screen, players can configure all key gameplay settings before starting a new game.

Available options include:

- One\-player or two\-player mode
- Game speed
- Paddle size (Small or Normal)
- Rebound angle (Shallow 20° or Steep 40°)
- Automatic or Manual Serve
- Scanlines On or Off
- CRT phosphor colour (Green, Amber, or White)
- Keyboard or Joystick controls

Once configured, selecting **Start Game** begins play using the chosen settings.

## Gameplay

The game is inspired by early electronic squash and paddle sports games. Players defend a vertical paddle line positioned near the right\-hand side of the court while returning a ball that rebounds around the playfield.

The game scales proportionally as the application window is resized. Any excess horizontal or vertical space is automatically letterboxed to preserve the fixed 4:3 aspect ratio of the gameplay area.

Paddles are constrained between the upper and lower dashed court boundaries and cannot move beyond them. Ball movement remains physically consistent regardless of window size.

Additional shot mechanics include:

- **Slice Shot** – increases the rebound angle for a faster and less predictable return.
- **Curve Shot** – applies a controlled vertical curve to the ball's trajectory.
- Curved shots may not cause the ball to pass behind a paddle.
- Ball movement includes subtle CRT\-inspired glow and trail effects.

## Visual and Audio Presentation

Visual presentation emulates a period CRT display using:

- Black background
- Green, Amber, or White phosphor rendering
- Optional screen\-wide scanlines
- Bright vector\-style graphics
- CRT\-style ball glow and motion trails
- Large upright scoreboard display

When enabled, scanlines are applied consistently across the entire scene, including walls, paddles, ball, and score displays.

Audio effects are generated using simple electronic tones inspired by 1970s arcade hardware:

| Event | Sound Effect |
| --- | --- |
| Side\-wall rebound | Crisp high\-pitched beep |
| Front\-wall impact | Double\-pitched click |
| Paddle contact | Double\-pitched click |
| Score registered | Descending tone |
| Game over | Low descending buzz |
| Replay | Low descending buzz |

## One\-Player Rules and Scoring

In one\-player mode, a single paddle is used to return the ball.

The objective is to keep the ball in play for as long as possible.

Rules:

- Each missed ball increases the miss counter by one.
- The miss counter is displayed prominently on the scoreboard.
- The game ends after **15 missed balls**.
- Following a miss, the ball continues off\-screen before the miss is officially recorded and a new serve is prepared.
- In Automatic Serve mode, play resumes immediately.
- In Manual Serve mode, the player must initiate the next serve.

The player's final score is the total number of successful returns achieved before reaching 15 misses.

## Two\-Player Rules and Scoring

In two\-player mode, two paddles share the same vertical playing column.

Rules:

- The primary paddle begins in its normal position.
- The secondary paddle appears above the primary paddle.
- Players compete to return the ball and force misses.
- If the ball passes beyond the right edge of the screen, the opposing player is awarded a point.
- The ball continues travelling off\-screen before the score is updated and the next serve is prepared.
- The first player to reach **15 points** wins the game.

The scoreboard displays both players' scores at the top of the screen throughout the match.

## Serving

Two serving modes are available:

### Automatic Serve

The ball is automatically relaunched after a point or miss.

### Manual Serve

The player must trigger the serve manually:

- **Keyboard:** Space Bar
- **Joystick:** South Face Button

## Keyboard Controls

### One\-Player Mode

| Action | Key |
| --- | --- |
| Move Paddle Up | W |
| Move Paddle Down | S |
| Alternative Movement | Up / Down Arrow |
| Serve Ball | Space |
| Slice Shot | A |
| Curve Shot | D |

### Two\-Player Mode

| Player | Up | Down |
| --- | --- | --- |
| Player 1 | W | S |
| Player 2 | Up Arrow | Down Arrow |

Additional controls:

| Action | Key |
| --- | --- |
| Return to Attraction Screen | Escape |
| Play Again | R |
| Quit Game | Q |

## Joystick Controls

### One\-Player Mode

| Action | Control |
| --- | --- |
| Move Paddle | Left Stick |
| Alternative Movement | D\-Pad |
| Serve Ball | South Face Button |
| Slice Shot | West Face Button |
| Curve Shot | North Face Button |
| Return to Attraction Screen | East Face Button |

### Two\-Player Mode

| Player | Control |
| --- | --- |
| Player 1 | Left Stick |
| Player 2 | Right Stick |
| Alternate Player 2 Control | D\-Pad |

Gamepad face\-button functions remain identical in both one\-player and two\-player modes.

## Game Over

When a game concludes, a menu is displayed offering three options:

- **Play Again**
- **Attraction Screen**
- **Quit**

These actions can also be selected using keyboard shortcuts:

| Action | Key |
| --- | --- |
| Play Again | R |
| Attraction Screen | Escape |
| Quit | Q |

This project serves as both a playable retro sports game and a practical investigation into how far a free AI\-assisted development workflow can be pushed when recreating classic electronic entertainment experiences from the 1970s.


