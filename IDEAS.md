# Future Cleanup Thoughts

- Split `Game` messaging into a structured log (e.g. `enum Message`) so rendering/test code can format once in one place.
- Extract palette styles into a `Theme`/`Skin` layer (keep palettes swappable without touching logic).
- Add simple unit tests for movement and collision (wall bumps, stair messages, monster bites).
- Replace step-toward-player chasing with a pathfinding helper (once there are walls/rooms to navigate).
- Consider an action queue (player + monsters) to support simultaneous turns later.
