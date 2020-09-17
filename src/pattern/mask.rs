use crate::config::Config;
use crate::drawing::Pad;

pub fn calc(pads: &mut Vec<Pad>, config: &Config) {
    let mask_width = config.get_f64("pattern.minimum.mask-width").unwrap_or(0.0);
    let mask = config
        .get_f64("pattern.clearance.pad-to-mask")
        .unwrap_or(0.0);
    for i in 0..pads.len() {
        pads[i].mask = mask;
    }
    if pads.len() < 2 {
        return;
    }
    let last = pads.len() - 1;
    for i in 0..=last {
        for j in (i + 1)..=last {
            let p1 = &pads[i];
            let p2 = &pads[j];
            let hspace = (p2.origin.x - p1.origin.x).abs() - (p1.size.x + p2.size.x) / 2.0;
            let vspace = (p2.origin.y - p1.origin.y).abs() - (p1.size.y + p2.size.y) / 2.0;
            let space = hspace.max(vspace);

            // If pads are too near one to another, we need to shrink the mask. Minimum mask is zero (= copper).
            let mut pad_mask = mask;
            if (space - 2.0 * pad_mask) < mask_width {
                pad_mask = (space - mask_width) / 2.0;
                if pad_mask < 0.0 {
                    pad_mask = 0.0;
                }
            }
            if pad_mask < pads[i].mask {
                pads[i].mask = pad_mask;
            }
            if pad_mask < pads[j].mask {
                pads[j].mask = pad_mask;
            }
        }
    }
}
