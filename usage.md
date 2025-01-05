# Bungee Usage Guide

## Pitch Shifting Parameters

The `pitch` parameter in Bungee is a frequency multiplier where:
- `1.0` means no pitch change
- `2.0` means one octave up
- `0.5` means one octave down

### Converting from Semitones

To convert from musical semitones to Bungee's pitch multiplier, use this formula:
```
multiplier = 2^(semitones/12)
```

Common musical intervals:
| Semitones | Musical Interval | Pitch Multiplier |
|-----------|-----------------|------------------|
| +12       | Octave Up       | 2.000           |
| +7        | Perfect Fifth   | 1.498           |
| +5        | Perfect Fourth  | 1.335           |
| +4        | Major Third     | 1.260           |
| +3        | Minor Third     | 1.189           |
| 0         | Unison         | 1.000           |
| -3        | Minor Third Down| 0.841           |
| -4        | Major Third Down| 0.794           |
| -5        | Fourth Down     | 0.749           |
| -7        | Fifth Down      | 0.667           |
| -12       | Octave Down     | 0.500           |

### Example Usage

```rust
// Pitch up one octave
let request = Request {
    position: 0.0,
    speed: 1.0,
    pitch: 2.0,  // one octave up
    reset: true,
};

// Pitch down a perfect fifth
let request = Request {
    position: 0.0,
    speed: 1.0,
    pitch: 0.667,  // perfect fifth down
    reset: true,
};

// Helper function to convert semitones to pitch multiplier
fn semitones_to_pitch(semitones: f64) -> f64 {
    2.0f64.powf(semitones / 12.0)
}

// Using semitones
let request = Request {
    position: 0.0,
    speed: 1.0,
    pitch: semitones_to_pitch(7.0),  // perfect fifth up
    reset: true,
};
``` 