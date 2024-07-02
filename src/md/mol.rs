use super::VecF;
#[derive(Clone, Debug)]
pub struct Mol {
    r: VecF,
    rv: VecF,
    ra: VecF,
}
impl Mol {
    pub fn new(r: VecF, rv: VecF, ra: VecF) -> Self {
        Self { r, rv, ra }
    }
    pub fn zero() -> Self {
        Self::new(VecF::zero(), VecF::zero(), VecF::zero())
    }
    pub fn r(&self) -> VecF {
        self.r
    }
    pub fn rv(&self) -> VecF {
        self.rv
    }
    pub fn ra(&self) -> VecF {
        self.ra
    }
    pub fn mut_r(&mut self) -> &mut VecF {
        &mut self.r
    }
    pub fn mut_ra(&mut self) -> &mut VecF {
        &mut self.ra
    }
    pub fn set_r(&mut self, r: VecF) {
        self.r = r;
    }
    pub fn set_rv(&mut self, rv: VecF) {
        self.rv = rv;
    }
}
