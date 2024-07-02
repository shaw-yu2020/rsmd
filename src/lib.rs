mod md;
pub use md::{MolDynBuilder, MolDynRunner};
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut md = md::MolDynBuilder::new()
            .delta_t(0.005)
            .density(0.8)
            .init_cell(20.0, 20.0)
            .step_avg(100)
            .step_limit(10000)
            .temperature(1.0)
            .build();
        md.cycle();
    }
}
