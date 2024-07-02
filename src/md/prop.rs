use super::{Fmd, MolDynRunner};
#[derive(Debug)]
pub struct Prop {
    val: Fmd,
    sum: Fmd,
    sum2: Fmd,
}
impl Prop {
    pub fn new(val: Fmd, sum: Fmd, sum2: Fmd) -> Prop {
        Prop { val, sum, sum2 }
    }
    pub fn zero() -> Prop {
        Self::new(0.0, 0.0, 0.0)
    }
    pub fn val(&self) -> Fmd {
        self.val
    }
    pub fn sum(&self) -> Fmd {
        self.sum
    }
    pub fn sum2(&self) -> Fmd {
        self.sum2
    }
    pub fn set_val(&mut self, val: Fmd) {
        self.val = val;
    }
    fn prop_zero(&mut self) {
        // reference:p32
        self.sum = 0.0;
        self.sum2 = 0.0;
    }
    fn prop_accum(&mut self) {
        // reference:p32
        self.sum += self.val;
        self.sum2 += self.val.powi(2);
    }
    fn prop_avg(&mut self, n: Fmd) {
        // reference:p32
        self.sum /= n;
        self.sum2 = (self.sum2 / n - self.sum.powi(2)).max(0.0).sqrt();
    }
}
impl MolDynRunner {
    pub fn props_zero(&mut self) {
        // reference:p32
        self.tot_energy.prop_zero();
        self.kin_energy.prop_zero();
        self.pressure.prop_zero();
    }
    pub fn props_accum(&mut self) {
        // reference:p32
        self.tot_energy.prop_accum();
        self.kin_energy.prop_accum();
        self.pressure.prop_accum();
    }
    pub fn props_avg(&mut self) {
        // reference:p32
        self.tot_energy.prop_avg(self.step_avg as Fmd);
        self.kin_energy.prop_avg(self.step_avg as Fmd);
        self.pressure.prop_avg(self.step_avg as Fmd);
    }
}
