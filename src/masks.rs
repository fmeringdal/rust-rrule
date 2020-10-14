// =============================================================================
// Date masks
// =============================================================================

// Every mask is 7 days longer to handle cross-year weekly periods.

pub struct Masks {
    pub WDAY: Vec<usize>,
    pub M365: Vec<usize>,
    pub M365RANGE: Vec<usize>,
    pub M366: Vec<usize>,
    pub M366RANGE: Vec<usize>,
    pub MDAY365: Vec<usize>,
    pub MDAY366: Vec<usize>,
    pub NMDAY365: Vec<isize>,
    pub NMDAY366: Vec<isize>,
}

impl Masks {
    pub fn new() -> Self {
        let M28: Vec<usize> = (1..29).collect();
        let M29: Vec<usize> = (1..30).collect();
        let M30: Vec<usize> = (1..31).collect();
        let M31: Vec<usize> = (1..32).collect();

        let NM28: Vec<isize> = (-28..0).collect();
        let NM29: Vec<isize> = (-29..0).collect();
        let NM30: Vec<isize> = (-30..0).collect();
        let NM31: Vec<isize> = (-31..0).collect();

        Self {
            WDAY: vec![(0..7).collect::<Vec<usize>>(); 55]
                .into_iter()
                .flatten()
                .collect(),
            M365: vec![
                vec![1; 31],
                vec![2; 28],
                vec![3; 31],
                vec![4; 30],
                vec![5; 31],
                vec![6; 30],
                vec![7; 31],
                vec![8; 31],
                vec![9; 30],
                vec![10; 31],
                vec![11; 30],
                vec![12; 31],
                vec![1; 7],
            ]
            .into_iter()
            .flatten()
            .collect(),
            M365RANGE: vec![0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365],
            MDAY366: vec![
                M31.clone(),
                M29.clone(),
                M31.clone(),
                M30.clone(),
                M31.clone(),
                M30.clone(),
                M31.clone(),
                M31.clone(),
                M30.clone(),
                M31.clone(),
                M30.clone(),
                M31.clone(),
                Vec::from(&M31.clone()[0..7]),
            ]
            .into_iter()
            .flatten()
            .collect(),
            M366RANGE: vec![0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
            MDAY365: vec![
                M31.clone(),
                M28.clone(),
                M31.clone(),
                M30.clone(),
                M31.clone(),
                M30.clone(),
                M31.clone(),
                M31.clone(),
                M30.clone(),
                M31.clone(),
                M30.clone(),
                M31.clone(),
                Vec::from(&M31.clone()[0..7]),
            ]
            .into_iter()
            .flatten()
            .collect(),
            M366: vec![
                vec![1; 31],
                vec![2; 29],
                vec![3; 31],
                vec![4; 30],
                vec![5; 31],
                vec![6; 30],
                vec![7; 31],
                vec![8; 31],
                vec![9; 30],
                vec![10; 31],
                vec![11; 30],
                vec![12; 31],
                vec![1; 7],
            ]
            .into_iter()
            .flatten()
            .collect(),
            NMDAY365: vec![
                NM31.clone(),
                NM28.clone(),
                NM31.clone(),
                NM30.clone(),
                NM31.clone(),
                NM30.clone(),
                NM31.clone(),
                NM31.clone(),
                NM30.clone(),
                NM31.clone(),
                NM30.clone(),
                NM31.clone(),
                Vec::from(&NM31.clone()[0..7]),
            ]
            .into_iter()
            .flatten()
            .collect(),
            NMDAY366: vec![
                NM31.clone(),
                NM29.clone(),
                NM31.clone(),
                NM30.clone(),
                NM31.clone(),
                NM30.clone(),
                NM31.clone(),
                NM31.clone(),
                NM30.clone(),
                NM31.clone(),
                NM30.clone(),
                NM31.clone(),
                Vec::from(&NM31.clone()[0..7]),
            ]
            .into_iter()
            .flatten()
            .collect(),
        }
    }
}
