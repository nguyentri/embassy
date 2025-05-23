use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    env,
};


fn main() {
    // ---------- Parse -----------------------------------------------------------------------
    let cfg_dir = match std::env::var_os("CFG_DIR") {
        Some(val) if !val.is_empty() => PathBuf::from(val),
        _ => {
            println!("cargo:info=CFG_DIR not set, using current directory");
            PathBuf::from(".")
        }
    };
    let generated_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let pins = parse_ra_cfg(&cfg_dir.join("ra_cfg.txt").to_string_lossy());

    // ---------- Generate --------------------------------------------------------------------
    let mut out =
        File::create(generated_dir.join("generated.rs")).expect("cannot create ./generated.rs – check path");

    write_preamble(&mut out);
    write_foreach_pin_macro(&mut out, &pins);
    write_peripherals_mod(&mut out, &pins);

    // ---------- Tell Cargo when to re-run ----------------------------------------------------
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}/ra_cfg.txt", cfg_dir.display());
    println!("cargo:rerun-if-changed={}/generated.rs", generated_dir.display());
}

/* ------------------------------------------------------------------------------------------ */
/* Parsing                                                                                    */
/* ------------------------------------------------------------------------------------------ */

/// Scan `ra_cfg.txt` and return a list of enabled GPIO pins:
/// (`ident`, `PORTx`, port_number, pin_number)
fn parse_ra_cfg(path: &str) -> Vec<(String, String, u8, u8)> {
    let file = File::open(path).expect("cannot open ra_cfg.txt");
    let reader = BufReader::new(file);

    let mut inside_pin_section = false;
    let mut seen: HashSet<(u8, u8)> = HashSet::new();
    let mut out: Vec<(String, String, u8, u8)> = Vec::new();

    for line in reader.lines().flatten() {
        let line = line.trim();

        if line.starts_with("Pin Configurations") {
            inside_pin_section = true;
            continue;
        }
        if !inside_pin_section {
            continue;
        }

        // Expect lines whose first token looks like "P000" … "P915"
        let mut toks = line.split_whitespace();
        let label = match toks.next() {
            Some(l) if l.starts_with('P') && l.len() == 4 => l,
            _ => continue,
        };

        // Skip disabled pins
        if line.contains("Disabled") {
            continue;
        }

        // Parse "Pxyz": x = port, yz = pin
        let port_num = label.as_bytes()[1] - b'0';         // single-digit port (0-9)
        let pin_num: u8 = label[2..4].parse().unwrap_or(0);

        if !seen.insert((port_num, pin_num)) {
            continue; // duplicate
        }

        let pin_ident  = format!("P{}_{}", port_num, pin_num); // e.g. P1_05
        let port_ident = format!("PORT{}", port_num);          // e.g. PORT1
        out.push((pin_ident, port_ident, port_num, pin_num));
    }

    // Guarantee at least one entry so the file is never empty.
    if out.is_empty() {
        out.push(("P0_0".into(), "PORT0".into(), 0, 0));
    }

    out
}

/* ------------------------------------------------------------------------------------------ */
/* Code generation helpers                                                                    */
/* ------------------------------------------------------------------------------------------ */

fn write_preamble(out: &mut File) {
    writeln!(
        out,
        "// -----------------------------------------------------------------------------\n\
         //  *** AUTO-GENERATED – DO NOT EDIT ***\n\
         //  Generated by build.rs – will be overwritten on every `cargo build`.\n\
         // -----------------------------------------------------------------------------\n"
    )
    .unwrap();
}

fn write_foreach_pin_macro(out: &mut File, pins: &[(String, String, u8, u8)]) {
    writeln!(out, "use embassy_hal_internal::impl_peripheral;\n").unwrap();
    writeln!(out, "#[doc(hidden)]").unwrap();
    writeln!(out, "#[macro_export]").unwrap();
    writeln!(out, "macro_rules! foreach_pin {{").unwrap();
    writeln!(
        out,
        "    ($macro:ident) => {{"
    )
    .unwrap();

    for (pin, port, pnum, pinum) in pins {
        writeln!(out, "        $macro!({pin}, {port}, {pnum}, {pinum});").unwrap();
    }

    writeln!(out, "    }};").unwrap(); // close arm
    writeln!(out, "}}").unwrap();      // close macro
}

fn write_peripherals_mod(out: &mut File, pins: &[(String, String, u8, u8)]) {
    writeln!(out, "\n/// Zero-sized marker types for every enabled pin").unwrap();
    writeln!(out, "pub mod peripherals {{").unwrap();
    writeln!(out, "    use super::*;").unwrap();
    writeln!(out).unwrap();

    for (pin, _, _, _) in pins {
        writeln!(
            out,
            "    #[derive(Debug)]\n    pub struct {pin} {{ pub _private: () }}\n    \
             impl_peripheral!({pin});",
        )
        .unwrap();
    }

    writeln!(out, "}}").unwrap(); // end module
}