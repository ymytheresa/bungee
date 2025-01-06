# Bungee Usage Guide

## Audio Processing Model

Bungee processes audio in grains (chunks) which allows for real-time processing and smooth transitions. The processing pipeline consists of:

1. **Grain Specification**
   - Define grain position in input audio
   - Set processing parameters
   - Request necessary input frames

2. **Analysis Phase**
   - Process input audio data
   - Analyze frequency content
   - Prepare for synthesis

3. **Synthesis Phase**
   - Generate output audio
   - Apply time-stretching and pitch-shifting
   - Ensure smooth transitions between grains

## Processing Parameters

### Time Stretching
The `speed` parameter controls time stretching where:
- `1.0` means normal speed
- `2.0` means twice as fast
- `0.5` means half speed

### Pitch Shifting
The `pitch` parameter is a frequency multiplier where:
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

## Memory Management

### Input/Output Buffers
- Input buffers must be large enough for requested frames
- Output buffers must accommodate stretched audio
- Buffer sizes can be queried via `maxInputFrameCount`
- Non-interleaved audio format (separate channels)

### State Management
- Use `reset` flag to clear internal state
- Check `isFlushed` to verify processing completion
- Maintain proper grain sequence for smooth output

## Example Usage

### Basic Time Stretching
```rust
// Slow down to half speed
let request = Request {
    position: 0.0,
    speed: 0.5,    // half speed
    pitch: 1.0,    // maintain pitch
    reset: true,
};
```

### Pitch Shifting
```rust
// Pitch up one octave
let request = Request {
    position: 0.0,
    speed: 1.0,    // maintain speed
    pitch: 2.0,    // one octave up
    reset: true,
};

// Pitch down a perfect fifth
let request = Request {
    position: 0.0,
    speed: 1.0,
    pitch: 0.667,  // perfect fifth down
    reset: true,
};
```

### Helper Functions
```rust
// Convert semitones to pitch multiplier
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

## Real-time Processing Tips

1. **Buffer Management**
   - Pre-allocate buffers to avoid runtime allocations
   - Use `maxInputFrameCount` to determine buffer sizes
   - Keep buffer sizes consistent for stable performance

2. **Grain Sequencing**
   - Maintain proper grain positions
   - Use `next` to advance grain position
   - Handle position wraparound for looping

3. **State Handling**
   - Use `reset` when changing parameters significantly
   - Check `isFlushed` before cleanup
   - Handle error states appropriately 