use once_cell::sync::Lazy;

// =============================================================================
// Date masks
// =============================================================================

// Every mask is 7 days longer to handle cross-year weekly periods.

pub static MASKS: Lazy<Masks> = Lazy::new(Masks::default);

#[derive(Clone)]
pub struct Masks {
    pub wday: Vec<usize>,
    pub m365: Vec<usize>,
    pub m365range: Vec<usize>,
    pub m366: Vec<usize>,
    pub m366range: Vec<usize>,
    pub mday365: Vec<usize>,
    pub mday366: Vec<usize>,
    pub nmday365: Vec<isize>,
    pub nmday366: Vec<isize>,
}

impl Default for Masks {
    fn default() -> Self {
        let m28: Vec<usize> = (1..29).collect();
        let m29: Vec<usize> = (1..30).collect();
        let m30: Vec<usize> = (1..31).collect();
        let m31: Vec<usize> = (1..32).collect();

        let nm28: Vec<isize> = (-28..0).collect();
        let nm29: Vec<isize> = (-29..0).collect();
        let nm30: Vec<isize> = (-30..0).collect();
        let nm31: Vec<isize> = (-31..0).collect();

        Self {
            wday: vec![(0..7).collect::<Vec<usize>>(); 55]
                .into_iter()
                .flatten()
                .collect(),
            m365: vec![
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
            m365range: vec![0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365],
            mday366: vec![
                m31.clone(),
                m29,
                m31.clone(),
                m30.clone(),
                m31.clone(),
                m30.clone(),
                m31.clone(),
                m31.clone(),
                m30.clone(),
                m31.clone(),
                m30.clone(),
                m31.clone(),
                Vec::from(&m31[0..7]),
            ]
            .into_iter()
            .flatten()
            .collect(),
            m366range: vec![0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
            mday365: vec![
                m31.clone(),
                m28,
                m31.clone(),
                m30.clone(),
                m31.clone(),
                m30.clone(),
                m31.clone(),
                m31.clone(),
                m30.clone(),
                m31.clone(),
                m30,
                m31.clone(),
                Vec::from(&m31[0..7]),
            ]
            .into_iter()
            .flatten()
            .collect(),
            m366: vec![
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
            nmday365: vec![
                nm31.clone(),
                nm28,
                nm31.clone(),
                nm30.clone(),
                nm31.clone(),
                nm30.clone(),
                nm31.clone(),
                nm31.clone(),
                nm30.clone(),
                nm31.clone(),
                nm30.clone(),
                nm31.clone(),
                Vec::from(&nm31[0..7]),
            ]
            .into_iter()
            .flatten()
            .collect(),
            nmday366: vec![
                nm31.clone(),
                nm29,
                nm31.clone(),
                nm30.clone(),
                nm31.clone(),
                nm30.clone(),
                nm31.clone(),
                nm31.clone(),
                nm30.clone(),
                nm31.clone(),
                nm30,
                nm31.clone(),
                Vec::from(&nm31[0..7]),
            ]
            .into_iter()
            .flatten()
            .collect(),
        }
    }
}
