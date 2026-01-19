//! Pre-defined symbols for common Hyperliquid assets
//!
//! For perpetuals, use the coin name directly (e.g., `BTC`, `ETH`)
//! For spot pairs, use the Hyperliquid notation (e.g., `@0` for PURR/USDC)

use crate::types::symbol::Symbol;

// ==================== MAINNET ====================

// ==================== MAINNET PERPETUALS ====================

/// AAVE Perpetual (index: 28)
pub const AAVE: Symbol = Symbol::from_static("AAVE");

/// ACE Perpetual (index: 96)
pub const ACE: Symbol = Symbol::from_static("ACE");

/// ADA Perpetual (index: 65)
pub const ADA: Symbol = Symbol::from_static("ADA");

/// AI Perpetual (index: 115)
pub const AI: Symbol = Symbol::from_static("AI");

/// AI16Z Perpetual (index: 166)
pub const AI16Z: Symbol = Symbol::from_static("AI16Z");

/// AIXBT Perpetual (index: 167)
pub const AIXBT: Symbol = Symbol::from_static("AIXBT");

/// ALGO Perpetual (index: 158)
pub const ALGO: Symbol = Symbol::from_static("ALGO");

/// ALT Perpetual (index: 107)
pub const ALT: Symbol = Symbol::from_static("ALT");

/// ANIME Perpetual (index: 176)
pub const ANIME: Symbol = Symbol::from_static("ANIME");

/// APE Perpetual (index: 8)
pub const APE: Symbol = Symbol::from_static("APE");

/// APT Perpetual (index: 27)
pub const APT: Symbol = Symbol::from_static("APT");

/// AR Perpetual (index: 117)
pub const AR: Symbol = Symbol::from_static("AR");

/// ARB Perpetual (index: 11)
pub const ARB: Symbol = Symbol::from_static("ARB");

/// ARK Perpetual (index: 55)
pub const ARK: Symbol = Symbol::from_static("ARK");

/// ATOM Perpetual (index: 2)
pub const ATOM: Symbol = Symbol::from_static("ATOM");

/// AVAX Perpetual (index: 6)
pub const AVAX: Symbol = Symbol::from_static("AVAX");

/// BABY Perpetual (index: 189)
pub const BABY: Symbol = Symbol::from_static("BABY");

/// BADGER Perpetual (index: 77)
pub const BADGER: Symbol = Symbol::from_static("BADGER");

/// BANANA Perpetual (index: 49)
pub const BANANA: Symbol = Symbol::from_static("BANANA");

/// BCH Perpetual (index: 26)
pub const BCH: Symbol = Symbol::from_static("BCH");

/// BERA Perpetual (index: 180)
pub const BERA: Symbol = Symbol::from_static("BERA");

/// BIGTIME Perpetual (index: 59)
pub const BIGTIME: Symbol = Symbol::from_static("BIGTIME");

/// BIO Perpetual (index: 169)
pub const BIO: Symbol = Symbol::from_static("BIO");

/// BLAST Perpetual (index: 137)
pub const BLAST: Symbol = Symbol::from_static("BLAST");

/// BLUR Perpetual (index: 62)
pub const BLUR: Symbol = Symbol::from_static("BLUR");

/// BLZ Perpetual (index: 47)
pub const BLZ: Symbol = Symbol::from_static("BLZ");

/// BNB Perpetual (index: 7)
pub const BNB: Symbol = Symbol::from_static("BNB");

/// BNT Perpetual (index: 56)
pub const BNT: Symbol = Symbol::from_static("BNT");

/// BOME Perpetual (index: 120)
pub const BOME: Symbol = Symbol::from_static("BOME");

/// BRETT Perpetual (index: 134)
pub const BRETT: Symbol = Symbol::from_static("BRETT");

/// BSV Perpetual (index: 64)
pub const BSV: Symbol = Symbol::from_static("BSV");

/// BTC Perpetual (index: 0)
pub const BTC: Symbol = Symbol::from_static("BTC");

/// CAKE Perpetual (index: 99)
pub const CAKE: Symbol = Symbol::from_static("CAKE");

/// CANTO Perpetual (index: 57)
pub const CANTO: Symbol = Symbol::from_static("CANTO");

/// CATI Perpetual (index: 143)
pub const CATI: Symbol = Symbol::from_static("CATI");

/// CELO Perpetual (index: 144)
pub const CELO: Symbol = Symbol::from_static("CELO");

/// CFX Perpetual (index: 21)
pub const CFX: Symbol = Symbol::from_static("CFX");

/// CHILLGUY Perpetual (index: 155)
pub const CHILLGUY: Symbol = Symbol::from_static("CHILLGUY");

/// COMP Perpetual (index: 29)
pub const COMP: Symbol = Symbol::from_static("COMP");

/// CRV Perpetual (index: 16)
pub const CRV: Symbol = Symbol::from_static("CRV");

/// CYBER Perpetual (index: 45)
pub const CYBER: Symbol = Symbol::from_static("CYBER");

/// DOGE Perpetual (index: 12)
pub const DOGE: Symbol = Symbol::from_static("DOGE");

/// DOOD Perpetual (index: 194)
pub const DOOD: Symbol = Symbol::from_static("DOOD");

/// DOT Perpetual (index: 48)
pub const DOT: Symbol = Symbol::from_static("DOT");

/// DYDX Perpetual (index: 4)
pub const DYDX: Symbol = Symbol::from_static("DYDX");

/// DYM Perpetual (index: 109)
pub const DYM: Symbol = Symbol::from_static("DYM");

/// EIGEN Perpetual (index: 130)
pub const EIGEN: Symbol = Symbol::from_static("EIGEN");

/// ENA Perpetual (index: 122)
pub const ENA: Symbol = Symbol::from_static("ENA");

/// ENS Perpetual (index: 101)
pub const ENS: Symbol = Symbol::from_static("ENS");

/// ETC Perpetual (index: 102)
pub const ETC: Symbol = Symbol::from_static("ETC");

/// ETH Perpetual (index: 1)
pub const ETH: Symbol = Symbol::from_static("ETH");

/// ETHFI Perpetual (index: 121)
pub const ETHFI: Symbol = Symbol::from_static("ETHFI");

/// FARTCOIN Perpetual (index: 165)
pub const FARTCOIN: Symbol = Symbol::from_static("FARTCOIN");

