use crate::frontend::ApiResponse;
use crate::simulation::simulation_state::SimulationState;

pub mod simulation_state;

pub struct Simulator {
    simulation_state: SimulationState,
    history: ApiResponse,
}

impl From<SimulationState> for Simulator {
    fn from(simulation_state: SimulationState) -> Self {
        Simulator {simulation_state, history: ApiResponse::default()}
    }
}

impl Simulator {
    pub fn update(&mut self) -> ApiResponse {


        self.history.clone()
    }
}