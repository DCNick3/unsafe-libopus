use super::MAX_PACKET;

use std::str::FromStr;
use unsafe_libopus::{
    OPUS_APPLICATION_AUDIO, OPUS_APPLICATION_RESTRICTED_LOWDELAY, OPUS_APPLICATION_VOIP,
    OPUS_BANDWIDTH_FULLBAND, OPUS_BANDWIDTH_MEDIUMBAND, OPUS_BANDWIDTH_NARROWBAND,
    OPUS_BANDWIDTH_SUPERWIDEBAND, OPUS_BANDWIDTH_WIDEBAND,
};

#[derive(Debug, Copy, Clone)]
pub enum Application {
    Voip,
    Audio,
    RestrictedLowDelay,
}

impl Application {
    pub fn into_opus(self) -> i32 {
        match self {
            Application::Voip => OPUS_APPLICATION_VOIP,
            Application::Audio => OPUS_APPLICATION_AUDIO,
            Application::RestrictedLowDelay => OPUS_APPLICATION_RESTRICTED_LOWDELAY,
        }
    }
}

impl FromStr for Application {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "voip" => Ok(Application::Voip),
            "audio" => Ok(Application::Audio),
            "restricted-lowdelay" => Ok(Application::RestrictedLowDelay),
            _ => Err("Unsupported application"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SampleRate {
    R48000,
    R24000,
    R16000,
    R12000,
    R8000,
}

impl FromStr for SampleRate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .parse()
            .map_err(|_| "Cannot parse the sample rate as number")?;

        match s {
            48000 => Ok(SampleRate::R48000),
            24000 => Ok(SampleRate::R24000),
            16000 => Ok(SampleRate::R16000),
            12000 => Ok(SampleRate::R12000),
            8000 => Ok(SampleRate::R8000),
            _ => Err("Unsupported sample rate"),
        }
    }
}

impl From<SampleRate> for usize {
    fn from(value: SampleRate) -> Self {
        match value {
            SampleRate::R48000 => 48000,
            SampleRate::R24000 => 24000,
            SampleRate::R16000 => 16000,
            SampleRate::R12000 => 12000,
            SampleRate::R8000 => 8000,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Channels {
    Mono,
    Stereo,
}

impl FromStr for Channels {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Channels::Mono),
            "2" => Ok(Channels::Stereo),
            _ => Err("Unsupported number of channels"),
        }
    }
}

impl From<Channels> for usize {
    fn from(value: Channels) -> Self {
        match value {
            Channels::Mono => 1,
            Channels::Stereo => 2,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Bandwidth {
    Narrowband,
    Mediumband,
    Wideband,
    SuperWideband,
    Fullband,
}

impl Bandwidth {
    pub fn into_opus(self) -> i32 {
        match self {
            Bandwidth::Narrowband => OPUS_BANDWIDTH_NARROWBAND,
            Bandwidth::Mediumband => OPUS_BANDWIDTH_MEDIUMBAND,
            Bandwidth::Wideband => OPUS_BANDWIDTH_WIDEBAND,
            Bandwidth::SuperWideband => OPUS_BANDWIDTH_SUPERWIDEBAND,
            Bandwidth::Fullband => OPUS_BANDWIDTH_FULLBAND,
        }
    }
}

impl FromStr for Bandwidth {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NB" => Ok(Bandwidth::Narrowband),
            "MB" => Ok(Bandwidth::Mediumband),
            "WB" => Ok(Bandwidth::Wideband),
            "SWB" => Ok(Bandwidth::SuperWideband),
            "FB" => Ok(Bandwidth::Fullband),
            _ => Err("Unsupported bandwidth"),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum FrameSize {
    Ms2_5,
    Ms5,
    Ms10,
    #[default]
    Ms20,
    Ms40,
    Ms60,
    Ms80,
    Ms100,
    Ms120,
}

impl FrameSize {
    pub fn samples_for_rate(self, rate: SampleRate) -> usize {
        let rate: usize = rate.into();
        match self {
            FrameSize::Ms2_5 => rate / 400,
            FrameSize::Ms5 => rate / 200,
            FrameSize::Ms10 => rate / 100,
            FrameSize::Ms20 => rate / 50,
            FrameSize::Ms40 => rate / 25,
            FrameSize::Ms60 => 3 * rate / 50,
            FrameSize::Ms80 => 4 * rate / 50,
            FrameSize::Ms100 => 5 * rate / 50,
            FrameSize::Ms120 => 6 * rate / 50,
        }
    }
}

impl FromStr for FrameSize {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2.5" => Ok(FrameSize::Ms2_5),
            "5" => Ok(FrameSize::Ms5),
            "10" => Ok(FrameSize::Ms10),
            "20" => Ok(FrameSize::Ms20),
            "40" => Ok(FrameSize::Ms40),
            "60" => Ok(FrameSize::Ms60),
            "80" => Ok(FrameSize::Ms80),
            "100" => Ok(FrameSize::Ms100),
            "120" => Ok(FrameSize::Ms120),
            _ => Err("Unsupported frame size"),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Complexity {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    #[default]
    C10,
}

impl FromStr for Complexity {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Complexity::C0),
            "1" => Ok(Complexity::C1),
            "2" => Ok(Complexity::C2),
            "3" => Ok(Complexity::C3),
            "4" => Ok(Complexity::C4),
            "5" => Ok(Complexity::C5),
            "6" => Ok(Complexity::C6),
            "7" => Ok(Complexity::C7),
            "8" => Ok(Complexity::C8),
            "9" => Ok(Complexity::C9),
            "10" => Ok(Complexity::C10),
            _ => Err("Unknown complexity"),
        }
    }
}

impl From<Complexity> for i32 {
    fn from(value: Complexity) -> Self {
        match value {
            Complexity::C0 => 0,
            Complexity::C1 => 1,
            Complexity::C2 => 2,
            Complexity::C3 => 3,
            Complexity::C4 => 4,
            Complexity::C5 => 5,
            Complexity::C6 => 6,
            Complexity::C7 => 7,
            Complexity::C8 => 8,
            Complexity::C9 => 9,
            Complexity::C10 => 10,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct CommonOptions {
    /// Enable SILK inband FEC
    pub inbandfec: bool,
    /// Simulate packet loss, in percent (0-100)
    pub loss: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct EncoderOptions {
    pub cbr: bool,
    pub bandwidth: Option<Bandwidth>,
    pub framesize: FrameSize,
    pub max_payload: usize,
    pub complexity: Complexity,
    pub forcemono: bool,
    pub cvbr: bool,
    pub delayed_decision: bool,
    pub dtx: bool,
    pub common: CommonOptions,
    // TODO: do we need to support undocumented options?
    // there's a bunch of them:
    // -sweep, -random_framesize, -sweep_max, -random_fec
    // -silk8k_test, -silk12k_test, -silk16k_test
    // -hybrid24k_test, -hybrid48k_test, -celt_test, -celt_hq_test
}

impl Default for EncoderOptions {
    fn default() -> Self {
        Self {
            cbr: false,
            bandwidth: None,
            framesize: FrameSize::Ms20,
            max_payload: MAX_PACKET,
            complexity: Complexity::C10,
            forcemono: false,
            cvbr: false,
            delayed_decision: false,
            dtx: false,
            common: CommonOptions::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EncodeArgs {
    pub application: Application,
    pub sample_rate: SampleRate,
    pub channels: Channels,
    pub bitrate: u32,
    pub options: EncoderOptions,
}

#[derive(Debug, Copy, Clone)]
pub struct DecodeArgs {
    pub sample_rate: SampleRate,
    pub channels: Channels,
    pub options: CommonOptions,
}
