mod analyse;

use rustgis_plugin::{Kind, Request, Value, host, json};

fn handle(req: Request) -> Result<Value, String> {
    match (req.kind, req.id.as_str()) {
        (Kind::Command, "panel-state") => Ok(message_panneau("Prêt pour le développement.")),

        (Kind::Command, "test-panel") => {
            host::notify("La commande test-panel a bien été exécutée.");

            Ok(message_panneau(
                "Le panneau communique correctement avec Rust.",
            ))
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
