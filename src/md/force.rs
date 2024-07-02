use super::{Fmd, MolDynRunner, VecF};
impl MolDynRunner {
    pub fn compute_forces(&mut self) {
        // reference:p26
        let mut dr: VecF;
        let mut fc_val: Fmd;
        let mut rr: Fmd;
        let mut rri: Fmd;
        let mut rri3: Fmd;
        let rr_cut = self.r_cut.powi(2);
        for mol in self.mols.iter_mut() {
            mol.mut_ra().set(0.0, 0.0);
        }
        let num_mol = self.n_mol as usize;
        self.u_sum = 0.0;
        self.vir_sum = 0.0;
        for j1 in 0..num_mol - 1 {
            for j2 in j1 + 1..num_mol {
                dr = self.mols.get(j1).unwrap().r() - self.mols.get(j2).unwrap().r();
                dr.wrap_all(&self.region);
                rr = dr.len_sq();
                if rr < rr_cut {
                    rri = 1.0 / rr;
                    rri3 = rri.powi(3);
                    fc_val = 48.0 * rri3 * (rri3 - 0.5) * rri;
                    *self.mols.get_mut(j1).unwrap().mut_ra() += dr.scale(fc_val);
                    *self.mols.get_mut(j2).unwrap().mut_ra() -= dr.scale(fc_val);
                    self.u_sum += 4.0 * rri3 * (rri3 - 1.0) + 1.0;
                    self.vir_sum += fc_val * rr;
                }
            }
        }
    }
}
