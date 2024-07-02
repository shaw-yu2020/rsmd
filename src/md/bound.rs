use super::MolDynRunner;
impl MolDynRunner {
    pub fn apply_boundary_cond(&mut self) {
        // reference:p27
        for mol in self.mols.iter_mut() {
            mol.mut_r().wrap_all(&self.region);
        }
    }
}