/// FET Perpetual (index: 72)
pub const FET: Symbol = Symbol::from_static("FET");

/// FIL Perpetual (index: 80)
pub const FIL: Symbol = Symbol::from_static("FIL");

/// FRIEND Perpetual (index: 43)
pub const FRIEND: Symbol = Symbol::from_static("FRIEND");

/// FTM Perpetual (index: 22)
pub const FTM: Symbol = Symbol::from_static("FTM");

/// FTT Perpetual (index: 51)
pub const FTT: Symbol = Symbol::from_static("FTT");

/// FXS Perpetual (index: 32)
pub const FXS: Symbol = Symbol::from_static("FXS");

/// GALA Perpetual (index: 93)
pub const GALA: Symbol = Symbol::from_static("GALA");

/// GAS Perpetual (index: 69)
pub const GAS: Symbol = Symbol::from_static("GAS");

/// GMT Perpetual (index: 86)
pub const GMT: Symbol = Symbol::from_static("GMT");

/// GMX Perpetual (index: 23)
pub const GMX: Symbol = Symbol::from_static("GMX");

/// GOAT Perpetual (index: 149)
pub const GOAT: Symbol = Symbol::from_static("GOAT");

/// GRASS Perpetual (index: 151)
pub const GRASS: Symbol = Symbol::from_static("GRASS");

/// GRIFFAIN Perpetual (index: 170)
pub const GRIFFAIN: Symbol = Symbol::from_static("GRIFFAIN");

/// HBAR Perpetual (index: 127)
pub const HBAR: Symbol = Symbol::from_static("HBAR");

/// HMSTR Perpetual (index: 145)
pub const HMSTR: Symbol = Symbol::from_static("HMSTR");

/// HPOS Perpetual (index: 33)
pub const HPOS: Symbol = Symbol::from_static("HPOS");

/// HYPE Perpetual (index: 159)
pub const HYPE: Symbol = Symbol::from_static("HYPE");

/// HYPER Perpetual (index: 191)
pub const HYPER: Symbol = Symbol::from_static("HYPER");

/// ILV Perpetual (index: 83)
pub const ILV: Symbol = Symbol::from_static("ILV");

/// IMX Perpetual (index: 84)
pub const IMX: Symbol = Symbol::from_static("IMX");

/// INIT Perpetual (index: 193)
pub const INIT: Symbol = Symbol::from_static("INIT");

/// INJ Perpetual (index: 13)
pub const INJ: Symbol = Symbol::from_static("INJ");

/// IO Perpetual (index: 135)
pub const IO: Symbol = Symbol::from_static("IO");

/// IOTA Perpetual (index: 157)
pub const IOTA: Symbol = Symbol::from_static("IOTA");

/// IP Perpetual (index: 183)
pub const IP: Symbol = Symbol::from_static("IP");

/// JELLY Perpetual (index: 179)
pub const JELLY: Symbol = Symbol::from_static("JELLY");

/// JTO Perpetual (index: 94)
pub const JTO: Symbol = Symbol::from_static("JTO");

/// JUP Perpetual (index: 90)
pub const JUP: Symbol = Symbol::from_static("JUP");

/// KAITO Perpetual (index: 185)
pub const KAITO: Symbol = Symbol::from_static("KAITO");

/// KAS Perpetual (index: 60)
pub const KAS: Symbol = Symbol::from_static("KAS");

/// LAUNCHCOIN Perpetual (index: 195)
pub const LAUNCHCOIN: Symbol = Symbol::from_static("LAUNCHCOIN");

/// LAYER Perpetual (index: 182)
pub const LAYER: Symbol = Symbol::from_static("LAYER");

/// LDO Perpetual (index: 17)
pub const LDO: Symbol = Symbol::from_static("LDO");

/// LINK Perpetual (index: 18)
pub const LINK: Symbol = Symbol::from_static("LINK");

/// LISTA Perpetual (index: 138)
pub const LISTA: Symbol = Symbol::from_static("LISTA");

/// LOOM Perpetual (index: 52)
pub const LOOM: Symbol = Symbol::from_static("LOOM");

/// LTC Perpetual (index: 10)
pub const LTC: Symbol = Symbol::from_static("LTC");

/// MANTA Perpetual (index: 104)
pub const MANTA: Symbol = Symbol::from_static("MANTA");

/// MATIC Perpetual (index: 3)
pub const MATIC: Symbol = Symbol::from_static("MATIC");

/// MAV Perpetual (index: 97)
pub const MAV: Symbol = Symbol::from_static("MAV");

/// MAVIA Perpetual (index: 110)
pub const MAVIA: Symbol = Symbol::from_static("MAVIA");

/// ME Perpetual (index: 160)
pub const ME: Symbol = Symbol::from_static("ME");

/// MELANIA Perpetual (index: 175)
pub const MELANIA: Symbol = Symbol::from_static("MELANIA");

/// MEME Perpetual (index: 75)
pub const MEME: Symbol = Symbol::from_static("MEME");

/// MERL Perpetual (index: 126)
pub const MERL: Symbol = Symbol::from_static("MERL");

/// MEW Perpetual (index: 139)
pub const MEW: Symbol = Symbol::from_static("MEW");

/// MINA Perpetual (index: 67)
pub const MINA: Symbol = Symbol::from_static("MINA");

/// MKR Perpetual (index: 30)
pub const MKR: Symbol = Symbol::from_static("MKR");

/// MNT Perpetual (index: 123)
pub const MNT: Symbol = Symbol::from_static("MNT");

/// MOODENG Perpetual (index: 150)
pub const MOODENG: Symbol = Symbol::from_static("MOODENG");

/// MORPHO Perpetual (index: 173)
pub const MORPHO: Symbol = Symbol::from_static("MORPHO");

/// MOVE Perpetual (index: 161)
pub const MOVE: Symbol = Symbol::from_static("MOVE");

/// MYRO Perpetual (index: 118)
pub const MYRO: Symbol = Symbol::from_static("MYRO");

/// NEAR Perpetual (index: 74)
pub const NEAR: Symbol = Symbol::from_static("NEAR");

/// NEIROETH Perpetual (index: 147)
pub const NEIROETH: Symbol = Symbol::from_static("NEIROETH");

/// NEO Perpetual (index: 78)
pub const NEO: Symbol = Symbol::from_static("NEO");

