use eco_fairness_guard::{EcoFairnessGuard, GuardError as EcoGuardError, RohModel};
use vkernel::ViabilityKernel;

pub struct GuardianSet {
    pub neurorights_guard: NeurorightsGuard,
    pub roh_guard: RohGuard,
    pub eco_guard: EcoFairnessGuard,
    pub evolve_guard: EvolveGuard,
    // ...
}

impl GuardianSet {
    pub fn new_from_policies<P: AsRef<std::path::Path>>(policies_dir: P) -> anyhow::Result<Self> {
        let roh = RohModel::load(policies_dir.as_ref().join("rohmodel.aln"))?;
        let vkernel = ViabilityKernel::load(policies_dir.as_ref().join("vkernel.aln"))?;

        Ok(Self {
            neurorights_guard: NeurorightsGuard::new_from_dir(&policies_dir)?,
            roh_guard: RohGuard::new(roh.clone()),
            eco_guard: EcoFairnessGuard::new(roh, vkernel),
            evolve_guard: EvolveGuard::new_from_dir(&policies_dir)?,
            // ...
        })
    }
}
