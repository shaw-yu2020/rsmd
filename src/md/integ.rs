use super::MolDynRunner;
impl MolDynRunner {
    pub fn leap_frog_step_1(&mut self) {
        // reference:p27
        for mol in self.mols.iter_mut() {
            mol.set_rv(mol.rv() + mol.ra().scale(0.5 * self.delta_t));
            mol.set_r(mol.r() + mol.rv().scale(self.delta_t));
        }
    }
    pub fn leap_frog_step_2(&mut self) {
        // reference:p27
        for mol in self.mols.iter_mut() {
            mol.set_rv(mol.rv() + mol.ra().scale(0.5 * self.delta_t));
        }
    }
}