/// NFTI Perpetual (index: 89)
pub const NFTI: Symbol = Symbol::from_static("NFTI");

/// NIL Perpetual (index: 186)
pub const NIL: Symbol = Symbol::from_static("NIL");

/// NOT Perpetual (index: 132)
pub const NOT: Symbol = Symbol::from_static("NOT");

/// NTRN Perpetual (index: 95)
pub const NTRN: Symbol = Symbol::from_static("NTRN");

/// NXPC Perpetual (index: 196)
pub const NXPC: Symbol = Symbol::from_static("NXPC");

/// OGN Perpetual (index: 53)
pub const OGN: Symbol = Symbol::from_static("OGN");

/// OM Perpetual (index: 184)
pub const OM: Symbol = Symbol::from_static("OM");

/// OMNI Perpetual (index: 129)
pub const OMNI: Symbol = Symbol::from_static("OMNI");

/// ONDO Perpetual (index: 106)
pub const ONDO: Symbol = Symbol::from_static("ONDO");

/// OP Perpetual (index: 9)
pub const OP: Symbol = Symbol::from_static("OP");

/// ORBS Perpetual (index: 61)
pub const ORBS: Symbol = Symbol::from_static("ORBS");

/// ORDI Perpetual (index: 76)
pub const ORDI: Symbol = Symbol::from_static("ORDI");

/// OX Perpetual (index: 42)
pub const OX: Symbol = Symbol::from_static("OX");

/// PANDORA Perpetual (index: 112)
pub const PANDORA: Symbol = Symbol::from_static("PANDORA");

/// PAXG Perpetual (index: 187)
pub const PAXG: Symbol = Symbol::from_static("PAXG");

/// PENDLE Perpetual (index: 70)
pub const PENDLE: Symbol = Symbol::from_static("PENDLE");

/// PENGU Perpetual (index: 163)
pub const PENGU: Symbol = Symbol::from_static("PENGU");

/// PEOPLE Perpetual (index: 100)
pub const PEOPLE: Symbol = Symbol::from_static("PEOPLE");

/// PIXEL Perpetual (index: 114)
pub const PIXEL: Symbol = Symbol::from_static("PIXEL");

/// PNUT Perpetual (index: 153)
pub const PNUT: Symbol = Symbol::from_static("PNUT");

/// POL Perpetual (index: 142)
pub const POL: Symbol = Symbol::from_static("POL");

/// POLYX Perpetual (index: 68)
pub const POLYX: Symbol = Symbol::from_static("POLYX");

/// POPCAT Perpetual (index: 128)
pub const POPCAT: Symbol = Symbol::from_static("POPCAT");

/// PROMPT Perpetual (index: 188)
pub const PROMPT: Symbol = Symbol::from_static("PROMPT");

/// PURR Perpetual (index: 152)
pub const PURR: Symbol = Symbol::from_static("PURR");

/// PYTH Perpetual (index: 81)
pub const PYTH: Symbol = Symbol::from_static("PYTH");

/// RDNT Perpetual (index: 54)
pub const RDNT: Symbol = Symbol::from_static("RDNT");

/// RENDER Perpetual (index: 140)
pub const RENDER: Symbol = Symbol::from_static("RENDER");

/// REQ Perpetual (index: 58)
pub const REQ: Symbol = Symbol::from_static("REQ");

/// REZ Perpetual (index: 131)
pub const REZ: Symbol = Symbol::from_static("REZ");

/// RLB Perpetual (index: 34)
pub const RLB: Symbol = Symbol::from_static("RLB");

/// RNDR Perpetual (index: 20)
pub const RNDR: Symbol = Symbol::from_static("RNDR");

/// RSR Perpetual (index: 92)
pub const RSR: Symbol = Symbol::from_static("RSR");

/// RUNE Perpetual (index: 41)
pub const RUNE: Symbol = Symbol::from_static("RUNE");

/// S Perpetual (index: 172)
pub const S: Symbol = Symbol::from_static("S");

/// SAGA Perpetual (index: 125)
pub const SAGA: Symbol = Symbol::from_static("SAGA");

/// SAND Perpetual (index: 156)
pub const SAND: Symbol = Symbol::from_static("SAND");

/// SCR Perpetual (index: 146)
pub const SCR: Symbol = Symbol::from_static("SCR");

/// SEI Perpetual (index: 40)
pub const SEI: Symbol = Symbol::from_static("SEI");

/// SHIA Perpetual (index: 44)
pub const SHIA: Symbol = Symbol::from_static("SHIA");

/// SNX Perpetual (index: 24)
pub const SNX: Symbol = Symbol::from_static("SNX");

/// SOL Perpetual (index: 5)
pub const SOL: Symbol = Symbol::from_static("SOL");

/// SOPH Perpetual (index: 197)
pub const SOPH: Symbol = Symbol::from_static("SOPH");

/// SPX Perpetual (index: 171)
pub const SPX: Symbol = Symbol::from_static("SPX");

/// STG Perpetual (index: 71)
pub const STG: Symbol = Symbol::from_static("STG");

/// STRAX Perpetual (index: 73)
pub const STRAX: Symbol = Symbol::from_static("STRAX");

/// STRK Perpetual (index: 113)
pub const STRK: Symbol = Symbol::from_static("STRK");

/// STX Perpetual (index: 19)
pub const STX: Symbol = Symbol::from_static("STX");

/// SUI Perpetual (index: 14)
pub const SUI: Symbol = Symbol::from_static("SUI");

/// SUPER Perpetual (index: 87)
pub const SUPER: Symbol = Symbol::from_static("SUPER");

/// SUSHI Perpetual (index: 82)
pub const SUSHI: Symbol = Symbol::from_static("SUSHI");

/// TAO Perpetual (index: 116)
pub const TAO: Symbol = Symbol::from_static("TAO");

/// TIA Perpetual (index: 63)
pub const TIA: Symbol = Symbol::from_static("TIA");

/// TNSR Perpetual (index: 124)
pub const TNSR: Symbol = Symbol::from_static("TNSR");

/// TON Perpetual (index: 66)
pub const TON: Symbol = Symbol::from_static("TON");

/// TRB Perpetual (index: 50)
pub const TRB: Symbol = Symbol::from_static("TRB");

