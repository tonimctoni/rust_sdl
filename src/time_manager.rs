
use sdl2::TimerSubsystem;


pub struct TimeManager {
    timer: TimerSubsystem,
    last: u32
}

impl TimeManager {
    pub fn init(mut timer: TimerSubsystem) -> TimeManager {
        let last=timer.ticks();
        TimeManager{timer: timer, last: last}
    }

    pub fn cap_fps(&mut self){
        const MS_PER_FRAME: u32 = 1_000/60;
        let elapsed_ms=self.timer.ticks()-self.last;
        if elapsed_ms < MS_PER_FRAME{
            self.timer.delay(MS_PER_FRAME-elapsed_ms);
        }
        self.last=self.timer.ticks();
    }
}
