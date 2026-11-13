#[derive(Debug, PartialEq)]
pub(crate) struct RapportTexte {
    pub(crate) total: usize,
    pub(crate) valeurs_nulles: usize,
    pub(crate) textes_vides: usize,
    pub(crate) textes_renseignes: usize,
}
#[derive(Debug, PartialEq)]
pub(crate) enum ValeurAttribut {
    Absente,
    Nulle,
    Text(String),
    Autre,
}

pub(crate) fn analyser_textes(valeurs: &[Option<String>]) -> RapportTexte {
    let mut rapport = RapportTexte {
        total: valeurs.len(),
        valeurs_nulles: 0,
        textes_vides: 0,
        textes_renseignes: 0,
    };

    for valeur in valeurs {
        match valeur {
            None => {
                rapport.valeurs_nulles += 1;
            }
            Some(texte) => {
                if texte.trim().is_empty() {
                    rapport.textes_vides += 1;
                } else {
                    rapport.textes_renseignes += 1;
                }
            }
        }
    }

    rapport
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyse_un_melange_de_valeurs() {
        let valeurs = [
            Some(String::from("École")),
            None,
            Some(String::from("")),
            Some(String::from("    ")),
            Some(String::from(" Mairie ")),
        ];

        let rapport = analyser_textes(&valeurs);

        assert_eq!(rapport.total, 5);
    }

    #[test]
    fn analyse_avec_valeurs_vide() {
        let rapport = analyser_textes(&[]);

        assert_eq!(rapport.total, 0);
        assert_eq!(rapport.valeurs_nulles, 0);
        assert_eq!(rapport.textes_vides, 0);
        assert_eq!(rapport.textes_renseignes, 0);
    }
}