/// TRUMP Perpetual (index: 174)
pub const TRUMP: Symbol = Symbol::from_static("TRUMP");

/// TRX Perpetual (index: 37)
pub const TRX: Symbol = Symbol::from_static("TRX");

/// TST Perpetual (index: 181)
pub const TST: Symbol = Symbol::from_static("TST");

/// TURBO Perpetual (index: 133)
pub const TURBO: Symbol = Symbol::from_static("TURBO");

/// UMA Perpetual (index: 105)
pub const UMA: Symbol = Symbol::from_static("UMA");

/// UNI Perpetual (index: 39)
pub const UNI: Symbol = Symbol::from_static("UNI");

/// UNIBOT Perpetual (index: 35)
pub const UNIBOT: Symbol = Symbol::from_static("UNIBOT");

/// USTC Perpetual (index: 88)
pub const USTC: Symbol = Symbol::from_static("USTC");

/// USUAL Perpetual (index: 164)
pub const USUAL: Symbol = Symbol::from_static("USUAL");

/// VINE Perpetual (index: 177)
pub const VINE: Symbol = Symbol::from_static("VINE");

/// VIRTUAL Perpetual (index: 162)
pub const VIRTUAL: Symbol = Symbol::from_static("VIRTUAL");

/// VVV Perpetual (index: 178)
pub const VVV: Symbol = Symbol::from_static("VVV");

/// W Perpetual (index: 111)
pub const W: Symbol = Symbol::from_static("W");

/// WCT Perpetual (index: 190)
pub const WCT: Symbol = Symbol::from_static("WCT");

/// WIF Perpetual (index: 98)
pub const WIF: Symbol = Symbol::from_static("WIF");

/// WLD Perpetual (index: 31)
pub const WLD: Symbol = Symbol::from_static("WLD");

/// XAI Perpetual (index: 103)
pub const XAI: Symbol = Symbol::from_static("XAI");

/// XLM Perpetual (index: 154)
pub const XLM: Symbol = Symbol::from_static("XLM");

/// XRP Perpetual (index: 25)
pub const XRP: Symbol = Symbol::from_static("XRP");

/// YGG Perpetual (index: 36)
pub const YGG: Symbol = Symbol::from_static("YGG");

/// ZEN Perpetual (index: 79)
pub const ZEN: Symbol = Symbol::from_static("ZEN");

/// ZEREBRO Perpetual (index: 168)
pub const ZEREBRO: Symbol = Symbol::from_static("ZEREBRO");

/// ZETA Perpetual (index: 108)
pub const ZETA: Symbol = Symbol::from_static("ZETA");

/// ZK Perpetual (index: 136)
pub const ZK: Symbol = Symbol::from_static("ZK");

/// ZORA Perpetual (index: 192)
pub const ZORA: Symbol = Symbol::from_static("ZORA");

/// ZRO Perpetual (index: 46)
pub const ZRO: Symbol = Symbol::from_static("ZRO");

/// kBONK Perpetual (index: 85)
pub const KBONK: Symbol = Symbol::from_static("kBONK");

/// kDOGS Perpetual (index: 141)
pub const KDOGS: Symbol = Symbol::from_static("kDOGS");

/// kFLOKI Perpetual (index: 119)
pub const KFLOKI: Symbol = Symbol::from_static("kFLOKI");

/// kLUNC Perpetual (index: 91)
pub const KLUNC: Symbol = Symbol::from_static("kLUNC");

/// kNEIRO Perpetual (index: 148)
pub const KNEIRO: Symbol = Symbol::from_static("kNEIRO");

/// kPEPE Perpetual (index: 15)
pub const KPEPE: Symbol = Symbol::from_static("kPEPE");

/// kSHIB Perpetual (index: 38)
pub const KSHIB: Symbol = Symbol::from_static("kSHIB");

// ==================== MAINNET SPOT PAIRS ====================

/// ADHD/USDC Spot (index: 40, @40)
pub const ADHD_USDC: Symbol = Symbol::from_static("@40");

/// ANON/USDC Spot (index: 166, @166)
pub const ANON_USDC: Symbol = Symbol::from_static("@166");

/// ANSEM/USDC Spot (index: 18, @18)
pub const ANSEM_USDC: Symbol = Symbol::from_static("@18");

/// ANT/USDC Spot (index: 55, @55)
pub const ANT_USDC: Symbol = Symbol::from_static("@55");

/// ARI/USDC Spot (index: 53, @53)
pub const ARI_USDC: Symbol = Symbol::from_static("@53");

/// ASI/USDC Spot (index: 36, @36)
pub const ASI_USDC: Symbol = Symbol::from_static("@36");

/// ATEHUN/USDC Spot (index: 51, @51)
pub const ATEHUN_USDC: Symbol = Symbol::from_static("@51");

/// AUTIST/USDC Spot (index: 93, @93)
pub const AUTIST_USDC: Symbol = Symbol::from_static("@93");

/// BAGS/USDC Spot (index: 17, @17)
pub const BAGS_USDC: Symbol = Symbol::from_static("@17");

/// BEATS/USDC Spot (index: 128, @128)
pub const BEATS_USDC: Symbol = Symbol::from_static("@128");

/// BERA/USDC Spot (index: 115, @115)
pub const BERA_USDC: Symbol = Symbol::from_static("@115");

/// BID/USDC Spot (index: 33, @33)
pub const BID_USDC: Symbol = Symbol::from_static("@33");

/// BIGBEN/USDC Spot (index: 25, @25)
pub const BIGBEN_USDC: Symbol = Symbol::from_static("@25");

/// BOZO/USDC Spot (index: 76, @76)
pub const BOZO_USDC: Symbol = Symbol::from_static("@76");

/// BUBZ/USDC Spot (index: 117, @117)
pub const BUBZ_USDC: Symbol = Symbol::from_static("@117");

/// BUDDY/USDC Spot (index: 155, @155)
pub const BUDDY_USDC: Symbol = Symbol::from_static("@155");

/// BUSSY/USDC Spot (index: 81, @81)
pub const BUSSY_USDC: Symbol = Symbol::from_static("@81");

/// CAPPY/USDC Spot (index: 7, @7)
pub const CAPPY_USDC: Symbol = Symbol::from_static("@7");

/// CAT/USDC Spot (index: 124, @124)
pub const CAT_USDC: Symbol = Symbol::from_static("@124");

