#[derive(Debug, PartialEq)]
pub(crate) struct RapportTexte {
    pub(crate) total: usize,
    pub(crate) valeurs_nulles: usize,
    pub(crate) textes_vides: usize,
    pub(crate) textes_renseignes: usize,
}

pub(crate) fn analyser_textes(valeurs: &[Option<String>]) -> RapportTexte {
    todo!()
}
