#[derive(Debug, PartialEq)]
pub(crate) struct RapportTexte {
    pub(crate) total: usize,
    pub(crate) valeurs_nulles: usize,
    pub(crate) textes_vides: usize,
    pub(crate) textes_renseignes: usize,
}

#[derive(Debug, PartialEq)]
pub(crate) struct RapportAttribut {
    pub(crate) total: usize,
    pub(crate) valeurs_absentes: usize,
    pub(crate) valeurs_nulles: usize,
    pub(crate) textes_vides: usize,
    pub(crate) textes_renseignes: usize,
    pub(crate) autres_types: usize,
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

pub(crate) fn analyser_attributs(valeurs: &[ValeurAttribut]) -> RapportAttribut {
    let mut rapport = RapportAttribut {
        total: valeurs.len(),
        valeurs_absentes: 0,
        valeurs_nulles: 0,
        textes_vides: 0,
        textes_renseignes: 0,
        autres_types: 0,
    };

    for valeur in valeurs {
        match valeur {
            ValeurAttribut::Absente => rapport.valeurs_absentes += 1,
            ValeurAttribut::Nulle => rapport.valeurs_nulles += 1,
            ValeurAttribut::Text(texte) => {
                if texte.trim().is_empty() {
                    rapport.textes_vides += 1;
                } else {
                    rapport.textes_renseignes += 1;
                }
            }
            ValeurAttribut::Autre => rapport.autres_types += 1,
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

    #[test]
    fn analyse_avec_les_variantes() {
        let valeurs = [
            ValeurAttribut::Absente,
            ValeurAttribut::Nulle,
            ValeurAttribut::Text(String::from("École")),
            ValeurAttribut::Text(String::from("   ")),
            ValeurAttribut::Autre,
        ];

        for valeur in &valeurs {
            match valeur {
                ValeurAttribut::Absente => println!("Attribut absent."),
                ValeurAttribut::Nulle => println!("Attribut nul."),
                ValeurAttribut::Text(texte) => {
                    if texte.trim().is_empty() {
                        println!("Valeur vide");
                    } else {
                        println!("Valeur : {texte}");
                    }
                }
                ValeurAttribut::Autre => println!("Autre attribut"),
            }
        }
    }

    #[test]
    fn test_fonction_analyse_attributs() {
        let valeurs = [
            ValeurAttribut::Absente,
            ValeurAttribut::Nulle,
            ValeurAttribut::Text(String::from("École")),
            ValeurAttribut::Text(String::from("   ")),
            ValeurAttribut::Autre,
        ];

        let rapport = analyser_attributs(&valeurs);

        assert_eq!(rapport.total, 5);
        assert_eq!(rapport.valeurs_absentes, 1);
        assert_eq!(rapport.valeurs_nulles, 1);
        assert_eq!(rapport.textes_vides, 1);
        assert_eq!(rapport.textes_renseignes, 1);
        assert_eq!(rapport.autres_types, 1);
    }
}
