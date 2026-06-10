#[derive(Default)]
pub struct DmgTimers {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    previous_and_result: bool,
}

pub trait TimingComponent  {
    fn new() -> Self where Self: Sized;

    fn div(&self) -> u16;
    fn set_div(&mut self, value: u8);
    fn inc_div(&mut self);

    fn tac(&self) -> u8;
    fn set_tac(&mut self, value: u8);

    fn tma(&self) -> u8;
    fn set_tma(&mut self, value: u8);

    fn tima (&self) -> u8;
    fn set_tima(&mut self, value: u8);
    fn inc_tima(&mut self);

    fn previous_and_result(&self) -> bool;
    fn set_next_and_result(&mut self, and_result: bool);

    fn and_result(&self) -> bool { 
        let enabled = (self.tac() & 0b100) > 0;
        let mask = 0b1
            << match self.tac() & 0b11 {
                0b00 => 9,
                0b01 => 3,
                0b10 => 5,
                0b11 => 7,
                _ => unreachable!(),
            };

        let kept_bit = (self.div() & mask) > 0;
        kept_bit && enabled
    }


    fn tick(&mut self) -> bool {
        self.inc_div();
        let mut overflowed = false;
        let and_result = self.and_result();
        if self.previous_and_result() && !and_result {
            self.inc_tima();
            if self.tima() == 0 {
                self.set_tima(self.tma());
                overflowed = true
            }
        }
        self.set_next_and_result(and_result);
        overflowed
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            DIV_ADDR => self.set_div(0),
            TIMA_ADDR => self.set_tima(value),
            TMA_ADDR => self.set_tma(value),
            TAC_ADDR => self.set_tac(value),
            _ => unreachable!(),
        }
    }

    fn read(&self, addr: u16) -> u8 {
        match addr {
            DIV_ADDR => (self.div() >> 8) as u8,
            TIMA_ADDR => self.tima(),
            TMA_ADDR => self.tma(),
            TAC_ADDR => self.tac(),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
pub struct CgbTimer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    previous_and_result: bool,
}

impl TimingComponent for CgbTimer {
    fn new() -> Self where Self: Sized { Self::default() }
    fn div(&self) -> u16 { self.div }
    fn set_div(&mut self, value: u8) { self.div = value as u16 }
    fn inc_div(&mut self) { self.div = self.div.wrapping_add(1) }

    fn tac(&self) -> u8 { self.tac }
    fn set_tac(&mut self, value: u8) { self.tac = value }

    fn tma(&self) -> u8 {self.tma }
    fn set_tma(&mut self, value: u8) { self.tma = value }

    fn tima (&self) -> u8 { self.tima }
    fn set_tima(&mut self, value: u8) { self.tima = value }
    fn inc_tima(&mut self) { self.tima = self.tima.wrapping_add(1); }

    fn previous_and_result(&self) -> bool { self.previous_and_result }
    fn set_next_and_result(&mut self, and_result: bool) { self.previous_and_result = and_result}
}

const DIV_ADDR: u16 = 0xFF04;
const TIMA_ADDR: u16 = 0xFF05;
const TMA_ADDR: u16 = 0xFF06;
const TAC_ADDR: u16 = 0xFF07;

impl TimingComponent for GbaTimers {
    fn new() -> Self where Self: Sized { Self::default() }
    fn div(&self) -> u16 { self.div }
    fn set_div(&mut self, value: u8) { self.div = value as u16 }
    fn inc_div(&mut self) { self.div = self.div.wrapping_add(1) }

    fn tac(&self) -> u8 { self.tac }
    fn set_tac(&mut self, value: u8) { self.tac = value }

    fn tma(&self) -> u8 {self.tma }
    fn set_tma(&mut self, value: u8) { self.tma = value }

    fn tima (&self) -> u8 { self.tima }
    fn set_tima(&mut self, value: u8) { self.tima = value }
    fn inc_tima(&mut self) { self.tima = self.tima.wrapping_add(1); }
    
    fn previous_and_result(&self) -> bool { self.previous_and_result }
    fn set_next_and_result(&mut self, and_result: bool) { self.previous_and_result = and_result}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let timer = DmgTimers::default();
        assert_eq!(timer.div, 0);
        assert_eq!(timer.tima, 0);
        assert_eq!(timer.tma, 0);
        assert_eq!(timer.tac, 0);
    }

    #[test]
    fn test_div_is_ticking_once_by_tick() {
        let mut timer = DmgTimers::default();

        for _ in 0..20 {
            timer.tick();
        }

        assert_eq!(timer.div, 20);
    }

    #[test]
    fn test_read_div_exposed_register_ticks_once_every_overflow() {
        let mut timer = DmgTimers::default();

        let tick_value = timer.read(DIV_ADDR);
        assert_eq!(tick_value, 0);
        for _ in 0..66000 {
            timer.tick();
        }
        let tick_value = timer.read(DIV_ADDR);
        assert_eq!(tick_value, 1);
    }

    #[test]
    fn test_div_reset_when_written_to() {
        let mut timer = DmgTimers::default();

        timer.tick();
        assert_ne!(timer.div, 0);
        timer.write(DIV_ADDR, 27);
        assert_eq!(timer.div, 0);
    }

    #[test]
    fn test_other_than_div_register_has_read_and_write_possible() {
        let vec = vec![TIMA_ADDR, TMA_ADDR, TAC_ADDR];
        let mut timers = DmgTimers::default();
        let value = 42;

        for addr in &vec {
            timers.write(*addr, value);
        }
        assert_eq!(value, timers.tima);
        assert_eq!(value, timers.tma);
        assert_eq!(value, timers.tac);

        for addr in vec {
            assert_eq!(value, timers.read(addr));
        }
    }

    #[test]
    fn test_tac_disable_tima_ticking() {
        let mut timers = DmgTimers::default();

        for _ in 0..66000 {
            timers.tick();
        }
        assert_eq!(0, timers.tima);

        timers.write(TAC_ADDR, 0b101);
        for _ in 0..66000 {
            timers.tick();
        }
        assert_ne!(0, timers.tima);
    }

    #[test]
    fn test_tima_tick_at_good_pace_bit_3() {
        let mut timers = DmgTimers::default();

        timers.write(TAC_ADDR, 0b101);
        for _ in 0..15 {
            timers.tick();
            assert_eq!(0, timers.tima);
        }
        timers.tick();
        assert_eq!(1, timers.tima);
    }

    #[test]
    fn test_tima_tick_at_good_pace_bit_5() {
        let mut timers = DmgTimers::default();

        timers.write(TAC_ADDR, 0b110);
        for _ in 0..63 {
            timers.tick();
            assert_eq!(0, timers.tima);
        }
        timers.tick();
        assert_eq!(1, timers.tima);
    }

    #[test]
    fn test_tima_tick_at_good_pace_bit_7() {
        let mut timers = DmgTimers::default();

        timers.write(TAC_ADDR, 0b111);
        for _ in 0..255 {
            timers.tick();
            assert_eq!(0, timers.tima);
        }
        timers.tick();
        assert_eq!(1, timers.tima);
    }

    #[test]
    fn test_tima_tick_at_good_pace_bit_9() {
        let mut timers = DmgTimers::default();

        timers.write(TAC_ADDR, 0b100);
        for _ in 0..1023 {
            timers.tick();
            assert_eq!(0, timers.tima);
        }
        timers.tick();
        assert_eq!(1, timers.tima);
    }

    #[test]
    fn test_timer_tick_overflowing_returns_true() {
        let mut timers = DmgTimers {
            tima: 0xFF,
            ..Default::default()
        };
        timers.write(TAC_ADDR, 0b101);
        for a in 0..15 {
            assert_eq!(false, timers.tick(), "overflow comming for : {a}");
        }
        assert_eq!(true, timers.tick());
    }

    #[test]
    fn test_timer_overflowing_reset_tima_to_tma() {
        let mut timers = DmgTimers {
            tima: 0xFF,
            tma: 0x53,
            ..Default::default()
        };
        timers.write(TAC_ADDR, 0b101);
        (0..=15).into_iter().for_each(|_| {timers.tick();});
        
        assert_eq!(timers.tima, timers.tma);
    }
}