/// CATBAL/USDC Spot (index: 59, @59)
pub const CATBAL_USDC: Symbol = Symbol::from_static("@59");

/// CATNIP/USDC Spot (index: 26, @26)
pub const CATNIP_USDC: Symbol = Symbol::from_static("@26");

/// CHEF/USDC Spot (index: 106, @106)
pub const CHEF_USDC: Symbol = Symbol::from_static("@106");

/// CHINA/USDC Spot (index: 68, @68)
pub const CHINA_USDC: Symbol = Symbol::from_static("@68");

/// CINDY/USDC Spot (index: 67, @67)
pub const CINDY_USDC: Symbol = Symbol::from_static("@67");

/// COOK/USDC Spot (index: 164, @164)
pub const COOK_USDC: Symbol = Symbol::from_static("@164");

/// COPE/USDC Spot (index: 102, @102)
pub const COPE_USDC: Symbol = Symbol::from_static("@102");

/// COZY/USDC Spot (index: 52, @52)
pub const COZY_USDC: Symbol = Symbol::from_static("@52");

/// CZ/USDC Spot (index: 16, @16)
pub const CZ_USDC: Symbol = Symbol::from_static("@16");

/// DEFIN/USDC Spot (index: 143, @143)
pub const DEFIN_USDC: Symbol = Symbol::from_static("@143");

/// DEPIN/USDC Spot (index: 126, @126)
pub const DEPIN_USDC: Symbol = Symbol::from_static("@126");

/// DIABLO/USDC Spot (index: 159, @159)
pub const DIABLO_USDC: Symbol = Symbol::from_static("@159");

/// DROP/USDC Spot (index: 46, @46)
pub const DROP_USDC: Symbol = Symbol::from_static("@46");

/// EARTH/USDC Spot (index: 97, @97)
pub const EARTH_USDC: Symbol = Symbol::from_static("@97");

/// FARM/USDC Spot (index: 121, @121)
pub const FARM_USDC: Symbol = Symbol::from_static("@121");

/// FARMED/USDC Spot (index: 30, @30)
pub const FARMED_USDC: Symbol = Symbol::from_static("@30");

/// FATCAT/USDC Spot (index: 82, @82)
pub const FATCAT_USDC: Symbol = Symbol::from_static("@82");

/// FEIT/USDC Spot (index: 89, @89)
pub const FEIT_USDC: Symbol = Symbol::from_static("@89");

/// FEUSD/USDC Spot (index: 149, @149)
pub const FEUSD_USDC: Symbol = Symbol::from_static("@149");

/// FLASK/USDC Spot (index: 122, @122)
pub const FLASK_USDC: Symbol = Symbol::from_static("@122");

/// FLY/USDC Spot (index: 135, @135)
pub const FLY_USDC: Symbol = Symbol::from_static("@135");

/// FRAC/USDC Spot (index: 50, @50)
pub const FRAC_USDC: Symbol = Symbol::from_static("@50");

/// FRCT/USDC Spot (index: 167, @167)
pub const FRCT_USDC: Symbol = Symbol::from_static("@167");

/// FRIED/USDC Spot (index: 70, @70)
pub const FRIED_USDC: Symbol = Symbol::from_static("@70");

/// FRUDO/USDC Spot (index: 90, @90)
pub const FRUDO_USDC: Symbol = Symbol::from_static("@90");

/// FUCKY/USDC Spot (index: 15, @15)
pub const FUCKY_USDC: Symbol = Symbol::from_static("@15");

/// FUN/USDC Spot (index: 41, @41)
pub const FUN_USDC: Symbol = Symbol::from_static("@41");

/// FUND/USDC Spot (index: 158, @158)
pub const FUND_USDC: Symbol = Symbol::from_static("@158");

/// G/USDC Spot (index: 75, @75)
pub const G_USDC: Symbol = Symbol::from_static("@75");

/// GENESY/USDC Spot (index: 116, @116)
pub const GENESY_USDC: Symbol = Symbol::from_static("@116");

/// GMEOW/USDC Spot (index: 10, @10)
pub const GMEOW_USDC: Symbol = Symbol::from_static("@10");

/// GOD/USDC Spot (index: 139, @139)
pub const GOD_USDC: Symbol = Symbol::from_static("@139");

/// GPT/USDC Spot (index: 31, @31)
pub const GPT_USDC: Symbol = Symbol::from_static("@31");

/// GUESS/USDC Spot (index: 61, @61)
pub const GUESS_USDC: Symbol = Symbol::from_static("@61");

/// GUP/USDC Spot (index: 29, @29)
pub const GUP_USDC: Symbol = Symbol::from_static("@29");

/// H/USDC Spot (index: 131, @131)
pub const H_USDC: Symbol = Symbol::from_static("@131");

/// HAPPY/USDC Spot (index: 22, @22)
pub const HAPPY_USDC: Symbol = Symbol::from_static("@22");

/// HBOOST/USDC Spot (index: 27, @27)
pub const HBOOST_USDC: Symbol = Symbol::from_static("@27");

/// HEAD/USDC Spot (index: 141, @141)
pub const HEAD_USDC: Symbol = Symbol::from_static("@141");

/// HFUN/USDC Spot (index: 1, @1)
pub const HFUN_USDC: Symbol = Symbol::from_static("@1");

/// HGOD/USDC Spot (index: 95, @95)
pub const HGOD_USDC: Symbol = Symbol::from_static("@95");

/// HODL/USDC Spot (index: 34, @34)
pub const HODL_USDC: Symbol = Symbol::from_static("@34");

/// HOLD/USDC Spot (index: 113, @113)
pub const HOLD_USDC: Symbol = Symbol::from_static("@113");

/// HOP/USDC Spot (index: 100, @100)
pub const HOP_USDC: Symbol = Symbol::from_static("@100");

/// HOPE/USDC Spot (index: 80, @80)
pub const HOPE_USDC: Symbol = Symbol::from_static("@80");

/// HORSY/USDC Spot (index: 174, @174)
pub const HORSY_USDC: Symbol = Symbol::from_static("@174");

/// HPEPE/USDC Spot (index: 44, @44)
pub const HPEPE_USDC: Symbol = Symbol::from_static("@44");

/// HPUMP/USDC Spot (index: 64, @64)
pub const HPUMP_USDC: Symbol = Symbol::from_static("@64");

