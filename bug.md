# Audio Processing Duration Bug

## Configuration
- Full duration mode: false
- Target duration: 30 seconds
- Speed: 1
- Pitch shift: 12 semitones (multiplier: 2)

## Problem
The output duration (7.616145 seconds) does not match the expected duration (15 seconds).
Duration error: 7.383854866027832 seconds

## Processing Details
- Input duration: 180.6 seconds
- Sample rate: 44100 Hz
- Channels: 2
- Total input frames: 7964460
- Processed frames: 1323520
- Output frames: 335872
- Expected output frames: 661500
- Chunks processed: 328

## Analysis
The output duration is significantly different than expected. This might indicate:
1. Frames are being dropped during processing
2. The stretcher is not generating the expected number of output frames
3. There might be an issue with the frame counting logic

## Steps to Reproduce
1. Run wav_test without --full-duration flag --semitones 12
2. Process 30 second audio file
3. Check output duration
