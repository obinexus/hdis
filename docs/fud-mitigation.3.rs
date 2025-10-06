// FUD mitigation through HDIS error scale (-12 to +12)
pub struct FUDAccessibility {
    fear_mitigation: ErrorLevel,      // -3 to -6: Self-correcting warnings
    uncertainty_resolution: ErrorLevel, // -6 to -10: Active compensation  
    doubt_elimination: ErrorLevel,    // -10 to -12: Human intervention
}

impl FUDAccessibility {
    pub fn mitigate_neurodivergent_stress(&self, system_state: &HDISState) -> AccessibilityResult {
        match system_state.error_level {
            -3..=0 => {
                // Optimal operation - system adapts transparently
                self.transparent_adaptation()
            }
            -6..=-4 => {
                // Self-correcting - system explains changes
                self.explained_adaptation()
            }
            -10..=-7 => {
                // Active compensation - user guides adaptation
                self.collaborative_adaptation()
            }
            -12..=-11 => {
                // Radiation zone - preserve state, await human help
                self.preservation_mode()
            }
            _ => self.default_behavior()
        }
    }
}