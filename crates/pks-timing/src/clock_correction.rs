const MAX_CORRECTION_NS: i64 = 10_000_000;

#[derive(Debug, Clone, Copy)]
pub struct ClockCorrectionController {
    proportional_gain: f64,
    integral_gain: f64,
    integral_error_ns: f64,
    last_offset_ns: i64,
    last_correction_ns: i64,
}

impl ClockCorrectionController {
    pub fn new(proportional_gain: f64, integral_gain: f64) -> Self {
        Self {
            proportional_gain,
            integral_gain,
            integral_error_ns: 0.0,
            last_offset_ns: 0,
            last_correction_ns: 0,
        }
    }

    pub fn tick(&mut self, measured_offset_ns: i64) -> i64 {
        let error_ns = measured_offset_ns as f64;
        self.integral_error_ns += error_ns;
        let correction_ns =
            self.proportional_gain * error_ns + self.integral_gain * self.integral_error_ns;
        self.last_offset_ns = measured_offset_ns;
        self.last_correction_ns = correction_ns
            .round()
            .clamp(-MAX_CORRECTION_NS as f64, MAX_CORRECTION_NS as f64)
            as i64;
        self.last_correction_ns
    }

    pub fn last_offset_ns(&self) -> i64 {
        self.last_offset_ns
    }
    pub fn last_correction_ns(&self) -> i64 {
        self.last_correction_ns
    }
    pub fn integral_error_ns(&self) -> f64 {
        self.integral_error_ns
    }

    pub fn integral_ns(&self) -> f64 {
        self.integral_error_ns
    }
}

impl Default for ClockCorrectionController {
    fn default() -> Self {
        Self::new(0.1, 0.001)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_zero_offset_when_corrected_then_correction_is_zero() {
        let mut controller = ClockCorrectionController::default();
        assert_eq!(controller.tick(0), 0);
        assert_eq!(controller.last_offset_ns(), 0);
    }

    #[test]
    fn given_positive_offset_when_corrected_then_correction_is_positive() {
        let mut controller = ClockCorrectionController::default();
        assert!(controller.tick(1_000_000) > 0);
    }

    #[test]
    fn given_negative_offset_when_corrected_then_correction_is_negative() {
        let mut controller = ClockCorrectionController::default();
        assert!(controller.tick(-1_000_000) < 0);
    }

    #[test]
    fn given_large_offset_when_corrected_then_correction_is_clamped() {
        let mut controller = ClockCorrectionController::default();
        assert!(controller.tick(100_000_000).abs() <= MAX_CORRECTION_NS);
    }

    #[test]
    fn given_repeated_offset_when_corrected_then_integral_accumulates() {
        let mut controller = ClockCorrectionController::new(0.0, 1.0);
        controller.tick(1_000);
        controller.tick(1_000);
        assert!(controller.integral_error_ns() > 1_000.0);
    }
}
