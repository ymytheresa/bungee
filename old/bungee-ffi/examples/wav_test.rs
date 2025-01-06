use bungee_ffi::{Request, Stretcher, OutputChunk};
use hound::{WavReader, WavWriter};
use std::ffi::CString;

const CHUNK_SIZE: usize = 512;
const MAX_CHANNELS: usize = 2;
const MAX_BUFFER_SIZE: usize = CHUNK_SIZE * MAX_CHANNELS;

#[derive(clap::Parser)]
#[command(name = "wav_test")]
#[command(about = "Process audio with pitch shifting")]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long, default_value_t = 12.0)]
    semitones: f64,
}

fn process_chunk(
    input_buffer: &[f32],
    output_chunk: &mut OutputChunk,
    stretcher: &Stretcher,
    request: &mut Request,
    writer: &mut WavWriter<std::io::BufWriter<std::fs::File>>,
    num_channels: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the required input range
    let input_chunk = stretcher.specify_grain(request)?;
    
    // Process the input
    stretcher.analyse_grain(input_buffer, num_channels as isize)?;
    
    // Get the next position
    stretcher.next(request)?;
    
    // Generate output
    stretcher.synthesise_grain(output_chunk)?;

    // Write output samples
    if output_chunk.frameCount > 0 {
        for frame in 0..output_chunk.frameCount as usize {
            for channel in 0..num_channels {
                let sample_idx = frame + (channel * output_chunk.channelStride as usize);
                writer.write_sample((output_chunk.data[sample_idx] * 32768.0) as i16)?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize debug logging
    let log_path = CString::new("bungee_wav_test.log").unwrap();
    unsafe {
        bungee_ffi::Bungee_Stretcher_initDebugLog(log_path.as_ptr());
    }
    
    let args = <Args as clap::Parser>::parse();
    
    let mut reader = WavReader::open(&args.input)?;
    let spec = reader.spec();
    let num_channels = spec.channels as usize;
    
    if num_channels > MAX_CHANNELS {
        return Err(format!("Too many channels: {}, maximum supported is {}", 
                         num_channels, MAX_CHANNELS).into());
    }
    
    let stretcher = Stretcher::new(
        spec.sample_rate as i32,
        num_channels as i32,
    )?;
    
    let mut writer = WavWriter::create(&args.output, spec)?;
    
    let mut request = Request::with_semitones(0.0, 1.0, args.semitones, true);
    
    let mut output_chunk = OutputChunk {
        data: vec![0.0; MAX_BUFFER_SIZE],
        frameCount: 0,
        channelStride: CHUNK_SIZE as isize,  // Store channels with frame stride
    };
    
    let mut input_buffer = vec![0.0; MAX_BUFFER_SIZE];
    let mut samples = reader.samples::<i16>();

    stretcher.preroll(&mut request)?;

    'outer: while !stretcher.is_flushed()? {
        input_buffer.fill(0.0);
        let mut samples_read = 0;

        // Read samples until we have a full chunk or reach the end of the file
        while samples_read < CHUNK_SIZE * num_channels {
            match samples.next() {
                Some(Ok(sample)) => {
                    input_buffer[samples_read] = sample as f32 / 32768.0;
                    samples_read += 1;
                }
                Some(Err(e)) => return Err(e.into()),
                None => {
                    if samples_read == 0 {
                        break 'outer;
                    } else {
                        break;
                    }
                }
            }
        }

        // Deinterleave channels
        let mut deinterleaved = vec![0.0; samples_read];
        for i in 0..samples_read / num_channels {
            for ch in 0..num_channels {
                deinterleaved[i + ch * CHUNK_SIZE] = input_buffer[i * num_channels + ch];
            }
        }

        process_chunk(
            &deinterleaved[..samples_read],
            &mut output_chunk,
            &stretcher,
            &mut request,
            &mut writer,
            num_channels,
        )?;
    }

    // Close debug logging
    unsafe {
        bungee_ffi::Bungee_Stretcher_closeDebugLog();
    }

    Ok(())
} 