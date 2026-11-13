mod analyse;

use rustgis_plugin::{Kind, Request, Value, host, json};

fn handle(req: Request) -> Result<Value, String> {
    match (req.kind, req.id.as_str()) {
        (Kind::Command, "panel-state") => Ok(message_panneau("Prêt pour le développement.")),

        (Kind::Command, "test-panel") => {
            let valeurs = [
                Some(String::from("École")),
                None,
                Some(String::from("")),
                Some(String::from("    ")),
                Some(String::from(" Mairie ")),
            ];

            let rapport = analyse::analyser_textes(&valeurs);
            let message = formater_rapport(&rapport);

            host::notify(&message);

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

fn formater_rapport(rapport: &analyse::RapportTexte) -> String {
    let message = format!(
        "Analyse des données d'exemple\nTotal: {}\nValeurs nulles : {}\nTextes vides: {}\nTextes renseignés: {}",
        rapport.total, rapport.valeurs_nulles, rapport.textes_vides, rapport.textes_renseignes
    );

    message
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
}
