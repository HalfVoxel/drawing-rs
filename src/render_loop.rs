use rendy::{
    command::Families,
    factory::{Config, Factory},
    wsi::winit::{Event, EventsLoop, Window, WindowBuilder, WindowEvent},
};
use std::time::{Duration, Instant};

use gfx_hal::Backend;

pub struct FPSLimiter {
    desired_delta_time: Duration,
    last_frame: Instant,
}

impl FPSLimiter {
    pub fn new(desired_fps: f64) -> Self {
        FPSLimiter {
            desired_delta_time: Duration::from_nanos((1_000_000_000.0 / desired_fps) as u64),
            last_frame: Instant::now(),
        }
    }

    pub fn sleep(&mut self) {
        let delta_time = self.last_frame.elapsed();
        if self.desired_delta_time > delta_time {
            let wait_time = self.desired_delta_time - delta_time;
            log::info!("Waiting {:#?}", wait_time);
            std::thread::sleep(wait_time);
        }
        self.last_frame = Instant::now();
    }
}
pub struct RenderLoop<B: Backend> {
    pub factory: Factory<B>,
    pub families: Families<B>,
    pub frame: u64,
    pub event_loop: EventsLoop,
}

impl<B> RenderLoop<B>
where
    B: Backend,
{
    pub fn new(config: Config, event_loop: EventsLoop) -> Self {
        let (factory, families): (Factory<B>, _) = rendy::factory::init(config).unwrap();
        let mut res = RenderLoop {
            factory,
            families,
            frame: 0,
            event_loop: event_loop,
        };
        res.event_loop.poll_events(|_| ());
        res
    }

    pub fn run(&mut self) -> bool {
        self.factory.maintain(&mut self.families);

        let mut should_close = false;

        self.event_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => should_close = true,
            _ => {
                // log::info!("Event {:#?}", event);
            }
        });
        
        self.frame += 1;

        !should_close
    }
}
