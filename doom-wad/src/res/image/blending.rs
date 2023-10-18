pub(super) type BlendFunction = fn(&[u8], &[u8], Option<usize>) -> Box<[u8]>;

pub(super) fn mix(a: &[u8], b: &[u8], alpha_index: Option<usize>) -> Box<[u8]> {
    // Based on https://github.com/Talon1024/paledit/blob/5170cd3d/src/app/palette-model/rgb.ts#L35
    let b_percent = match alpha_index {
        None => 1.,
        Some(alpha_index) => b.get(alpha_index).copied().unwrap() as f32 / 255.
    };
    let a_percent = 1. - b_percent;
    Box::from_iter(a.iter().copied().zip(b.iter().copied()).map(|(coa, cob)| {
        let coa = coa as f32 / 255.;
        let cob = cob as f32 / 255.;
        ((coa * a_percent + cob * b_percent) * 255.) as u8
    }))
}

pub(super) fn add(a: &[u8], b: &[u8], alpha_index: Option<usize>) -> Box<[u8]> {
    // Based on https://github.com/Talon1024/paledit/blob/5170cd3d/src/app/palette-model/rgb.ts#L35
    let b_percent = match alpha_index {
        None => 1.,
        Some(alpha_index) => b.get(alpha_index).copied().unwrap() as f32 / 255.
    };
    Box::from_iter(a.iter().copied().zip(b.iter().copied()).map(|(coa, cob)| {
        let coa = coa as f32 / 255.;
        let cob = cob as f32 / 255.;
        ((coa + cob * b_percent) * 255.) as u8
    }))
}
