use super::{Fmd, MolDynRunner};
impl MolDynRunner {
    pub fn cycle(&mut self) {
        // reference:p21
        loop {
            self.single_step();
            if self.step_count >= self.step_limit {
                break;
            }
        }
    }
}
impl MolDynRunner {
    fn single_step(&mut self) {
        // reference:p21
        self.step_count += 1;
        self.time_now = self.step_count as Fmd * self.delta_t;
        self.leap_frog_step_1();
        self.apply_boundary_cond();
        self.compute_forces();
        self.leap_frog_step_2();
        self.eval_props();
        self.props_accum();
        if self.step_count % self.step_avg == 0 {
            self.props_avg();
            self.print_summary();
            self.props_zero();
        }
    }
}
