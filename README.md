# Ski Resort Tycoon

## TODO Lists

### Terrain

- [ ] Make terrain more smooth. Set corner heights closer to the median of the neighboring cells, rather than just the mean because that creates weird spikes.
- [ ] Larger cursors.
- [ ] Improve performance by only updating cells next to the ones changed.
- [ ] Vary material per triangle for cliffs, instead of per hexagon.
- [ ] Clean up water generation.

### Interaction

- [ ] TAB to cycle between hovered items.
- [ ] Fix cells being picked through items, even when the correct tool is selected.
- [ ] Generally improve the logic for selecting items by selecting cells.
- [ ] Solve "Mouse clicked structure before it was hovered." error.
