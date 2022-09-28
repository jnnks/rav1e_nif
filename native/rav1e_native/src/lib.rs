use std::sync::{Mutex, MutexGuard};

use rav1e::{InvalidConfig, EncoderConfig, EncoderStatus};
use rustler::{Env, Error, ResourceArc, Term};

mod encoder_config;
use encoder_config::ExEncoderConfig;

pub struct Rav1e {
    settings: EncoderConfig,
    pub context: Mutex<rav1e::Context<u8>>
}

impl Rav1e {
    pub fn new(settings: EncoderConfig) -> Result<Self, InvalidConfig> {
      
        let config: rav1e::Config = rav1e::Config::new()
            .with_threads(4)
            .with_encoder_config(settings);

        let context: rav1e::Context<u8> = config.new_context()?;
    
        Ok(Rav1e {
            settings,
            context: Mutex::new(context)
        })
    }

    pub fn encode(&self, yuv_frame: [&[u8]; 3]) -> Result<(u64, Vec<u8>), EncoderStatus> {
        let mut context: MutexGuard<'_, rav1e::Context<u8>> = match self.context.lock() {
            Ok(context) => context,
            Err(_) => return Err(EncoderStatus::Failure),
        };
    
        // convert to rav1e frame
        let mut frame = context.new_frame();
        frame.planes[0].copy_from_raw_u8(yuv_frame[0], self.settings.width as usize, 1);
        frame.planes[1].copy_from_raw_u8(yuv_frame[1], self.settings.width as usize, 1);
        frame.planes[2].copy_from_raw_u8(yuv_frame[2], self.settings.width as usize, 1);
        
        context.send_frame(frame).expect("cannot send frame");
        
        match context.receive_packet() {
            Ok(packet) => Ok( (packet.input_frameno, packet.data) ),
            Err(e) => Err(e),
        }
    }

    pub fn flush(&self) -> Result<Vec<(u64, Vec<u8>)>, EncoderStatus> {
        let mut context: MutexGuard<'_, rav1e::Context<u8>> = match self.context.lock() {
            Ok(context) => context,
            Err(_) => return Err(EncoderStatus::Failure),
        };

        context.send_frame(None).expect("cannot send frame");

        let mut frames = vec![];
        while let Ok(packet) = context.receive_packet() {
            frames.push((packet.input_frameno, packet.data));
        }

        Ok( frames )
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn init(settings: ExEncoderConfig) -> Result<ResourceArc<Rav1e>, Error> {
    match Rav1e::new(settings.to_rav1e()) {
        Ok(rav1e) => Ok(ResourceArc::new(rav1e)),
        Err(e) => Err(Error::Term(Box::new(format!("Failed to initialize encoder: {}", e)))),
    }
}

fn av1_packet_to_binary<'a>(env: Env<'a>, data: &Vec<u8>) -> rustler::Binary<'a> {
    let mut bin_pack = rustler::OwnedBinary::new(data.len()).expect("");
    bin_pack.copy_from_slice(&data);
    rustler::Binary::from_owned(bin_pack, env)
}

#[rustler::nif(schedule = "DirtyCpu")]
fn encode<'a>(env: Env<'a>, rav1e: ResourceArc<Rav1e>, y_plane: rustler::Binary, cb_plane: rustler::Binary, cr_plane: rustler::Binary) -> Result<(u64, rustler::Binary<'a>), Error> {
    match rav1e.encode([y_plane.as_slice(), cb_plane.as_slice(), cr_plane.as_slice()]) {
        Ok((idx, vec_packet)) => {
            Ok((idx, av1_packet_to_binary(env, &vec_packet)))
        },
        Err(EncoderStatus::Encoded) => Err(Error::Atom("encoded")),
        Err(EncoderStatus::EnoughData) => Err(Error::Atom("enough_data")),
        Err(EncoderStatus::Failure) => Err(Error::Atom("failure")),
        Err(EncoderStatus::LimitReached) => Err(Error::Atom("limit_reached")),
        Err(EncoderStatus::NeedMoreData) => Err(Error::Atom("need_more_data")),
        Err(EncoderStatus::NotReady) => Err(Error::Atom("not_ready"))
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn flush(env: Env, rav1e: ResourceArc<Rav1e>) -> Result<Vec<(u64, rustler::Binary)>, Error> {
    match rav1e.flush() {
        Ok(frames) => {
            let frames = frames.iter().map(|(idx, vec_packet)| {
                (*idx, av1_packet_to_binary(env, vec_packet))
            }).collect();
            Ok(frames)
        },
        Err(EncoderStatus::Encoded) => Err(Error::Atom("encoded")),
        Err(EncoderStatus::EnoughData) => Err(Error::Atom("enough_data")),
        Err(EncoderStatus::Failure) => Err(Error::Atom("failure")),
        Err(EncoderStatus::LimitReached) => Err(Error::Atom("limit_reached")),
        Err(EncoderStatus::NeedMoreData) => Err(Error::Atom("need_more_data")),
        Err(EncoderStatus::NotReady) => Err(Error::Atom("not_ready"))
    }
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(Rav1e, env);
    true
}

rustler::init!("Elixir.Rav1e.Native", [init, encode, flush], load = load);