/// HPYH/USDC Spot (index: 103, @103)
pub const HPYH_USDC: Symbol = Symbol::from_static("@103");

/// HWTR/USDC Spot (index: 138, @138)
pub const HWTR_USDC: Symbol = Symbol::from_static("@138");

/// HYENA/USDC Spot (index: 125, @125)
pub const HYENA_USDC: Symbol = Symbol::from_static("@125");

/// HYPE/USDC Spot (index: 105, @105)
pub const HYPE_USDC: Symbol = Symbol::from_static("@105");

/// ILIENS/USDC Spot (index: 14, @14)
pub const ILIENS_USDC: Symbol = Symbol::from_static("@14");

/// JEET/USDC Spot (index: 45, @45)
pub const JEET_USDC: Symbol = Symbol::from_static("@45");

/// JEFF/USDC Spot (index: 4, @4)
pub const JEFF_USDC: Symbol = Symbol::from_static("@4");

/// JPEG/USDC Spot (index: 144, @144)
pub const JPEG_USDC: Symbol = Symbol::from_static("@144");

/// KOBE/USDC Spot (index: 21, @21)
pub const KOBE_USDC: Symbol = Symbol::from_static("@21");

/// LADY/USDC Spot (index: 42, @42)
pub const LADY_USDC: Symbol = Symbol::from_static("@42");

/// LAUNCH/USDC Spot (index: 120, @120)
pub const LAUNCH_USDC: Symbol = Symbol::from_static("@120");

/// LICK/USDC Spot (index: 2, @2)
pub const LICK_USDC: Symbol = Symbol::from_static("@2");

/// LIQD/USDC Spot (index: 130, @130)
pub const LIQD_USDC: Symbol = Symbol::from_static("@130");

/// LIQUID/USDC Spot (index: 96, @96)
pub const LIQUID_USDC: Symbol = Symbol::from_static("@96");

/// LORA/USDC Spot (index: 58, @58)
pub const LORA_USDC: Symbol = Symbol::from_static("@58");

/// LQNA/USDC Spot (index: 85, @85)
pub const LQNA_USDC: Symbol = Symbol::from_static("@85");

/// LUCKY/USDC Spot (index: 101, @101)
pub const LUCKY_USDC: Symbol = Symbol::from_static("@101");

/// MAGA/USDC Spot (index: 94, @94)
pub const MAGA_USDC: Symbol = Symbol::from_static("@94");

/// MANLET/USDC Spot (index: 3, @3)
pub const MANLET_USDC: Symbol = Symbol::from_static("@3");

/// MAXI/USDC Spot (index: 62, @62)
pub const MAXI_USDC: Symbol = Symbol::from_static("@62");

/// MBAPPE/USDC Spot (index: 47, @47)
pub const MBAPPE_USDC: Symbol = Symbol::from_static("@47");

/// MEOW/USDC Spot (index: 110, @110)
pub const MEOW_USDC: Symbol = Symbol::from_static("@110");

/// MOG/USDC Spot (index: 43, @43)
pub const MOG_USDC: Symbol = Symbol::from_static("@43");

/// MON/USDC Spot (index: 127, @127)
pub const MON_USDC: Symbol = Symbol::from_static("@127");

/// MONAD/USDC Spot (index: 79, @79)
pub const MONAD_USDC: Symbol = Symbol::from_static("@79");

/// MUNCH/USDC Spot (index: 114, @114)
pub const MUNCH_USDC: Symbol = Symbol::from_static("@114");

/// NASDAQ/USDC Spot (index: 86, @86)
pub const NASDAQ_USDC: Symbol = Symbol::from_static("@86");

/// NEIRO/USDC Spot (index: 111, @111)
pub const NEIRO_USDC: Symbol = Symbol::from_static("@111");

/// NFT/USDC Spot (index: 56, @56)
pub const NFT_USDC: Symbol = Symbol::from_static("@56");

/// NIGGO/USDC Spot (index: 99, @99)
pub const NIGGO_USDC: Symbol = Symbol::from_static("@99");

/// NMTD/USDC Spot (index: 63, @63)
pub const NMTD_USDC: Symbol = Symbol::from_static("@63");

/// NOCEX/USDC Spot (index: 71, @71)
pub const NOCEX_USDC: Symbol = Symbol::from_static("@71");

/// OMNIX/USDC Spot (index: 73, @73)
pub const OMNIX_USDC: Symbol = Symbol::from_static("@73");

/// ORA/USDC Spot (index: 129, @129)
pub const ORA_USDC: Symbol = Symbol::from_static("@129");

/// OTTI/USDC Spot (index: 171, @171)
pub const OTTI_USDC: Symbol = Symbol::from_static("@171");

/// PANDA/USDC Spot (index: 38, @38)
pub const PANDA_USDC: Symbol = Symbol::from_static("@38");

/// PEAR/USDC Spot (index: 112, @112)
pub const PEAR_USDC: Symbol = Symbol::from_static("@112");

/// PEG/USDC Spot (index: 162, @162)
pub const PEG_USDC: Symbol = Symbol::from_static("@162");

/// PENIS/USDC Spot (index: 160, @160)
pub const PENIS_USDC: Symbol = Symbol::from_static("@160");

/// PEPE/USDC Spot (index: 11, @11)
pub const PEPE_USDC: Symbol = Symbol::from_static("@11");

/// PERP/USDC Spot (index: 168, @168)
pub const PERP_USDC: Symbol = Symbol::from_static("@168");

/// PICKL/USDC Spot (index: 118, @118)
pub const PICKL_USDC: Symbol = Symbol::from_static("@118");

/// PIGEON/USDC Spot (index: 65, @65)
pub const PIGEON_USDC: Symbol = Symbol::from_static("@65");

/// PILL/USDC Spot (index: 39, @39)
pub const PILL_USDC: Symbol = Symbol::from_static("@39");

/// PIP/USDC Spot (index: 84, @84)
pub const PIP_USDC: Symbol = Symbol::from_static("@84");

/// POINTS/USDC Spot (index: 8, @8)
pub const POINTS_USDC: Symbol = Symbol::from_static("@8");

/// PRFI/USDC Spot (index: 156, @156)
pub const PRFI_USDC: Symbol = Symbol::from_static("@156");

