use crate::instructions::load::write_memory;
use crate::{defines::Cpu, implemenation::Reg8, implemenation::Reg16};

pub fn inc_r8<Dest: Reg8>(cpu: &mut Cpu) {
    cpu.set_r8::<Dest>(cpu.get_r8::<Dest>().wrapping_add(1));
}

//Increment the value pointed BY the r16 and update it
pub fn inc_addr<Addr: Reg16, Value: Reg8>(cpu: &mut Cpu) {
    inc_r8::<Value>(cpu);
    write_memory::<Addr, Value>(cpu);
}

pub fn dec_r8<Reg: Reg8>(cpu: &mut Cpu) {
    cpu.set_r8::<Reg>(cpu.get_r8::<Reg>().wrapping_sub(1));
}

//Decrement the value pointed BY the r16 and update it
pub fn dec_addr<Addr: Reg16, Value: Reg8>(cpu: &mut Cpu) {
    dec_r8::<Value>(cpu);
    write_memory::<Addr, Value>(cpu);
}

pub fn inc_r16<Dest: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Dest>(cpu.get_r16::<Dest>().wrapping_add(1));
}

pub fn dec_r16<Dest: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Dest>(cpu.get_r16::<Dest>().wrapping_sub(1));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defines::Cpu;
    use crate::implemenation::{A, B, HL, SP};

    fn cpu() -> Cpu {
        Cpu {
            queue: &[],
            r8: [0; 14],
            flags: 0,
            instructions_list: vec![],
            op_index: 0,
            bus: [0; 0x10000],
        }
    }

    #[test]
    fn inc_r8_basic() {
        let mut c = cpu();
        c.set_r8::<B>(41);
        inc_r8::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 42);
    }

    #[test]
    fn inc_r8_wraps() {
        let mut c = cpu();
        c.set_r8::<A>(0xFF);
        inc_r8::<A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0x00);
    }

    #[test]
    fn dec_r8_basic() {
        let mut c = cpu();
        c.set_r8::<B>(10);
        dec_r8::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 9);
    }

    #[test]
    fn dec_r8_wraps() {
        let mut c = cpu();
        c.set_r8::<A>(0x00);
        dec_r8::<A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0xFF);
    }

    #[test]
    fn inc_r16_basic() {
        let mut c = cpu();
        c.set_r16::<HL>(0x1234);
        inc_r16::<HL>(&mut c);
        assert_eq!(c.get_r16::<HL>(), 0x1235);
    }

    #[test]
    fn inc_r16_wraps() {
        let mut c = cpu();
        c.set_r16::<SP>(0xFFFF);
        inc_r16::<SP>(&mut c);
        assert_eq!(c.get_r16::<SP>(), 0x0000);
    }

    #[test]
    fn dec_r16_basic() {
        let mut c = cpu();
        c.set_r16::<HL>(0x1235);
        dec_r16::<HL>(&mut c);
        assert_eq!(c.get_r16::<HL>(), 0x1234);
    }

    #[test]
    fn dec_r16_wraps() {
        let mut c = cpu();
        c.set_r16::<SP>(0x0000);
        dec_r16::<SP>(&mut c);
        assert_eq!(c.get_r16::<SP>(), 0xFFFF);
    }

    #[test]
    fn inc_addr_increments_memory() {
        use crate::implemenation::Z;
        let mut c = cpu();
        c.set_r16::<HL>(0x8000);
        c.bus[0x8000] = 41;
        c.set_r8::<Z>(c.bus[0x8000]);
        inc_addr::<HL, Z>(&mut c);
        assert_eq!(c.bus[0x8000], 42);
    }

    #[test]
    fn dec_addr_decrements_memory() {
        use crate::implemenation::Z;
        let mut c = cpu();
        c.set_r16::<HL>(0x8000);
        c.bus[0x8000] = 10;
        c.set_r8::<Z>(c.bus[0x8000]);
        dec_addr::<HL, Z>(&mut c);
        assert_eq!(c.bus[0x8000], 9);
    }
}
