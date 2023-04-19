
use std::fs;
use std::fs::File;
use std::path::Path;
use regex::Regex;
use std::io::Write;

#[derive(Debug)]
struct FileFunc {
    def: Vec<String>,
    filename: String
}

fn main() {
    // Chemin vers le répertoire à parcourir
    let repo_path = "./../ts";

    // Regex pour matcher les noms d'interfaces, d'énumérations et de types
    let interface_regex = Regex::new(r"\binterface\s+(\w+)\b").unwrap();
    let enum_regex = Regex::new(r"\benum\s+(\w+)\b").unwrap();
    let type_regex = Regex::new(r"\btype\s+(\w+)\b").unwrap();
    let mut vec_def:Vec<FileFunc> = Vec::new();
    // Parcours récursif des fichiers .ts du répertoire
    let entries = fs::read_dir(repo_path).expect("Erreur lors de la lecture du répertoire");
    for entry in entries {
        let mut names:Vec<String> = Vec::new();
        if let Ok(entry) = entry {
            let file_path = entry.path();
            if file_path.is_file() && file_path.extension().map(|ext| ext == "ts").unwrap_or(false) {
                let file_content = fs::read_to_string(&file_path).expect("Erreur lors de la lecture du fichier");

                // Recherche des noms d'interfaces, d'énumérations et de types dans le contenu du fichier
                for captures in interface_regex.captures_iter(&file_content) {
                    if let Some(name) = captures.get(1) {
                        names.push(name.as_str().to_string());
                    }
                }
                for captures in enum_regex.captures_iter(&file_content) {
                    if let Some(name) = captures.get(1) {
                        names.push(name.as_str().to_string());
                    }
                }
                for captures in type_regex.captures_iter(&file_content) {
                    if let Some(name) = captures.get(1) {
                        names.push(name.as_str().to_string());
                    }
                    vec_def.push(FileFunc{ def: names.to_vec(), filename: file_path.as_path().to_str().unwrap().to_string() });
                }

            }

        }
    }
    let output_file_path = "./../ts/index.ts";
    let mut output_file = File::create(output_file_path).expect("Erreur lors de la création du fichier de sortie");
    for def in &vec_def {
        let format = format!("export {{{}}} from '{}'", def.def.join(","), "./output");
        writeln!(&mut output_file, "{}", format).expect("Erreur lors de l'écriture dans le fichier");
    }

    println!("Les noms ont été écrits dans le fichier : {}", output_file_path);
}