/// PUMP/USDC Spot (index: 20, @20)
pub const PUMP_USDC: Symbol = Symbol::from_static("@20");

/// PURR/USDC Spot (index: 0, @0)
pub const PURR_USDC: Symbol = Symbol::from_static("@0");

/// PURRO/USDC Spot (index: 169, @169)
pub const PURRO_USDC: Symbol = Symbol::from_static("@169");

/// PURRPS/USDC Spot (index: 32, @32)
pub const PURRPS_USDC: Symbol = Symbol::from_static("@32");

/// QUANT/USDC Spot (index: 150, @150)
pub const QUANT_USDC: Symbol = Symbol::from_static("@150");

/// RAGE/USDC Spot (index: 49, @49)
pub const RAGE_USDC: Symbol = Symbol::from_static("@49");

/// RANK/USDC Spot (index: 72, @72)
pub const RANK_USDC: Symbol = Symbol::from_static("@72");

/// RAT/USDC Spot (index: 152, @152)
pub const RAT_USDC: Symbol = Symbol::from_static("@152");

/// RETARD/USDC Spot (index: 109, @109)
pub const RETARD_USDC: Symbol = Symbol::from_static("@109");

/// RICH/USDC Spot (index: 57, @57)
pub const RICH_USDC: Symbol = Symbol::from_static("@57");

/// RIP/USDC Spot (index: 74, @74)
pub const RIP_USDC: Symbol = Symbol::from_static("@74");

/// RISE/USDC Spot (index: 66, @66)
pub const RISE_USDC: Symbol = Symbol::from_static("@66");

/// RUB/USDC Spot (index: 165, @165)
pub const RUB_USDC: Symbol = Symbol::from_static("@165");

/// RUG/USDC Spot (index: 13, @13)
pub const RUG_USDC: Symbol = Symbol::from_static("@13");

/// SCHIZO/USDC Spot (index: 23, @23)
pub const SCHIZO_USDC: Symbol = Symbol::from_static("@23");

/// SELL/USDC Spot (index: 24, @24)
pub const SELL_USDC: Symbol = Symbol::from_static("@24");

/// SENT/USDC Spot (index: 133, @133)
pub const SENT_USDC: Symbol = Symbol::from_static("@133");

/// SHEEP/USDC Spot (index: 119, @119)
pub const SHEEP_USDC: Symbol = Symbol::from_static("@119");

/// SHOE/USDC Spot (index: 78, @78)
pub const SHOE_USDC: Symbol = Symbol::from_static("@78");

/// SHREK/USDC Spot (index: 83, @83)
pub const SHREK_USDC: Symbol = Symbol::from_static("@83");

/// SIX/USDC Spot (index: 5, @5)
pub const SIX_USDC: Symbol = Symbol::from_static("@5");

/// SOLV/USDC Spot (index: 134, @134)
pub const SOLV_USDC: Symbol = Symbol::from_static("@134");

/// SOVRN/USDC Spot (index: 137, @137)
pub const SOVRN_USDC: Symbol = Symbol::from_static("@137");

/// SPH/USDC Spot (index: 77, @77)
pub const SPH_USDC: Symbol = Symbol::from_static("@77");

/// STACK/USDC Spot (index: 69, @69)
pub const STACK_USDC: Symbol = Symbol::from_static("@69");

/// STAR/USDC Spot (index: 132, @132)
pub const STAR_USDC: Symbol = Symbol::from_static("@132");

/// STEEL/USDC Spot (index: 108, @108)
pub const STEEL_USDC: Symbol = Symbol::from_static("@108");

/// STRICT/USDC Spot (index: 92, @92)
pub const STRICT_USDC: Symbol = Symbol::from_static("@92");

/// SUCKY/USDC Spot (index: 28, @28)
pub const SUCKY_USDC: Symbol = Symbol::from_static("@28");

/// SYLVI/USDC Spot (index: 88, @88)
pub const SYLVI_USDC: Symbol = Symbol::from_static("@88");

/// TATE/USDC Spot (index: 19, @19)
pub const TATE_USDC: Symbol = Symbol::from_static("@19");

/// TEST/USDC Spot (index: 48, @48)
pub const TEST_USDC: Symbol = Symbol::from_static("@48");

/// TILT/USDC Spot (index: 153, @153)
pub const TILT_USDC: Symbol = Symbol::from_static("@153");

/// TIME/USDC Spot (index: 136, @136)
pub const TIME_USDC: Symbol = Symbol::from_static("@136");

/// TJIF/USDC Spot (index: 60, @60)
pub const TJIF_USDC: Symbol = Symbol::from_static("@60");

/// TREND/USDC Spot (index: 154, @154)
pub const TREND_USDC: Symbol = Symbol::from_static("@154");

/// TRUMP/USDC Spot (index: 9, @9)
pub const TRUMP_USDC: Symbol = Symbol::from_static("@9");

/// UBTC/USDC Spot (index: 140, @140)
pub const UBTC_USDC: Symbol = Symbol::from_static("@140");

/// UETH/USDC Spot (index: 147, @147)
pub const UETH_USDC: Symbol = Symbol::from_static("@147");

/// UFART/USDC Spot (index: 157, @157)
pub const UFART_USDC: Symbol = Symbol::from_static("@157");

/// UP/USDC Spot (index: 98, @98)
pub const UP_USDC: Symbol = Symbol::from_static("@98");

/// USDE/USDC Spot (index: 146, @146)
pub const USDE_USDC: Symbol = Symbol::from_static("@146");

/// USDHL/USDC Spot (index: 172, @172)
pub const USDHL_USDC: Symbol = Symbol::from_static("@172");

/// USDT0/USDC Spot (index: 161, @161)
pub const USDT0_USDC: Symbol = Symbol::from_static("@161");

/// USDXL/USDC Spot (index: 148, @148)
pub const USDXL_USDC: Symbol = Symbol::from_static("@148");

/// USH/USDC Spot (index: 163, @163)
pub const USH_USDC: Symbol = Symbol::from_static("@163");

/// USOL/USDC Spot (index: 151, @151)
pub const USOL_USDC: Symbol = Symbol::from_static("@151");

/// USR/USDC Spot (index: 170, @170)
pub const USR_USDC: Symbol = Symbol::from_static("@170");

/// VAPOR/USDC Spot (index: 37, @37)
pub const VAPOR_USDC: Symbol = Symbol::from_static("@37");

