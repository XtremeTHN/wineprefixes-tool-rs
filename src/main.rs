use glob::glob;
use std::env;
use std::fs;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Wineprefixes Tool",
    about = "Herramienta para leer/manipular prefijos de wine"
)]

struct Opt {
    #[structopt(
        short = "g",
        long = "get",
        help = "Imprime el prefijo especificado solo si se encuentra"
    )]
    get_opt: Option<String>,

    #[structopt(
        short = "r",
        long = "remove",
        help = "Elimina el prefijo especificado solo si se encuentra"
    )]
    remove_opt: Option<String>,

    #[structopt(short = "c", long = "create", help = "Crea un prefijo especificado")]
    create_opt: Option<String>,

    #[structopt(
        short = "ls",
        long = "list",
        help = "Lista los prefijos que se encuentren"
    )]
    list_opt: bool,
}
fn main() {
    // Obtaining HOME path...
    let home_path = env::var("HOME").unwrap_or_else(|_| {
        eprintln!("Error critico: No se pudo leer la variable de entorno 'HOME'");
        process::exit(1);
    });
    let complete_path = format!("{}/.local/share/wineprefixes/*", home_path);

    // Parsing arguments...
    let opt = Opt::from_args();

    let get_opt = opt.get_opt.unwrap_or_default();
    let remove_opt = opt.remove_opt.unwrap_or_default();
    let create_opt = opt.create_opt.unwrap_or_default();
    // Founding prefixes...
    if get_opt != "" {
        for entry in glob(&complete_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    if path.is_dir() && path.ends_with(&get_opt) {
                        println!("{}", path.display());
                    }
                }
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    }

    if remove_opt != "" {
        let path = format!("{}/.local/share/wineprefixes/{}", home_path, remove_opt);
        if fs::metadata(&path).is_ok() {
            fs::remove_dir_all(&path).unwrap();
            println!("Eliminado correctamente");
        } else {
            println!("El prefijo {} no existe", remove_opt);
        }
    }

    if create_opt != "" {
        let path = format!("{}/.local/share/wineprefixes/{}", home_path, create_opt);
        fs::create_dir_all(&path);
        env::set_var("WINEPREFIX", &path);
        let output = process::Command::new("wineboot")
                .arg("-u")
                .output()
                .expect(&format!("Error al crear el prefijo en la carpeta {}", path));
        println!("{}", path);
        let stdout = String::from_utf8(output.stdout).unwrap();
    }

    if opt.list_opt {
        let mut prefixes = Vec::new();
        for entry in glob(&complete_path).expect("Error al buscar los prefijos") {
            match entry {
                Ok(path) => {
                    if path.is_dir() {
                        prefixes.push(path.to_string_lossy().into_owned());
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
        if prefixes.is_empty() {
            println!("No se encontraron prefijos")
        } else {
            println!("Prefijos encontrados: {}", prefixes.join(", "));
        }
    }
}
