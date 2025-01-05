# 🎵 Bungee: Audio Time Stretch and Pitch Shift C++ Library

Bungee is a real-time library for stretching audio. It can:
* Adjust the playback speed of audio without affecting pitch
* Adjust pitch, or transpose, audio without affecting playback speed
* Or any combination of pitch and playhead position manipulation.

Bungee is unique in its controllability, allowing continually changing pitch and position with seamless support of zero and negative playback speeds. So it can be used for a "smooth scrub" or for rendering life-like audio for slow-motion videos.

## Features

* Simple, fast, with good quality audio output (hear some [comparisons](https://bungee.parabolaresearch.com/compare.html) with other approaches)
* Resonably low latency (of the order of 20ms for speed and pitch controls and 40ms from audio input to output)
* Frequency-domain phase-vocoder-based algorithm
* Modern C++ for clean and resilient code
* Static library with a command-line utility that operates on WAV files
* Permissively licensed under MPL-2

## Getting started

Bungee's dependencies are managed as git submodules; so clone like this:
```
git clone --recurse-submodules https://github.com/kupix/bungee
```

Use CMake to configure and build the bungee library and command-line executable:
```
cd bungee
mkdir build && cd build
cmake ..
cmake --build .
```

After a successful build, run the bungee executable
```
./bungee --help
```

## Using the Sample Code

### Basic Example
The repository includes several sample programs demonstrating Bungee's functionality:

1. Basic time stretching:
```cpp
#include "Bungee.h"

// Initialize stretcher for 44.1kHz stereo audio
Bungee::Stretcher<Bungee::Basic> stretcher({44100, 44100}, 2);

// Configure processing parameters
Bungee::Request request{};
request.position = 0.0;  // Start from beginning
request.speed = 0.75;    // Play at 75% speed
request.pitch = 1.0;     // Keep original pitch

// Initialize stretcher state
stretcher.preroll(request);

// Process audio in grains
while (!stretcher.is_flushed()) {
    // Get required input segment
    auto inputChunk = stretcher.specifyGrain(request);
    
    // Provide input audio data
    stretcher.analyseGrain(inputData, channelStride);
    
    // Get processed output
    Bungee::OutputChunk outputChunk;
    stretcher.synthesiseGrain(outputChunk);
    
    // Use output audio data
    // outputChunk.data contains the processed frames
    
    // Advance to next grain
    stretcher.next(request);
}
```

### Pitch Shifting Example
To change pitch without affecting speed:
```cpp
// Shift up by one octave
request.pitch = 2.0;     // 2.0 = one octave up
request.speed = 1.0;     // maintain original speed
```

### Time Stretching Example
To change speed without affecting pitch:
```cpp
// Slow down to half speed
request.speed = 0.5;     // 0.5 = half speed
request.pitch = 1.0;     // maintain original pitch
```

### Combined Effects
You can combine pitch and speed changes:
```cpp
// Double speed and shift up one octave
request.speed = 2.0;     // 2.0 = double speed
request.pitch = 2.0;     // 2.0 = one octave up
```

For more detailed examples, see:
- `cmd/main.cpp` - Command line interface implementation
- `examples/basic.cpp` - Basic usage example
- `examples/audio_test.cpp` - Audio processing test

## Using pre-built Bungee

By means of GitHub Actions and CMake Presets, every new tag on this repository is automatically built into a release. The release contains Bungee built as a shared library together with headers, sample code and a sample command-line executable that uses the shared library. The release supports common platforms including Linux, Windows, MacOS, Android and iOS.

## Using Bungee from your own code

Bungee operates on discrete, overlapping "grains" of audio, typically processing around 100 grains per second. Parameters such as position and pitch are provided on a per-grain basis so that they can be changed continuously as audio rendering progresses. This means that only minimal parameters are required for  instantiation.

For a working example of this API, see  [cmd/main.cpp](./cmd/main.cpp).

### Instantiation

To instantiate, include the [bungee/Bungee.h](./bungee/Bungee.h) header file, create a `Stretcher<Basic>` object and initialise a `Request` object:

``` C++
#include "Bungee.h"
#include <cmath>

const int sampleRate = 44100;

Bungee::Stretcher<Bungee::Basic> stretcher({sampleRate, sampleRate}, 2);

Bungee::Request request{};

// Set pitch, this example shows an upward transposition of one semitone.
request.pitch = std::pow(2., 1. / 12.);

// Set initial speed, this example shows how to achieve constant 75% output speed.
request.speed = 0.75;

// Set initial starting position at 0.5 seconds offset from the start of the input buffer.
request.position = 0.5;

// This call adjusts request.position so that stretcher's pipeline will be fully initialised by the
// time it reaches the starting position of 0.5 seconds offset.
stretcher.preroll(request);
```

### Granular loop

`Stretcher`'s processing functions are typically called from within a loop, each iteration of which corresponds to a grain of audio. For each grain, the functions `Stretcher<Basic>::specifiyGrain`, `Stretcher<Basic>::analyseGain` and `Stretcher<Basic>::synthesiseGrain` should be called in sequence.
```C++
while (true)
{
    // ...
    // Change request's members, for example, position, speed or pitch, as required here.
    // ...
 
    auto inputChunk = stretcher.specifyGrain(request);

    // ...
    // Examine inputChunk and retrieve the segment of input audio that the stretcher requires here.
    // Set data and channelStride to point to the input data.
    // ...

    stretcher.analyseGrain(data, channelStride);

    Bungee::OutputChunk outputChunk;
    stretcher.synthesiseGrain(outputChunk);

    // ...
    // Output the audio buffer indicated by outputChunk here.
    // ...

    // Prepare request for the next grain (modifies request.position according to request.speed)
    stretcher.next(request);
}
```

### Things to note

* Bungee is well tuned for stereo and mono operation at sample rates of 44.1kHz and 48kHz. In principle, though, any practical sample rate and number of audio channels are supported.

* `Request::position` is a timestamp, it defines the grain centre point in terms of an input audio frame offset. It is the primary control for speed adjustments and is also the driver for seek and scrub operations. The caller is responsible for deciding  `Request::position` for each grain. 

* The caller owns the input audio buffer and must provide the audio segment indicated by `InputChunk`. Successive grains' input audio chunks may overlap. The `Stretcher<Basic>` instance reads in the input chunk data when `Stretcher<Basic>::analyseGrain` is called.

* The `Stretcher<Basic>` instance owns the output audio buffer. It is valid from when `Stretcher<Basic>::synthesiseGrain` returns up until `Stretcher<Basic>::synthesiseGrain` is called for the subsequent grain. Output audio chunks do not overlap: they should be concatenated to produce an output audio stream.

* Output audio is timestamped. The original `Request` objects corresponding to the start and end of the chunk are provided by `OutputChunk`.

* Bungee works with 32-bit floating point audio samples and expects sample values in the range -1 to +1 on both input and output. The algorithm performs no clipping.

* When configured for 1x speed and no pitch adjustment, the difference between input and output signals should be very small: rounding errors only.

* Any special or non-numeric float values such as NaN and inf within the input audio may disrupt or cause loss of output audio.

## Dependencies

The Bungee library gratefully depends on:
* Eigen C++ library for buffer management and mathematical operations on vectors and arrays 
* KissFFT library for Fast Fourier Transforms

The sample `bungee` command-line utility also uses:
* cxxopts library for parsing command-line options

See this repo's [.gitmodules](.gitmodules) for versioned links to these projects.

## Bungee Pro

Bungee Pro is a closed-source commercial audio library built on Bungee's API and core philosophy. It uses novel algorithms for sharp and clear professional-grade audio and runs at least as fast as Bungee, thanks to platform-specific performance optimisations.

Try Bungee Pro [now in your browser](https://bungee.parabolaresearch.com/bungee-web-demo.html), see a [comparison](https://bungee.parabolaresearch.com/compare.html) with other techniques and consider licensing if your app would benefit from:
* Powerful, AI-enabled stretch algorithms adaptive to all genres of speech, music and sound with subjective transparency up to infinite time stretch
* Novel frequency- and time-domain processing for crisp transients and presevation of tonal envelope, vocal and instrumental timbre
* Performance optimisations for:
    * Web AudioWorklet with SIMD128 Web Assembler
    * Arm 64-bit for Android, iOS and MacOS
    * x86-64 for Linux, Windows and MacOS
* A ready-to-use Web Audio implementation 
