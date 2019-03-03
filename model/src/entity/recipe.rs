use crate::ware::Ware;

#[derive(Clone, Debug, Default)]
pub struct Recipe {
    inputs: Vec<Ware>,
    outputs: Vec<Ware>,
}

impl Recipe {
    pub fn new(inputs: Vec<Ware>, outputs: Vec<Ware>) -> Self {
        Recipe { inputs, outputs }
    }

    pub fn inputs(&self) -> &[Ware] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[Ware] {
        &self.outputs
    }
}
