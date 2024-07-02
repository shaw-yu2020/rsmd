use super::{Fmd, MolDynRunner, VecF};
impl MolDynRunner {
    pub fn eval_props(&mut self) {
        // reference:p31
        self.v_sum = VecF::zero();
        self.vv_sum = 0.0;
        for mol in self.mols.iter_mut() {
            self.v_sum += mol.rv();
            self.vv_sum += mol.rv().len_sq();
        }
        self.kin_energy.set_val(0.5 * self.vv_sum / self.n_mol);
        self.tot_energy
            .set_val(self.kin_energy.val() + self.u_sum / self.n_mol);
        self.pressure.set_val(
            self.density * (self.vv_sum + self.vir_sum) / (self.n_mol * self.n_dim as Fmd),
        );
    }
    pub fn print_summary(&self) {
        // reference:p33
        println!(
            "{:5} {:8.4} {:7.4} {:7.4} {:7.4} {:7.4} {:7.4} {:7.4} {:7.4}",
            self.step_count,
            self.time_now,
            self.v_sum.sum() / self.n_mol,
            self.tot_energy.sum(),
            self.tot_energy.sum2(),
            self.kin_energy.sum(),
            self.kin_energy.sum2(),
            self.pressure.sum(),
            self.pressure.sum2()
        );
    }
}
