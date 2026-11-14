mod analyse;
use crate::analyse::ValeurAttribut;

use rustgis_plugin::{Kind, Request, Value, host, json};

fn handle(req: Request) -> Result<Value, String> {
    match (req.kind, req.id.as_str()) {
        (Kind::Command, "panel-state") => Ok(message_panneau("Prêt pour le développement.")),

        (Kind::Command, "test-panel") => {
            let couches = host::layers()?;

            let message = format!("Couches disponibles\n{:#?}", couches);

            Ok(message_panneau(&message))
        }

        (_, id) => Err(format!("Commande inconnue : {id}")),
    }
}

fn message_panneau(message: &str) -> Value {
    json!({
        "panel": {
            "items": {
                "status": {
                    "text": message
                }
            }
        }
    })
}

fn formater_rapport(rapport: &analyse::RapportAttribut) -> String {
    let message = format!(
        "Analyse des données d'exemple\nTotal : {}\nValeurs absentes : {}\nValeurs nulles : {}\nTextes vides : {}\nTextes renseignés : {}\nAutres types : {}",
        rapport.total,
        rapport.valeurs_absentes,
        rapport.valeurs_nulles,
        rapport.textes_vides,
        rapport.textes_renseignes,
        rapport.autres_types,
    );

    message
}

fn trouver_couche_active(couches: &Value) -> Result<&Value, String> {
    let tableau = match couches.as_array() {
        Some(tableau) => tableau,
        None => {
            return Err(format!("La réponse des couches doit être un tableau JSON."));
        }
    };

    for couche in tableau {
        if let Some(active) = couche.get("active") {
            if active.as_bool() == Some(true) {
                return Ok(couche);
            }
        }
    }

    Err(format!("Aucune couche active trouvée"))
}

rustgis_plugin::export!(handle);

// Hors wasm, le SDK importe `call`/`result`/`log` depuis l'hôte RustGIS, qui n'existent
// pas lors d'un build natif (`cargo test`) : on fournit des stubs pour que l'édition de liens passe.
#[cfg(not(target_arch = "wasm32"))]
mod host_stubs {
    #[unsafe(no_mangle)]
    extern "C" fn call(_ptr: *const u8, _len: usize) -> i32 {
        -1
    }

    #[unsafe(no_mangle)]
    extern "C" fn result(_ptr: *mut u8) {}

    #[unsafe(no_mangle)]
    extern "C" fn log(_ptr: *const u8, _len: usize) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construit_le_message_du_panneau() {
        let reponse = message_panneau("Test réussi");

        assert_eq!(
            reponse["panel"]["items"]["status"]["text"].as_str(),
            Some("Test réussi")
        );
    }

    #[test]
    fn construit_un_rapport_complet_pour_le_panneau() {
        let proprietes = [
            json!({"nom": "École"}),
            json!({"nom": null}),
            json!({}),
            json!({"nom": "   "}),
            json!({"nom": 42}),
        ];

        let rapport = analyse::analyser_champ(&proprietes, "nom")
            .expect("L'analyse devrait réussir pour ces objets d'attributs valides");
        let message = formater_rapport(&rapport);
        let reponse = message_panneau(&message);

        let attendu = "Analyse des données d'exemple\nTotal : 5\nValeurs absentes : 1\nValeurs nulles : 1\nTextes vides : 1\nTextes renseignés : 1\nAutres types : 1";

        assert_eq!(
            reponse["panel"]["items"]["status"]["text"].as_str(),
            Some(attendu)
        )
    }

    #[test]
    fn trouve_la_couche_active() {
        let couches = json!([
            {
                "id": 10,
                "name": "Première couche",
                "active": false,
            },
            {
                "id": 20,
                "name": "Deuxième couche",
                "active": true
            }
        ]);

        let couche_active = trouver_couche_active(&couches).expect("Une couche active devrait être trouvée.");

        assert_eq!(couche_active["id"], json!(20));
    }

    #[test]
    fn signale_absence_de_couche_active() {
        let couches = json!([
            {"id": 10, "active": false},
            {"id": 20, "active": false}
        ]);

        let erreur = trouver_couche_active(&couches).expect_err("Une couche active doit être trouvée.");

        assert_eq!(erreur, "Aucune couche active trouvée")
    }

    #[test]
    fn refuse_une_liste_de_couches_mal_formee() {
        let couches = json!({
            "id": 10,
            "active": true
        });

        let erreur = trouver_couche_active(&couches).expect_err("Une couche active doit être trouvée.");

        assert_eq!(erreur, "La réponse des couches doit être un tableau JSON.")
    }
}
