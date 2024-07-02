type Fmd = f64;
type Imd = i32;
mod bound;
mod cycle;
mod eval;
mod force;
mod integ;
mod mol;
mod prop;
mod vec_f;
use mol::Mol;
use prop::Prop;
use vec_f::VecF;
mod mdb;
pub use mdb::MolDynBuilder;
pub struct MolDynRunner {
    n_dim: Imd,
    n_mol: Fmd,
    mols: Vec<Mol>,
    delta_t: Fmd,
    time_now: Fmd,
    density: Fmd,
    region: VecF,
    kin_energy: Prop,
    tot_energy: Prop,
    pressure: Prop,
    v_sum: VecF,
    vv_sum: Fmd,
    r_cut: Fmd,
    u_sum: Fmd,
    vir_sum: Fmd,
    step_avg: Imd,
    step_count: Imd,
    step_limit: Imd,
}
