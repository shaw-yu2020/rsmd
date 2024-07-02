use super::{Fmd, Imd, Mol, MolDynRunner, Prop, VecF};
pub struct MolDynBuilder {
    delta_t: Fmd,
    density: Fmd,
    init_cell: VecF,
    step_avg: Imd,
    step_limit: Imd,
    temperature: Fmd,
}
impl Default for MolDynBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl MolDynBuilder {
    pub fn new() -> Self {
        Self {
            delta_t: 0.005,
            density: 0.8,
            init_cell: VecF::new(20.0, 20.0),
            step_avg: 100,
            step_limit: 10000,
            temperature: 1.0,
        }
    }
    pub fn delta_t(mut self, delta_t: Fmd) -> Self {
        self.delta_t = delta_t;
        self
    }
    pub fn density(mut self, density: Fmd) -> Self {
        self.density = density;
        self
    }
    pub fn init_cell(mut self, x: Fmd, y: Fmd) -> Self {
        self.init_cell = VecF::new(x, y);
        self
    }
    pub fn step_avg(mut self, step_avg: Imd) -> Self {
        self.step_avg = step_avg;
        self
    }
    pub fn step_limit(mut self, step_limit: Imd) -> Self {
        self.step_limit = step_limit;
        self
    }
    pub fn temperature(mut self, temperature: Fmd) -> Self {
        self.temperature = temperature;
        self
    }
}
impl MolDynBuilder {
    pub fn build(self) -> MolDynRunner {
        let n_dim = 2.0;
        // reference:p31
        let region = self.init_cell.unscale(self.density.sqrt());
        let n_mol = self.init_cell.prod();
        let vel_mag = (n_dim * (1.0 - 1.0 / n_mol) * self.temperature).sqrt();
        // reference:p28
        let gap = region / self.init_cell;
        let mut mol: Mol = Mol::zero();
        let mut mols: Vec<Mol> = Vec::new();
        for ny in 0..self.init_cell.y() as i32 {
            for nx in 0..self.init_cell.x() as i32 {
                mol.set_r(VecF::new(nx as Fmd + 0.5, ny as Fmd + 0.5) * gap - region.scale(0.5));
                mols.push(mol.clone());
            }
        }
        // reference:p28
        let mut v_sum = VecF::zero();
        for mol in mols.iter_mut() {
            mol.set_rv(VecF::rand().scale(vel_mag));
            v_sum += mol.rv();
        }
        for mol in mols.iter_mut() {
            mol.set_rv(mol.rv() - v_sum.unscale(n_mol));
        }
        MolDynRunner {
            n_dim: n_dim as Imd,
            n_mol,
            mols,
            delta_t: self.delta_t,
            time_now: 0.0,
            density: self.density,
            region,
            kin_energy: Prop::zero(),
            tot_energy: Prop::zero(),
            pressure: Prop::zero(),
            v_sum: VecF::zero(),
            vv_sum: 0.0,
            r_cut: 2.0_f64.powf(1.0 / 6.0) as Fmd, // reference:p31
            u_sum: 0.0,
            vir_sum: 0.0,
            step_avg: self.step_avg,
            step_count: 0,
            step_limit: self.step_limit,
        }
    }
}
