use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use unicode_normalization::UnicodeNormalization;

fn to_bold(text: &str) -> String {
    text.nfd()
        .flat_map(|c| {
            let bold_char = match c {
                'a'..='z' => char::from_u32((c as u32 - 'a' as u32) + 0x1D41A).unwrap_or(c),
                'A'..='Z' => char::from_u32((c as u32 - 'A' as u32) + 0x1D400).unwrap_or(c),
                '0'..='9' => char::from_u32((c as u32 - '0' as u32) + 0x1D7CE).unwrap_or(c),
                _ => c,
            };
            bold_char.to_string().chars().collect::<Vec<_>>()
        })
        .collect()
}

fn to_italic(text: &str) -> String {
    text.nfd()
        .flat_map(|c| {
            let italic_char = match c {
                'a'..='z' => char::from_u32((c as u32 - 'a' as u32) + 0x1D44E).unwrap_or(c),
                'A'..='Z' => char::from_u32((c as u32 - 'A' as u32) + 0x1D434).unwrap_or(c),
                _ => c,
            };
            italic_char.to_string().chars().collect::<Vec<_>>()
        })
        .collect()
}

fn to_bold_italic(text: &str) -> String {
    text.nfd()
        .flat_map(|c| {
            let bold_italic_char = match c {
                'a'..='z' => char::from_u32((c as u32 - 'a' as u32) + 0x1D482).unwrap_or(c),
                'A'..='Z' => char::from_u32((c as u32 - 'A' as u32) + 0x1D468).unwrap_or(c),
                _ => c,
            };
            bold_italic_char.to_string().chars().collect::<Vec<_>>()
        })
        .collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_text(text: &str) -> String {
    let mut result = String::new();
    let mut buffer = String::new();
    let mut escape = false;
    let mut bold = false;
    let mut italic = false;
    //let mut style_stack = Vec::new(); // Pila para manejar estilos anidados
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if escape {
            buffer.push(c);
            escape = false;
        } else if c == '\\' {
            escape = true;
        } else if c == '*' {
            if chars.peek() == Some(&'*') {
                // Es negrita
                chars.next(); // Consume '*'
                if bold {
                    // Cierra el rango de negrita
                    bold = false;
                    if italic {
                        result.push_str(&to_bold_italic(&buffer));
                    } else {
                        result.push_str(&to_bold(&buffer));
                    }
                } else {
                    // Abre el rango de negrita
                    bold = true;
                    // Pasamos a italics si está activo para limpiar el buffer
                    if italic {
                        result.push_str(&to_italic(&buffer));
                    } else {
                        result.push_str(&buffer);
                    }
                }
            } else {
                // Es cursiva
                if italic {
                    // Cierra el rango de cursiva
                    italic = false;
                    if bold {
                        result.push_str(&to_bold_italic(&buffer));
                    } else {
                        result.push_str(&to_italic(&buffer));
                    }
                } else {
                    // Abre el rango de negrita
                    italic = true;
                    // Pasamos a italics si está activo para limpiar el buffer
                    if bold {
                        result.push_str(&to_bold(&buffer));
                    } else {
                        result.push_str(&buffer);
                    }
                }
            }
            buffer.clear();
        } else {
            // Agregar caracteres al buffer
            buffer.push(c);
        }
    }

    // Procesa el texto restante según el último estilo en la pila
    if !buffer.is_empty() {
        if bold && italic {
            result.push_str(&to_bold_italic(&buffer));
        } else if bold {
            result.push_str(&to_bold(&buffer));
        } else if italic {
            result.push_str(&to_italic(&buffer));
        } else {
            result.push_str(&buffer);
        }
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {:?} <file>", args[0]);
        return;
    }

    let filename = &args[1];

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(text) = line {
                let formatted_text = process_text(&text);
                println!("{}", formatted_text);
            }
        }
    } else {
        eprintln!("Could not find the given file {:?}", filename);
    }
}
