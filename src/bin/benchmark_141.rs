// Zamani Compiler — Automated Benchmark Suite for 141 Hardware Backends

use std::fs::File;
use std::io::Write;
use std::time::Instant;

// Import all backends from classic_backends
use zamani::compiler::classic_backends::*;

struct TargetBenchmarkResult {
    name: &'s str,
    era: &'s str,
    code_len: usize,
    line_count: usize,
    generation_time_us: u128,
    sample_code: String,
}

fn main() {
    println!("=== ZAMANI 141-TARGET AUTOMATED BENCHMARK SUITE ===");
    println!("Executing code generation benchmarks across all historical, mechanical, microcoded, and modern backends...\n");

    let mut results = Vec::new();

    macro_rules! bench_target {
        ($name:expr, $era:expr, $code:expr) => {
            let start = Instant::now();
            let generated = $code;
            let elapsed = start.elapsed().as_micros();
            let len = generated.len();
            let lines = generated.lines().count();
            results.push(TargetBenchmarkResult {
                name: $name,
                era: $era,
                code_len: len,
                line_count: lines,
                generation_time_us: elapsed,
                sample_code: generated,
            });
        };
    }

    // 1. Modern & General Purpose
    bench_target!("x86_64", "Modern (2020s)", X86_64Backend::emit_assembly("ZamaniTest"));
    bench_target!("ARM64", "Modern (2020s)", Arm64Backend::emit_assembly("ZamaniTest"));
    bench_target!("RISC-V", "Modern (2020s)", RiscvBackend::emit_assembly("ZamaniTest"));
    bench_target!("LoongArch", "Modern (2020s)", LoongArchBackend::emit_assembly("ZamaniTest"));
    bench_target!("WebAssembly", "Modern (2010s)", WasmBackend::emit_assembly("ZamaniTest"));

    // 2. Minicomputers & Mainframes
    bench_target!("VAX", "Minicomputer (1970s)", VaxBackend::emit_assembly("ZamaniTest"));
    bench_target!("PDP-11", "Minicomputer (1970s)", Pdp11Backend::emit_assembly("ZamaniTest"));
    bench_target!("SPARC", "Workstation (1980s)", SparcBackend::emit_assembly("ZamaniTest"));
    bench_target!("S390x", "Mainframe (Modern)", S390xBackend::emit_assembly("ZamaniTest"));
    bench_target!("Alpha", "Workstation (1990s)", AlphaBackend::emit_assembly("ZamaniTest"));
    bench_target!("IA-64", "Workstation (2000s)", Ia64Backend::emit_assembly("ZamaniTest"));
    bench_target!("PA-RISC", "Workstation (1980s)", PaRiscBackend::emit_assembly("ZamaniTest"));
    bench_target!("IBM System/360", "Mainframe (1960s)", System360Backend::emit_assembly("ZamaniTest"));
    bench_target!("IBM 7030 (Stretch)", "Supercomputer (1960s)", Ibm7030Backend::emit_assembly("ZamaniTest"));
    bench_target!("CDC 1604", "Scientific (1960s)", Cdc1604Backend::emit_assembly("ZamaniTest"));
    bench_target!("Burroughs B5000", "Mainframe (1960s)", BurroughsB5000Backend::emit_assembly("ZamaniTest"));
    bench_target!("PDP-8", "Minicomputer (1960s)", Pdp8Backend::emit_assembly("ZamaniTest"));
    bench_target!("PDP-10", "Mainframe (1960s)", Pdp10Backend::emit_assembly("ZamaniTest"));

    // 3. Embedded, RISC & DSP
    bench_target!("MIPS", "Embedded (1980s)", MipsBackend::emit_assembly("ZamaniTest"));
    bench_target!("PowerPC", "RISC (1990s)", PowerPcBackend::emit_assembly("ZamaniTest"));
    bench_target!("Motorola 68k", "Workstation (1980s)", M68kBackend::emit_assembly("ZamaniTest"));
    bench_target!("SuperH", "Embedded (1990s)", SuperHBackend::emit_assembly("ZamaniTest"));
    bench_target!("Xtensa", "IoT (2000s)", XtensaBackend::emit_assembly("ZamaniTest"));
    bench_target!("ARC", "Configurable RISC", ArcBackend::emit_assembly("ZamaniTest"));
    bench_target!("Blackfin", "DSP/MCU (2000s)", BlackfinBackend::emit_assembly("ZamaniTest"));
    bench_target!("Hexagon", "VLIW DSP", HexagonBackend::emit_assembly("ZamaniTest"));

    // 4. Microcontrollers & 8/16-bit
    bench_target!("AVR", "Microcontroller", AvrBackend::emit_assembly("ZamaniTest"));
    bench_target!("MSP430", "Ultra-Low-Power MCU", Msp430Backend::emit_assembly("ZamaniTest"));
    bench_target!("Intel 8051", "Microcontroller", Intel8051Backend::emit_assembly("ZamaniTest"));
    bench_target!("Microchip PIC", "Microcontroller", PicBackend::emit_assembly("ZamaniTest"));
    bench_target!("Zilog Z80", "8-bit Classic", Z80Backend::emit_assembly("ZamaniTest"));
    bench_target!("MOS 6502", "8-bit Classic", Mos6502Backend::emit_assembly("ZamaniTest"));
    bench_target!("Intel 8086", "16-bit Foundation", Intel8086Backend::emit_assembly("ZamaniTest"));
    bench_target!("Intel 8080", "8-bit CP/M", Intel8080Backend::emit_assembly("ZamaniTest"));
    bench_target!("Motorola 68HC11", "Microcontroller", M68HC11Backend::emit_assembly("ZamaniTest"));
    bench_target!("Renesas H8/300", "Embedded MCU", H8300Backend::emit_assembly("ZamaniTest"));

    // 5. Foundation & Historical (1941 - 1988)
    bench_target!("Konrad Zuse Z3", "Pioneering (1941)", Z3Backend::emit_assembly("ZamaniTest"));
    bench_target!("Atanasoff-Berry (ABC)", "Pioneering (1942)", AbcBackend::emit_assembly("ZamaniTest"));
    bench_target!("Colossus", "Pioneering (1943)", ColossusBackend::emit_assembly("ZamaniTest"));
    bench_target!("Harvard Mark I", "Pioneering (1944)", Mark1Backend::emit_assembly("ZamaniTest"));
    bench_target!("Manchester Baby", "Stored-Program (1948)", ManchesterBabyBackend::emit_assembly("ZamaniTest"));
    bench_target!("BINAC", "Pioneering (1949)", BinacBackend::emit_assembly("ZamaniTest"));
    bench_target!("CSIRAC", "Pioneering (1949)", CsiracBackend::emit_assembly("ZamaniTest"));
    bench_target!("SWAC", "Pioneering (1950)", SwacBackend::emit_assembly("ZamaniTest"));
    bench_target!("SEAC", "Pioneering (1950)", SeacBackend::emit_assembly("ZamaniTest"));
    bench_target!("LEO I", "Commercial (1951)", Leo1Backend::emit_assembly("ZamaniTest"));
    bench_target!("IAS Machine", "Stored-Program (1952)", IasBackend::emit_assembly("ZamaniTest"));
    bench_target!("ILLIAC I", "Scientific (1952)", Illiac1Backend::emit_assembly("ZamaniTest"));
    bench_target!("BESK", "Scientific (1953)", BeskBackend::emit_assembly("ZamaniTest"));
    bench_target!("IBM 704", "Scientific (1954)", Ibm704Backend::emit_assembly("ZamaniTest"));
    bench_target!("IBM 709", "Mainframe (1958)", Ibm709Backend::emit_assembly("ZamaniTest"));
    bench_target!("IBM 7090", "Transistor Mainframe (1959)", Ibm7090Backend::emit_assembly("ZamaniTest"));
    bench_target!("Manchester Atlas", "Supercomputer (1962)", AtlasBackend::emit_assembly("ZamaniTest"));
    bench_target!("GE-600 (Multics)", "Time-Sharing (1964)", Ge600Backend::emit_assembly("ZamaniTest"));
    bench_target!("SDS 940", "Time-Sharing (1966)", Sds940Backend::emit_assembly("ZamaniTest"));
    bench_target!("Honeywell 316", "Minicomputer (1969)", Honeywell316Backend::emit_assembly("ZamaniTest"));
    bench_target!("Intel 4004", "Microprocessor (1971)", Intel4004Backend::emit_assembly("ZamaniTest"));
    bench_target!("Xerox Alto", "Workstation (1973)", XeroxAltoBackend::emit_assembly("ZamaniTest"));
    bench_target!("Altair 8800", "Microcomputer (1974)", Altair8800Backend::emit_assembly("ZamaniTest"));
    bench_target!("Cray-1", "Vector Supercomputer (1975)", Cray1Backend::emit_assembly("ZamaniTest"));
    bench_target!("Apple I", "Home Computer (1976)", Apple1Backend::emit_assembly("ZamaniTest"));
    bench_target!("Commodore PET", "Home Computer (1977)", CommodorePetBackend::emit_assembly("ZamaniTest"));
    bench_target!("TRS-80", "Home Computer (1977)", Trs80Backend::emit_assembly("ZamaniTest"));
    bench_target!("Apple II", "Home Computer (1977)", Apple2Backend::emit_assembly("ZamaniTest"));
    bench_target!("BBC Micro", "Home Computer (1981)", BbcMicroBackend::emit_assembly("ZamaniTest"));
    bench_target!("IBM PC 5150", "Personal Computer (1981)", IbmPcBackend::emit_assembly("ZamaniTest"));
    bench_target!("ZX Spectrum", "Home Computer (1982)", ZxSpectrumBackend::emit_assembly("ZamaniTest"));

    // 6. Home Era & Ancestral Microcode/Bitstream
    bench_target!("Commodore 64", "Home Computer (1982)", Commodore64Backend::emit_assembly("ZamaniTest"));
    bench_target!("Apple Macintosh", "Workstation (1984)", MacintoshBackend::emit_assembly("ZamaniTest"));
    bench_target!("Amiga 1000", "Multimedia (1985)", Amiga1000Backend::emit_assembly("ZamaniTest"));
    bench_target!("Atari ST", "Workstation (1985)", AtariStBackend::emit_assembly("ZamaniTest"));
    bench_target!("NeXT Computer", "Workstation (1988)", NextComputerBackend::emit_assembly("ZamaniTest"));
    bench_target!("Whirlwind I", "Pioneering Microcode (1951)", WhirlwindBackend::emit_assembly("ZamaniTest"));
    bench_target!("Pilot ACE", "Pioneering (1950)", PilotAceBackend::emit_assembly("ZamaniTest"));
    bench_target!("Konrad Zuse Z4", "Pioneering (1945)", Z4Backend::emit_assembly("ZamaniTest"));
    bench_target!("EDVAC", "Stored-Program (1949)", EdvacBackend::emit_assembly("ZamaniTest"));
    bench_target!("Manchester Mark 1", "Stored-Program (1949)", ManchesterMark1Backend::emit_assembly("ZamaniTest"));
    bench_target!("Ferranti Mark 1", "Commercial (1951)", FerrantiMark1Backend::emit_assembly("ZamaniTest"));
    bench_target!("UNIVAC 1103", "Scientific (1953)", Univac1103Backend::emit_assembly("ZamaniTest"));
    bench_target!("AGC", "Spacecraft Microcode (1966)", AgcBackend::emit_assembly("ZamaniTest"));
    bench_target!("AMD 2901", "Bit-Slice Microcode (1975)", Amd2901Backend::emit_assembly("ZamaniTest"));
    bench_target!("Symbolics Lisp Machine", "Tagged Microcode (1981)", SymbolicsLispBackend::emit_assembly("ZamaniTest"));
    bench_target!("CM-2", "SIMD Bitstream (1987)", ConnectionMachine2Backend::emit_assembly("ZamaniTest"));
    bench_target!("Intel 8008", "Microprocessor (1972)", Intel8008Backend::emit_assembly("ZamaniTest"));
    bench_target!("Motorola 6809", "8-bit Advanced (1978)", Motorola6809Backend::emit_assembly("ZamaniTest"));
    bench_target!("RCA 1802", "Spacecraft CMOS (1976)", Rca1802Backend::emit_assembly("ZamaniTest"));
    bench_target!("Intel iAPX 432", "Object-Based (1981)", Iapx432Backend::emit_assembly("ZamaniTest"));
    bench_target!("Inmos Transputer", "Parallel (1983)", InmosTransputerBackend::emit_assembly("ZamaniTest"));
    bench_target!("Cray X-MP", "Vector Supercomputer (1982)", CrayXmpBackend::emit_assembly("ZamaniTest"));
    bench_target!("Xerox Star 8010", "GUI Workstation (1981)", XeroxStarBackend::emit_assembly("ZamaniTest"));
    bench_target!("Data General Nova", "Minicomputer (1969)", DataGeneralNovaBackend::emit_assembly("ZamaniTest"));
    bench_target!("TI TMS9900", "16-bit MCU (1976)", Tms9900Backend::emit_assembly("ZamaniTest"));
    bench_target!("Motorola 6800", "8-bit MCU (1974)", Motorola6800Backend::emit_assembly("ZamaniTest"));
    bench_target!("Motorola 68020", "32-bit CPU (1984)", Motorola68020Backend::emit_assembly("ZamaniTest"));
    bench_target!("Signetics 2650", "Arcade MCU (1975)", Signetics2650Backend::emit_assembly("ZamaniTest"));
    bench_target!("NatSem SC/MP", "Simple MCU (1976)", ScmpBackend::emit_assembly("ZamaniTest"));
    bench_target!("Fairchild F8", "Multi-chip MCU (1975)", FairchildF8Backend::emit_assembly("ZamaniTest"));
    bench_target!("Intel 8048", "Embedded MCU (1976)", Intel8048Backend::emit_assembly("ZamaniTest"));
    bench_target!("Acorn ARM1", "RISC Pioneer (1985)", Arm1Backend::emit_assembly("ZamaniTest"));
    bench_target!("Symbolics 3600", "Lisp Microcode (1983)", Symbolics3600Backend::emit_assembly("ZamaniTest"));

    // 7. Comprehensive Legacy Expansion
    bench_target!("Analytical Engine", "Mechanical (1837)", AnalyticalEngineBackend::emit_assembly("ZamaniTest"));
    bench_target!("Difference Engine", "Mechanical (1849)", DifferenceEngineBackend::emit_assembly("ZamaniTest"));
    bench_target!("Konrad Zuse Z1", "Mechanical (1938)", Z1Backend::emit_assembly("ZamaniTest"));
    bench_target!("Konrad Zuse Z2", "Hybrid Relay (1940)", Z2Backend::emit_assembly("ZamaniTest"));
    bench_target!("Differential Analyzer", "Analog (1931)", DifferentialAnalyzerBackend::emit_assembly("ZamaniTest"));
    bench_target!("WITCH", "Dekatron Decimal (1951)", WitchBackend::emit_assembly("ZamaniTest"));
    bench_target!("ORDVAC", "Scientific (1951)", OrdvacBackend::emit_assembly("ZamaniTest"));
    bench_target!("JOHNNIAC", "IAS Architecture (1953)", JohnniacBackend::emit_assembly("ZamaniTest"));
    bench_target!("MANIAC I", "Scientific (1952)", Maniac1Backend::emit_assembly("ZamaniTest"));
    bench_target!("SILLIAC", "University (1956)", SilliacBackend::emit_assembly("ZamaniTest"));
    bench_target!("WEIZAC", "Institute (1955)", WeizacBackend::emit_assembly("ZamaniTest"));
    bench_target!("DASK", "Educational (1958)", DaskBackend::emit_assembly("ZamaniTest"));
    bench_target!("PERM", "Drum/Core (1956)", PermBackend::emit_assembly("ZamaniTest"));
    bench_target!("DEC PDP-1", "Minicomputer (1959)", Pdp1Backend::emit_assembly("ZamaniTest"));
    bench_target!("DEC PDP-7", "Minicomputer (1964)", Pdp7Backend::emit_assembly("ZamaniTest"));
    bench_target!("IBM 1620", "Decimal Scientific (1959)", Ibm1620Backend::emit_assembly("ZamaniTest"));
    bench_target!("IBM 1130", "Scientific (1965)", Ibm1130Backend::emit_assembly("ZamaniTest"));
    bench_target!("CDC 3600", "Scientific (1963)", Cdc3600Backend::emit_assembly("ZamaniTest"));
    bench_target!("HP 2100", "Minicomputer (1970)", Hp2100Backend::emit_assembly("ZamaniTest"));
    bench_target!("HP 3000", "Stack Minicomputer (1972)", Hp3000Backend::emit_assembly("ZamaniTest"));
    bench_target!("Data General Eclipse", "Minicomputer (1974)", DataGeneralEclipseBackend::emit_assembly("ZamaniTest"));
    bench_target!("IMSAI 8080", "Microcomputer (1975)", Imsai8080Backend::emit_assembly("ZamaniTest"));
    bench_target!("MOS KIM-1", "Single-Board (1976)", Kim1Backend::emit_assembly("ZamaniTest"));
    bench_target!("Sinclair ZX81", "Home Computer (1981)", Zx81Backend::emit_assembly("ZamaniTest"));
    bench_target!("Amstrad CPC 464", "Home Computer (1984)", AmstradCpcBackend::emit_assembly("ZamaniTest"));
    bench_target!("MSX Standard", "Home Standard (1983)", MsxBackend::emit_assembly("ZamaniTest"));
    bench_target!("Sharp X68000", "Workstation (1987)", SharpX68000Backend::emit_assembly("ZamaniTest"));
    bench_target!("NEC PC-9801", "Business PC (1982)", NecPc98Backend::emit_assembly("ZamaniTest"));
    bench_target!("Commodore VIC-20", "Home Computer (1980)", Vic20Backend::emit_assembly("ZamaniTest"));
    bench_target!("Commodore 128", "Home Computer (1985)", Commodore128Backend::emit_assembly("ZamaniTest"));
    bench_target!("Atari 400/800", "Home Computer (1979)", Atari8BitBackend::emit_assembly("ZamaniTest"));
    bench_target!("TI-99/4A", "16-bit Home (1979)", Ti994ABackend::emit_assembly("ZamaniTest"));
    bench_target!("Dragon 32", "Home Computer (1982)", Dragon32Backend::emit_assembly("ZamaniTest"));
    bench_target!("Nintendo NES", "Console (1983)", NesBackend::emit_assembly("ZamaniTest"));
    bench_target!("Super Nintendo (SNES)", "Console (1990)", SnesBackend::emit_assembly("ZamaniTest"));
    bench_target!("Sega Genesis", "Console (1988)", SegaGenesisBackend::emit_assembly("ZamaniTest"));
    bench_target!("Nintendo Game Boy", "Handheld (1989)", GameBoyBackend::emit_assembly("ZamaniTest"));
    bench_target!("Game Boy Advance", "Handheld (2001)", GbaBackend::emit_assembly("ZamaniTest"));
    bench_target!("Atari 2600", "Console (1977)", Atari2600Backend::emit_assembly("ZamaniTest"));
    bench_target!("HP-41C", "Calculator (1979)", Hp41cBackend::emit_assembly("ZamaniTest"));
    bench_target!("TI-83", "Graphing Calculator (1996)", Ti83Backend::emit_assembly("ZamaniTest"));

    assert_eq!(results.len(), 141, "Expected exactly 141 benchmarked targets!");

    // Generate Markdown Report
    let mut report = String::new();
    report.push_str("# Zamani Compiler — 141-Target Benchmark Report\n\n");
    report.push_str("This report presents automated code generation metrics across all **141 historical, classical, microcoded, and modern architectures** supported by the Zamani compiler.\n\n");
    report.push_str("| Target Architecture | Technological Era | Code Length (Bytes) | Line Count | Gen Time (us) |\n");
    report.push_str("| :--- | :--- | :---: | :---: | :---: |\n");

    for r in &results {
        report.push_str(&format!(
            "| **{}** | {} | {} | {} | {} |\n",
            r.name, r.era, r.code_len, r.line_count, r.generation_time_us
        ));
    }

    let report_path = "/home/ubuntu/Zamani/BENCHMARK_141_REPORT.md";
    let mut file = File::create(report_path).unwrap();
    file.write_all(report.as_bytes()).unwrap();

    println!("Successfully benchmarked all 141 targets!");
    println!("Report written to {}", report_path);
}