/// VAULT/USDC Spot (index: 123, @123)
pub const VAULT_USDC: Symbol = Symbol::from_static("@123");

/// VEGAS/USDC Spot (index: 35, @35)
pub const VEGAS_USDC: Symbol = Symbol::from_static("@35");

/// VIZN/USDC Spot (index: 91, @91)
pub const VIZN_USDC: Symbol = Symbol::from_static("@91");

/// VORTX/USDC Spot (index: 142, @142)
pub const VORTX_USDC: Symbol = Symbol::from_static("@142");

/// WAGMI/USDC Spot (index: 6, @6)
pub const WAGMI_USDC: Symbol = Symbol::from_static("@6");

/// WASH/USDC Spot (index: 54, @54)
pub const WASH_USDC: Symbol = Symbol::from_static("@54");

/// WHYPI/USDC Spot (index: 145, @145)
pub const WHYPI_USDC: Symbol = Symbol::from_static("@145");

/// WOW/USDC Spot (index: 107, @107)
pub const WOW_USDC: Symbol = Symbol::from_static("@107");

/// XAUT0/USDC Spot (index: 173, @173)
pub const XAUT0_USDC: Symbol = Symbol::from_static("@173");

/// XULIAN/USDC Spot (index: 12, @12)
pub const XULIAN_USDC: Symbol = Symbol::from_static("@12");

/// YAP/USDC Spot (index: 104, @104)
pub const YAP_USDC: Symbol = Symbol::from_static("@104");

/// YEETI/USDC Spot (index: 87, @87)
pub const YEETI_USDC: Symbol = Symbol::from_static("@87");

// ==================== TESTNET ====================
// Only major assets included for testnet development

// ==================== TESTNET PERPETUALS ====================

/// APT Perpetual (testnet, index: 1)
pub const TEST_APT: Symbol = Symbol::from_static("APT");

/// ARB Perpetual (testnet, index: 13)
pub const TEST_ARB: Symbol = Symbol::from_static("ARB");

/// ATOM Perpetual (testnet, index: 2)
pub const TEST_ATOM: Symbol = Symbol::from_static("ATOM");

/// AVAX Perpetual (testnet, index: 7)
pub const TEST_AVAX: Symbol = Symbol::from_static("AVAX");

/// BNB Perpetual (testnet, index: 6)
pub const TEST_BNB: Symbol = Symbol::from_static("BNB");

/// BTC Perpetual (testnet, index: 3)
pub const TEST_BTC: Symbol = Symbol::from_static("BTC");

/// ETH Perpetual (testnet, index: 4)
pub const TEST_ETH: Symbol = Symbol::from_static("ETH");

/// MATIC Perpetual (testnet, index: 5)
pub const TEST_MATIC: Symbol = Symbol::from_static("MATIC");

/// OP Perpetual (testnet, index: 11)
pub const TEST_OP: Symbol = Symbol::from_static("OP");

/// SOL Perpetual (testnet, index: 0)
pub const TEST_SOL: Symbol = Symbol::from_static("SOL");

/// SUI Perpetual (testnet, index: 25)
pub const TEST_SUI: Symbol = Symbol::from_static("SUI");

// ==================== TESTNET SPOT PAIRS ====================

/// BTC/USDC Spot (testnet, index: 35, @35)
pub const TEST_BTC_USDC: Symbol = Symbol::from_static("@35");

// ==================== HELPERS ====================

/// USDC - convenience constant for the quote currency
/// Note: This is not a tradeable symbol itself, but useful for clarity
pub const USDC: Symbol = Symbol::from_static("USDC");

/// Create a new symbol at runtime (for assets not yet in the SDK)
///
/// # Example
/// ```
/// use ferrofluid::types::symbols::symbol;
///
/// let new_coin = symbol("NEWCOIN");
/// let new_spot = symbol("@999");
/// ```
pub fn symbol(s: impl Into<String>) -> Symbol {
    Symbol::from(s.into())
}

// ==================== PRELUDE ====================

/// Commonly used symbols for easy importing
///
/// # Example
/// ```
/// use ferrofluid::types::symbols::prelude::*;
///
/// // Now you can use BTC, ETH, etc. directly
/// assert_eq!(BTC.as_str(), "BTC");
/// assert_eq!(HYPE_USDC.as_str(), "@105");
///
/// // Create runtime symbols
/// let new_coin = symbol("NEWCOIN");
/// assert_eq!(new_coin.as_str(), "NEWCOIN");
/// ```
pub mod prelude {
    pub use super::{
        // Runtime symbol creation
        symbol,
        // Popular alts
        APT,
        ARB,
        AVAX,
        BNB,
        // Major perpetuals
        BTC,
        DOGE,

        ETH,
        // Hyperliquid native
        HYPE,
        // Major spot pairs
        HYPE_USDC,
        INJ,
        KPEPE,

        MATIC,
        OP,
        PURR,

        PURR_USDC,

        SEI,
        SOL,
        SUI,
        // Testnet symbols
        TEST_BTC,
        TEST_ETH,
        TEST_SOL,

        TIA,
        // Common quote currency
        USDC,

        WIF,
    };
    // Re-export Symbol type for convenience
    pub use crate::types::symbol::Symbol;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predefined_symbols() {
        assert_eq!(BTC.as_str(), "BTC");
        assert!(BTC.is_perp());

        assert_eq!(HYPE_USDC.as_str(), "@105");
        assert!(HYPE_USDC.is_spot());
    }

    #[test]
    fn test_runtime_symbol_creation() {
        let new_perp = symbol("NEWCOIN");
        assert_eq!(new_perp.as_str(), "NEWCOIN");
        assert!(new_perp.is_perp());

        let new_spot = symbol("@999");
        assert_eq!(new_spot.as_str(), "@999");
        assert!(new_spot.is_spot());
    }

    #[test]
    fn test_prelude_imports() {
        // Test that prelude symbols work
        use crate::types::symbols::prelude::*;

        assert_eq!(BTC.as_str(), "BTC");
        assert_eq!(ETH.as_str(), "ETH");
        assert_eq!(HYPE_USDC.as_str(), "@105");

        // Test runtime creation through prelude
        let custom = symbol("CUSTOM");
        assert_eq!(custom.as_str(), "CUSTOM");
    }
}
