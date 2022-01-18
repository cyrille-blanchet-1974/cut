use std::env;
use std::fs::File;

#[derive(Debug)]
pub struct Paramcli
//parameters from command line and/or confFile
{
    pub fic: String,     //file to work with if specified
    pub firstcar: usize, //for -c parm 1st char to get
    pub lastcar: usize,  //for -c parm last char to get
    pub field: usize,    //for -f parm  num of fileld
    pub delim: char,     //field delimiter
}

impl Default for Paramcli {
    fn default() -> Self {
        Paramcli::new()
    }
}

impl Paramcli {
    pub fn new() -> Paramcli {
        let mut fic = String::new();
        let mut first = 0;
        let mut last = 0;
        let mut delim = ' ';
        let mut field = 0;
        let mut charmode = false;
        let mut fieldmode = false;
        let mut delimmode = false;

        let args: Vec<String> = env::args().skip(1).collect();
        let name = env::args()
            .take(1)
            .next()
            .unwrap_or_else(|| String::from("cut"));
        if args.len() < 2 {
            println!("Error: not enough parameters");
            help(&name);
        }
        for arg in args {
            if arg == "/?"
                || arg == "-?"
                || arg.to_lowercase() == "/help"
                || arg.to_lowercase() == "-help"
            {
                help(&name);
            }
            if delimmode {
                delim = arg.chars().next().unwrap();
                delimmode = false;
                continue;
            }
            if charmode {
                //split arg
                //get 2 numbers
                let v: Vec<&str> = arg.split('-').collect();
                if v.len() != 2 {
                    println!("syntaxe error near -c");
                    help(&name);
                }
                let r: usize = match v.get(0).unwrap().parse() {
                    Err(e) => {
                        println!("erreur {}", e);
                        println!("syntaxe error near -c");
                        help(&name);
                        0
                    }
                    Ok(v) => v,
                };
                first = r;
                let r: usize = match v.get(1).unwrap().parse() {
                    Err(e) => {
                        println!("erreur {}", e);
                        println!("syntaxe error near -c");
                        help(&name);
                        0
                    }
                    Ok(v) => v,
                };
                last = r;
                charmode = false;
                if last < first {
                    println!("{} is lower than {}", last, first);
                    help(&name);
                }
                continue;
            }
            if fieldmode {
                let r: usize = match arg.parse() {
                    Err(e) => {
                        println!("erreur {}", e);
                        println!("syntaxe error near -f");
                        help(&name);
                        0
                    }
                    Ok(v) => v,
                };
                field = r - 1;
                fieldmode = false;
                continue;
            }
            if arg == "-c" {
                charmode = true;
                continue;
            }
            if arg == "-f" {
                fieldmode = true;
                continue;
            }
            if arg == "-d" {
                delimmode = true;
                continue;
            }
            fic = arg;
        }
        //checks
        if !fic.is_empty() {
            //check if file exists
            if File::open(&fic).is_err() {
                println!("Error file {} doesn't exists or unereadable", &fic);
                help(&name);
            };
        }
        Paramcli {
            fic,
            firstcar: first,
            lastcar: last,
            field,
            delim,
        }
    }
}

fn help(name: &str) {
    println!("{} 1.0 (2022)", name);
    println!("syntax : {} [file] [-c n-m] [-f o] [-d p]   ", name);
    println!("parameters between [] are optionnals");
    println!("------------------------------------");
    println!("fic: file to work with (if non use stdin");
    println!("-c n-m: get caractes n to m   (m must be lesser than m)");
    println!("-f o: get field in position o");
    println!("-d p: use p delimiter to identify fields");
    std::process::exit(0);
}
