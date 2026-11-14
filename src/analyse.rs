use rustgis_plugin::Value;

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

pub(crate) fn convertir_valeur(valeur: Option<&Value>) -> ValeurAttribut {
    match valeur {
        None => ValeurAttribut::Absente,
        Some(valeur_json) => {
            if valeur_json.is_null() {
                ValeurAttribut::Nulle
            } else {
                match valeur_json.as_str() {
                    Some(texte) => ValeurAttribut::Text(String::from(texte)),
                    None => ValeurAttribut::Autre,
                }
            }
        }
    }
}

pub(crate) fn analyser_champ(proprietes: &[Value], nom_champ: &str) -> RapportAttribut {
    let mut attributs = Vec::new();

    for objet in proprietes {
        let valeur = objet.get(nom_champ);
        let attribut = convertir_valeur(valeur);
        attributs.push(attribut);
    }

    let resultat = analyser_attributs(&attributs);

    resultat
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgis_plugin::json;

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

    #[test]
    fn analyse_attribut_absent() {
        let resultat = convertir_valeur(None);

        assert_eq!(resultat, ValeurAttribut::Absente);
    }

    #[test]
    fn analyse_attribut_null_explicite() {
        let valeur = json!(null);
        let resultat = convertir_valeur(Some(&valeur));

        assert_eq!(resultat, ValeurAttribut::Nulle);
    }

    #[test]
    fn analyse_attribut_renseigner() {
        let valeur = json!("École");
        let resultat = convertir_valeur(Some(&valeur));

        assert_eq!(resultat, ValeurAttribut::Text(String::from("École")));
    }

    #[test]
    fn analyse_attribut_texte_vide() {
        let valeur = json!("");
        let resultat = convertir_valeur(Some(&valeur));

        assert_eq!(resultat, ValeurAttribut::Text(String::from("")));
    }

    #[test]
    fn analyse_attribut_entier() {
        let valeur = json!(42);
        let resultat = convertir_valeur(Some(&valeur));

        assert_eq!(resultat, ValeurAttribut::Autre);
    }

    #[test]
    fn analyse_attribut_boolean() {
        let valeur = json!(true);
        let resultat = convertir_valeur(Some(&valeur));

        assert_eq!(resultat, ValeurAttribut::Autre);
    }

    #[test]
    fn analyse_attribut_espace() {
        let valeur = json!("   ");
        let resultat = convertir_valeur(Some(&valeur));

        assert_eq!(resultat, ValeurAttribut::Text(String::from("   ")));
    }

    #[test]
    fn analyse_un_texte_json_compose_d_espaces() {
        let valeur_json = json!("   ");

        let attribut = convertir_valeur(Some(&valeur_json));

        let attributs = [attribut];

        let rapport = analyser_attributs(&attributs);

        assert_eq!(rapport.total, 1);
        assert_eq!(rapport.textes_vides, 1);
        assert_eq!(rapport.textes_renseignes, 0);
    }

    #[test]
    fn detecte_un_champ_absent_des_proprietes() {
        let proprietes = json!({
            "nom": "École",
            "description": null
        });

        let valeur = proprietes.get("adresse");

        let attribut = convertir_valeur(valeur);

        assert_eq!(attribut, ValeurAttribut::Absente);
    }

    #[test]
    fn detecte_un_champ_nul_des_proprietes() {
        let proprietes = json!({
            "nom": "École",
            "description": null
        });

        let valeur = proprietes.get("description");
        let attribut = convertir_valeur(valeur);

        assert_eq!(attribut, ValeurAttribut::Nulle);
    }

    #[test]
    fn detecte_un_champ_renseigne_des_proprietes() {
        let proprietes = json!({
            "nom": "École",
            "description": null
        });

        let valeur = proprietes.get("nom");
        let attribut = convertir_valeur(valeur);

        assert_eq!(attribut, ValeurAttribut::Text(String::from("École")));
    }

    #[test]
    fn analyse_champ() {
        let proprietes = [
            json!({"nom": "École"}),
            json!({"nom": null}),
            json!({}),
            json!({"nom": "   "}),
            json!({"nom": 42}),
        ];

        let rapport = analyser_champ(&proprietes, "nom");

        assert_eq!(rapport.total, 5);
        assert_eq!(rapport.textes_renseignes, 1);
        assert_eq!(rapport.textes_vides, 1);
        assert_eq!(rapport.valeurs_absentes, 1);
        assert_eq!(rapport.valeurs_nulles, 1);
    }
}
