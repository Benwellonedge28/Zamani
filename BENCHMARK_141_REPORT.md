# Zamani Compiler — Comprehensive 141-Target Architecture Benchmark Report

This report provides automated code generation benchmarks across all **141 historical, mechanical, microcoded, console, and modern architectures** supported by the Zamani compiler ecosystem.

## Executive Summary

The Zamani Universal Trinity compiler achieves unprecedented architectural breadth, translating high-level omniversal logic into native representations ranging from Charles Babbage's 1837 mechanical barrels to modern AVX-512 vector pipelines and RISC-V extensions.

| # | Target Architecture | Era / Category | Description | Code Footprint (Bytes) | Lines | Gen Time (us) |
|:---|:---|:---|:---|:---:|:---:|:---:|
| 1 | **Abc** | Pioneering (1942) | Atanasoff-Berry Computer Regenerative Memory | 1249 | 18 | 63 |
| 2 | **Agc** | Spacecraft Microcode (1966) | Apollo Guidance Computer Core-Rope ROM Microcode | 1375 | 18 | 63 |
| 3 | **Alpha** | Workstation (1990s) | DEC Alpha 64-bit Workstation Assembly | 1323 | 18 | 12 |
| 4 | **Altair8800** | Microcomputer (1974) | MITS Altair 8800 Microcomputer Revolution | 1241 | 18 | 12 |
| 5 | **Amd2901** | Bit-Slice Microcode (1975) | AMD 2901 Bit-Slice ALU Microcode Control Store Word | 1439 | 18 | 46 |
| 6 | **Amiga1000** | Multimedia (1985) | Commodore Amiga 1000 Custom Chipset Multimedia | 1189 | 18 | 80 |
| 7 | **Amstrad Cpc** | Home Computer (1984) | Amstrad CPC 464 European Z80 Home Computer | 1133 | 18 | 29 |
| 8 | **Analytical Engine** | Mechanical (1837) | Charles Babbage Analytical Engine Barrel Program | 1375 | 18 | 46 |
| 9 | **Apple1** | Home Computer (1976) | Apple I Wozniak Single-Board Computer | 1123 | 18 | 29 |
| 10 | **Apple2** | Home Computer (1977) | Apple II Color Graphics Microcomputer | 1165 | 18 | 29 |
| 11 | **Arc** | Configurable RISC | Synopsys Configurable RISC Assembly | 1163 | 18 | 63 |
| 12 | **Arm1** | RISC Pioneer (1985) | Acorn ARM1 Original 32-bit RISC Architecture | 1121 | 18 | 80 |
| 13 | **Arm64** | Modern (2020s) | AArch64 Neon/SVE Assembly | 1375 | 18 | 12 |
| 14 | **Atari2600** | Console (1977) | Atari 2600 Video Computer System (VCS) | 1201 | 18 | 80 |
| 15 | **Atari8Bit** | Home Computer (1979) | Atari 400/800 ANTIC/GTIA Coprocessor Computer | 1213 | 18 | 80 |
| 16 | **Atari St** | Workstation (1985) | Atari ST GEMDOS Desktop Computer | 1179 | 18 | 63 |
| 17 | **Atlas** | Supercomputer (1962) | Manchester Atlas Virtual Memory Supercomputer | 1259 | 18 | 12 |
| 18 | **Avr** | Microcontroller | AVR 8-bit Microcontroller Assembly | 1205 | 18 | 63 |
| 19 | **Bbc Micro** | Home Computer (1981) | Acorn BBC Micro Educational Standard | 1119 | 18 | 80 |
| 20 | **Besk** | Scientific (1953) | BESK Swedish Vacuum Tube Computer | 1063 | 18 | 80 |
| 21 | **Binac** | Pioneering (1949) | BINAC Dual Delay Line Processor | 1149 | 18 | 12 |
| 22 | **Blackfin** | DSP/MCU (2000s) | Analog Devices Blackfin DSP/MCU Hybrid | 1221 | 18 | 63 |
| 23 | **Burroughs B5000** | Mainframe (1960s) | Burroughs B5000 Stack-Based Descriptor Architecture | 1275 | 18 | 12 |
| 24 | **C128** | Home Computer (1985) | Commodore 128 Dual-CPU 8-bit System (8502/Z80) | 1189 | 18 | 80 |
| 25 | **C64** | General Architecture | Architecture target for c64 | 1187 | 18 | 63 |
| 26 | **Cdc1604** | Scientific (1960s) | CDC 1604 48-bit Transistorized Scientific Computer | 1153 | 18 | 46 |
| 27 | **Cdc3600** | Scientific (1963) | CDC 3600 48-bit Large-Scale Scientific Computer | 1069 | 18 | 46 |
| 28 | **Cdc6600** | General Architecture | Architecture target for cdc6600 | 1119 | 18 | 46 |
| 29 | **Cm2** | SIMD Bitstream (1987) | Thinking Machines CM-2 64K-Processor SIMD Bitstream | 1323 | 18 | 63 |
| 30 | **Colossus** | Pioneering (1943) | Colossus Optoelectronic Cryptanalysis | 1255 | 18 | 63 |
| 31 | **Commodore Pet** | Home Computer (1977) | Commodore PET Trinity Home Computer | 1155 | 18 | 63 |
| 32 | **Cray1** | Vector Supercomputer (1975) | Cray-1 Vector Supercomputer | 1063 | 18 | 12 |
| 33 | **Cray Xmp** | Vector Supercomputer (1982) | Cray X-MP Multiprocessor Vector Supercomputer | 1147 | 18 | 63 |
| 34 | **Csirac** | Pioneering (1949) | CSIRAC Australian Digital Computer | 1137 | 18 | 29 |
| 35 | **Dask** | Educational (1958) | DASK Danish Educational & Scientific Computer | 1043 | 18 | 80 |
| 36 | **Dg Eclipse** | Minicomputer (1974) | Data General Eclipse 16-bit Minicomputer | 1189 | 18 | 12 |
| 37 | **Difference Engine** | Mechanical (1849) | Charles Babbage Difference Engine Sector Setup | 1401 | 18 | 46 |
| 38 | **Differential Analyzer** | Analog (1931) | Vannevar Bush Differential Analyzer Gear Train | 1449 | 18 | 29 |
| 39 | **Dragon32** | Home Computer (1982) | Dragon 32 Motorola 6809 British Home Computer | 1145 | 18 | 63 |
| 40 | **Edsac** | General Architecture | Architecture target for edsac | 1149 | 18 | 12 |
| 41 | **Edvac** | Stored-Program (1949) | EDVAC Mercury Delay Line Binary Stored Program | 1171 | 18 | 12 |
| 42 | **Eniac** | General Architecture | Architecture target for eniac | 1279 | 18 | 12 |
| 43 | **Fairchild F8** | Multi-chip MCU (1975) | Fairchild F8 Multi-Chip Microprocessor | 1099 | 18 | 46 |
| 44 | **Ferranti Mark1** | Commercial (1951) | Ferranti Mark 1 First Commercial Electronic Computer | 1289 | 18 | 80 |
| 45 | **Gameboy** | Handheld (1989) | Nintendo Game Boy Handheld Console | 1171 | 18 | 46 |
| 46 | **Gba** | Handheld (2001) | Game Boy Advance ARM7TDMI Handheld | 1217 | 18 | 63 |
| 47 | **Ge600** | Time-Sharing (1964) | GE-600 Multics Time-Sharing System | 1097 | 18 | 12 |
| 48 | **H316** | Minicomputer (1969) | Honeywell 316 Kitchen Computer | 1103 | 18 | 80 |
| 49 | **H8300** | Embedded MCU | Renesas H8/300 Embedded Controller Assembly | 1191 | 18 | 12 |
| 50 | **Hexagon** | VLIW DSP | Qualcomm Hexagon VLIW DSP Architecture | 1277 | 18 | 46 |
| 51 | **Hp2100** | Minicomputer (1970) | HP 2100 16-bit Minicomputer for Automation | 1153 | 18 | 29 |
| 52 | **Hp3000** | Stack Minicomputer (1972) | HP 3000 Stack-Oriented Commercial Minicomputer | 1165 | 18 | 29 |
| 53 | **Hp41C** | Calculator (1979) | HP-41C Nut Processor Handheld Calculator | 1213 | 18 | 12 |
| 54 | **I4004** | Microprocessor (1971) | Intel 4004 First Commercial Microprocessor | 1131 | 18 | 12 |
| 55 | **I8008** | Microprocessor (1972) | Intel 8008 Early 8-bit Microprocessor | 1093 | 18 | 12 |
| 56 | **I8048** | Embedded MCU (1976) | Intel 8048 Harvard Architecture Embedded MCU | 1237 | 18 | 12 |
| 57 | **I8051** | Microcontroller | Intel MCS-51 8-bit Microcontroller Assembly | 1191 | 18 | 12 |
| 58 | **I8080** | 8-bit CP/M | Intel 8-bit CP/M Assembly | 1207 | 18 | 12 |
| 59 | **I8086** | 16-bit Foundation | Intel 16-bit x86 Foundation Assembly | 1409 | 18 | 12 |
| 60 | **Ia64** | Workstation (2000s) | Itanium EPIC Explicit Parallel Assembly | 1161 | 18 | 80 |
| 61 | **Iapx432** | Object-Based (1981) | Intel iAPX 432 Object-Based 32-bit Architecture | 1183 | 18 | 46 |
| 62 | **Ias** | Stored-Program (1952) | IAS Machine Von Neumann Prototype | 1157 | 18 | 63 |
| 63 | **Ibm1130** | Scientific (1965) | IBM 1130 Low-Cost 16-bit Scientific Computer | 1063 | 18 | 46 |
| 64 | **Ibm1401** | General Architecture | Architecture target for ibm1401 | 1115 | 18 | 46 |
| 65 | **Ibm1620** | Decimal Scientific (1959) | IBM 1620 Decimal Scientific Computer ('Cadet') | 1173 | 18 | 46 |
| 66 | **Ibm650** | General Architecture | Architecture target for ibm650 | 1283 | 18 | 29 |
| 67 | **Ibm701** | General Architecture | Architecture target for ibm701 | 1147 | 18 | 29 |
| 68 | **Ibm7030** | Supercomputer (1960s) | IBM 7030 Stretch Pipelined Supercomputer | 1253 | 18 | 46 |
| 69 | **Ibm704** | Scientific (1954) | IBM 704 Floating-Point Hardware | 1101 | 18 | 29 |
| 70 | **Ibm709** | Mainframe (1958) | IBM 709 Data Channel Assembly | 1023 | 18 | 29 |
| 71 | **Ibm7090** | Transistor Mainframe (1959) | IBM 7090 Transistorized Mainframe | 1139 | 18 | 46 |
| 72 | **Ibmpc** | Personal Computer (1981) | IBM PC 5150 Real-Mode 8088 Standard | 1151 | 18 | 12 |
| 73 | **Illiac1** | Scientific (1952) | ILLIAC I University Computing | 1073 | 18 | 46 |
| 74 | **Imsai8080** | Microcomputer (1975) | IMSAI 8080 S-100 Bus Microcomputer (WarGames) | 1169 | 18 | 80 |
| 75 | **Johnniac** | IAS Architecture (1953) | JOHNNIAC RAND IAS-Architecture Computer | 1097 | 18 | 63 |
| 76 | **Kim1** | Single-Board (1976) | MOS KIM-1 6502 Single-Board Computer | 1087 | 18 | 80 |
| 77 | **Leo1** | Commercial (1951) | LEO I First Business Computer | 1195 | 18 | 80 |
| 78 | **Loongarch** | Modern (2020s) | Loongson 64-bit General Purpose Assembly | 1195 | 18 | 80 |
| 79 | **M6800** | 8-bit MCU (1974) | Motorola 6800 Early 8-bit Microprocessor | 1123 | 18 | 12 |
| 80 | **M68020** | 32-bit CPU (1984) | Motorola 68020 Full 32-bit Workstation Processor | 1183 | 18 | 29 |
| 81 | **M6809** | 8-bit Advanced (1978) | Motorola 6809 Orthogonal Advanced 8-bit Processor | 1155 | 18 | 12 |
| 82 | **M68Hc11** | Microcontroller | Motorola 8-bit Microcontroller Assembly | 1243 | 18 | 46 |
| 83 | **M68K** | Workstation (1980s) | Motorola 68000 Workstation Assembly | 1223 | 18 | 80 |
| 84 | **Macintosh** | Workstation (1984) | Apple Macintosh 128K QuickDraw GUI | 1243 | 18 | 80 |
| 85 | **Manchester Baby** | Stored-Program (1948) | Manchester Baby Williams Tube Stored-Program | 1345 | 18 | 12 |
| 86 | **Manchester Mark1** | Stored-Program (1949) | Manchester Mark 1 Index Register Williams Tube | 1193 | 18 | 29 |
| 87 | **Maniac1** | Scientific (1952) | MANIAC I Los Alamos Scientific Computer | 1093 | 18 | 46 |
| 88 | **Mark1** | Pioneering (1944) | Harvard Mark I Paper Tape Sequence Control | 1185 | 18 | 12 |
| 89 | **Mips** | Embedded (1980s) | MIPS32/64 Embedded Assembly | 1365 | 18 | 80 |
| 90 | **Mos6502** | 8-bit Classic | MOS 6502 Foundational 8-bit Assembly | 1127 | 18 | 46 |
| 91 | **Msp430** | Ultra-Low-Power MCU | MSP430 16-bit Ultra-Low-Power Assembly | 1241 | 18 | 29 |
| 92 | **Msx** | Home Standard (1983) | MSX Standard Z80 Home Computer | 1143 | 18 | 63 |
| 93 | **Nec Pc98** | Business PC (1982) | NEC PC-9801 x86 Business Computer | 1159 | 18 | 63 |
| 94 | **Nes** | Console (1983) | Nintendo Entertainment System (NES) 8-bit Console | 1259 | 18 | 63 |
| 95 | **Next Computer** | Workstation (1988) | NeXT Computer Workstation & Display PostScript | 1269 | 18 | 63 |
| 96 | **Nova** | Minicomputer (1969) | Data General Nova Minimalist 16-bit Minicomputer | 1259 | 18 | 80 |
| 97 | **Ordvac** | Scientific (1951) | ORDVAC Electrostatic Tube Computer | 1195 | 18 | 29 |
| 98 | **Parisc** | Workstation (1980s) | HP Precision Architecture RISC Assembly | 1319 | 18 | 29 |
| 99 | **Pdp1** | Minicomputer (1959) | DEC PDP-1 18-bit Minicomputer (Spacewar!) | 1127 | 18 | 80 |
| 100 | **Pdp10** | Mainframe (1960s) | DEC PDP-10 36-bit Mainframe Assembly | 1161 | 18 | 12 |
| 101 | **Pdp11** | Minicomputer (1970s) | DEC PDP-11 16-bit Assembly | 1261 | 18 | 12 |
| 102 | **Pdp7** | Minicomputer (1964) | DEC PDP-7 18-bit Minicomputer (Original Unix) | 1155 | 18 | 80 |
| 103 | **Pdp8** | Minicomputer (1960s) | DEC PDP-8 12-bit Minicomputer Assembly | 1261 | 18 | 80 |
| 104 | **Perm** | Drum/Core (1956) | PERM TU Munich Drum & Core Computer | 1071 | 18 | 80 |
| 105 | **Pic** | Microcontroller | Microchip PIC Low-Power Assembly | 1145 | 18 | 63 |
| 106 | **Pilot Ace** | Pioneering (1950) | National Physical Laboratory Pilot ACE Delay Line | 1259 | 18 | 80 |
| 107 | **Ppc** | RISC (1990s) | PowerPC PPC64 High-Reliability Assembly | 1373 | 18 | 63 |
| 108 | **Rca1802** | Spacecraft CMOS (1976) | RCA 1802 Radiation-Hardened Spacecraft Processor | 1151 | 18 | 46 |
| 109 | **Riscv** | Modern (2020s) | RV64GC Vector Assembly | 1367 | 18 | 12 |
| 110 | **S360** | Mainframe (1960s) | IBM System/360 32-bit Mainframe Architecture | 1273 | 18 | 80 |
| 111 | **S390X** | Mainframe (Modern) | IBM Z Mainframe Transactional Assembly | 1369 | 18 | 12 |
| 112 | **Scmp** | Simple MCU (1976) | National Semiconductor SC/MP Microprocessor | 1105 | 18 | 80 |
| 113 | **Sds940** | Time-Sharing (1966) | SDS 940 Berkeley Time-Sharing | 1131 | 18 | 29 |
| 114 | **Seac** | Pioneering (1950) | SEAC Standards Eastern Automatic | 1097 | 18 | 80 |
| 115 | **Sega Genesis** | Console (1988) | Sega Genesis / Mega Drive 16-bit Console | 1235 | 18 | 46 |
| 116 | **Sharp X68000** | Workstation (1987) | Sharp X68000 Motorola 68000 Workstation | 1149 | 18 | 46 |
| 117 | **Signetics2650** | Arcade MCU (1975) | Signetics 2650 Arcade and Video Game Microprocessor | 1123 | 18 | 63 |
| 118 | **Silliac** | University (1956) | SILLIAC University of Sydney IAS Computer | 1055 | 18 | 46 |
| 119 | **Snes** | Console (1990) | Super Nintendo Entertainment System (SNES) | 1257 | 18 | 80 |
| 120 | **Sparc** | Workstation (1980s) | Enterprise/Aerospace Windowed Assembly | 1321 | 18 | 12 |
| 121 | **Superh** | Embedded (1990s) | Hitachi SuperH Embedded Assembly | 1231 | 18 | 29 |
| 122 | **Swac** | Pioneering (1950) | SWAC Standards Western Automatic | 1097 | 18 | 80 |
| 123 | **Symbolics3600** | Lisp Microcode (1983) | Symbolics 3600 36-bit Lisp Workstation Microcode | 1285 | 18 | 63 |
| 124 | **Symbolics Lisp** | Tagged Microcode (1981) | Symbolics Lisp Machine Tagged Architecture Microcode | 1281 | 18 | 80 |
| 125 | **Ti83** | Graphing Calculator (1996) | TI-83 Z80 Graphing Calculator | 1077 | 18 | 80 |
| 126 | **Ti994A** | 16-bit Home (1979) | Texas Instruments TI-99/4A 16-bit Home Computer | 1133 | 18 | 29 |
| 127 | **Tms9900** | 16-bit MCU (1976) | Texas Instruments TMS9900 16-bit Microprocessor | 1209 | 18 | 46 |
| 128 | **Transputer** | Parallel (1983) | Inmos Transputer Occam Parallel Multiprocessing | 1349 | 18 | 12 |
| 129 | **Trs80** | Home Computer (1977) | Tandy TRS-80 Radio Shack Z80 Computer | 1151 | 18 | 12 |
| 130 | **Tx0** | General Architecture | Architecture target for tx0 | 1021 | 18 | 63 |
| 131 | **Univac1** | General Architecture | Architecture target for univac1 | 1099 | 18 | 46 |
| 132 | **Univac1103** | Scientific (1953) | UNIVAC 1103 Scientific Drum Storage Assembly | 1265 | 18 | 12 |
| 133 | **Vax** | Minicomputer (1970s) | DEC VAX Minicomputer Assembly | 1181 | 18 | 63 |
| 134 | **Vic20** | Home Computer (1980) | Commodore VIC-20 Friendly Home Computer | 1125 | 18 | 12 |
| 135 | **Wasm** | Modern (2010s) | Portable Wasm Text Format (.wat) | 1229 | 18 | 80 |
| 136 | **Weizac** | Institute (1955) | WEIZAC Weizmann Institute IAS Computer | 1063 | 18 | 29 |
| 137 | **Whirlwind** | Pioneering Microcode (1951) | MIT Whirlwind I Core Memory & Electrostatic Tube Microcode | 1345 | 18 | 80 |
| 138 | **Witch** | Dekatron Decimal (1951) | WITCH / Harwell Dekatron Cold-Cathode Decimal | 1261 | 18 | 12 |
| 139 | **X86 64** | Modern (2020s) | AVX-512 Vectorized Assembly | 1365 | 18 | 29 |
| 140 | **Xerox Alto** | Workstation (1973) | Xerox Alto First GUI Workstation | 1223 | 18 | 12 |
| 141 | **Xerox Star** | GUI Workstation (1981) | Xerox Star 8010 Mesa ViewPoint GUI Workstation | 1199 | 18 | 12 |
| 142 | **Xtensa** | IoT (2000s) | Tensilica ESP32 IoT Assembly | 1247 | 18 | 29 |
| 143 | **Z1** | Mechanical (1938) | Konrad Zuse Z1 Mechanical Floating-Point Slider | 1323 | 18 | 46 |
| 144 | **Z2** | Hybrid Relay (1940) | Konrad Zuse Z2 Hybrid Relay & Mechanical Memory | 1193 | 18 | 46 |
| 145 | **Z3 Machine** | Pioneering (1941) | Konrad Zuse Z3 Relay Computer Logic | 1261 | 18 | 12 |
| 146 | **Z4** | Pioneering (1945) | Konrad Zuse Z4 Commercial Relay & Punched Strip | 1227 | 18 | 46 |
| 147 | **Z80** | 8-bit Classic | Zilog Z80 8-bit Assembly | 1127 | 18 | 63 |
| 148 | **Zx81** | Home Computer (1981) | Sinclair ZX81 Z80 Home Computer | 1151 | 18 | 80 |
| 149 | **Zx Spectrum** | Home Computer (1982) | Sinclair ZX Spectrum Rubber-Keyed UK Computer | 1241 | 18 | 29 |
