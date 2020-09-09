#[derive(Debug, Default)]
pub struct Ipc7351B {
    pub pad_distance: f64,
    pub pad_size: (f64, f64),
    pub courtyard: f64,

    lead_span: (f64, f64),
    lead_len: (f64, f64),
    lead_width: (f64, f64),
    lead_height: (f64, f64),
    toe: f64,
    heel: f64,
    side: f64,
    fab_tol: f64,
    place_tol: f64,
}

impl Ipc7351B {
    /// Sets a lead span, i.e. a distance from the one lead edge to the opposite lead edge.
    pub fn lead_span(mut self, span: (f64, f64)) -> Self {
        self.lead_span = span;
        self
    }

    /// Sets the lead length.
    pub fn lead_len(mut self, len: (f64, f64)) -> Self {
        self.lead_len = len;
        self
    }

    /// Sets the lead width.
    pub fn lead_width(mut self, width: (f64, f64)) -> Self {
        self.lead_width = width;
        self
    }

    /// Sets the lead height.
    pub fn lead_height(mut self, height: (f64, f64)) -> Self {
        self.lead_height = height;
        self
    }

    /// Sets toe, heel, and side goals.
    pub fn goals(mut self, package: &str, density_level: &str) -> Self {
        let i = match density_level {
            "M" | "m" | "most" => 0,
            "L" | "l" | "least" => 2,
            _ => 1, // "N", "nominal"
        };
        let (toe, heel, side, courtyard) = match package {
            "chip" => {
                let h = (self.lead_height.0 + self.lead_height.1) / 2.0;
                (0.5 * h, 0.1 * h, 0.15 * h, 0.4 * h)
            }
            _ => (
                vec![0.55, 0.35, 0.15][i],
                vec![0.45, 0.35, 0.25][i],
                vec![0.05, 0.03, 0.01][i],
                vec![0.5, 0.25, 0.12][i],
            ),
        };
        self.toe = toe;
        self.heel = heel;
        self.side = side;
        self.courtyard = courtyard;
        self
    }

    /// Sets fabrication and placement tolerances.
    pub fn tols(mut self, fab: f64, place: f64) -> Self {
        self.fab_tol = fab;
        self.place_tol = place;
        self
    }

    pub fn calc(mut self) -> Self {
        let span_tol = self.lead_span.1 - self.lead_span.0;
        let len_tol = self.lead_len.1 - self.lead_len.0;
        let width_tol = self.lead_width.1 - self.lead_width.0;

        let s_min = self.lead_span.0 - 2.0 * self.lead_len.1;
        let s_max = self.lead_span.1 - 2.0 * self.lead_len.0;
        let s_tol = s_max - s_min;
        let s_tol_rms = (span_tol * span_tol + 2.0 * len_tol * len_tol).sqrt();
        let s_diff = s_tol - s_tol_rms;

        let new_s_min = s_min + s_diff / 2.0;
        let new_s_max = s_max - s_diff / 2.0;
        let new_s_tol = new_s_max - new_s_min;

        let toe_tol = (span_tol * span_tol
            + 4.0 * self.fab_tol * self.fab_tol
            + 4.0 * self.place_tol * self.place_tol)
            .sqrt();
        let heel_tol = (new_s_tol * new_s_tol
            + 4.0 * self.fab_tol * self.fab_tol
            + 4.0 * self.place_tol * self.place_tol)
            .sqrt();
        let side_tol = (width_tol * width_tol
            + 4.0 * self.fab_tol * self.fab_tol
            + 4.0 * self.place_tol * self.place_tol)
            .sqrt();

        let z_max = self.lead_span.0 + 2.0 * self.toe + toe_tol;
        let g_min = new_s_max - 2.0 * self.heel - heel_tol;
        let y_ref = self.lead_width.0 + 2.0 * self.side + side_tol;

        self.pad_size = (
            Self::round_size((z_max - g_min) / 2.0),
            Self::round_size(y_ref),
        );
        self.pad_distance = Self::round_place((z_max + g_min) / 2.0);

        self
    }

    fn round_place(value: f64) -> f64 {
        let factor = 0.02;
        (value / factor).round() * factor
    }

    fn round_size(value: f64) -> f64 {
        let factor = 0.01;
        (value / factor).round() * factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipc() {
        // Use calculator from pcblibraries.com for validation
        let ipc = Ipc7351B::default()
            .lead_span((5.85, 6.2))
            .lead_width((0.31, 0.51))
            .lead_len((0.4, 1.27))
            .goals("default", "N")
            .tols(0.05, 0.025) // TODO: Replace fabrication and placement tolerances by the config values
            .calc();

        assert_eq!(ipc.pad_distance, 4.96);
        assert_eq!(ipc.pad_size, (1.95, 0.6));
    }
}
