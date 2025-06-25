//! Pre-defined symbols for common Hyperliquid assets
//!
//! For perpetuals, use the coin name directly (e.g., `BTC`, `ETH`)
//! For spot pairs, use the Hyperliquid notation (e.g., `@0` for PURR/USDC)

use crate::types::symbol::Symbol;

// Define Perpetual Symbols
macro_rules! define_perp_symbols {
    ($( ($name:ident, $index:expr) ),* $(,)?) => {
        $(
            #[doc = concat!(stringify!($name), " Perpetual (index: ", $index, ")")]
            pub const $name: Symbol = Symbol::from_static(stringify!($name));
        )*
    };
}

// Define Perpetual Symbols Literal
macro_rules! define_perp_symbols_literal {
    ($( ($name:ident, $index:expr, $symbol:literal) ),* $(,)?) => {
        $(
            #[doc = concat!(stringify!($name), " Perpetual (index: ", $index, ")")]
            pub const $name: Symbol = Symbol::from_static($symbol);
        )*
    };
}

// Define Spot Symbols
macro_rules! define_spot_symbols {
    ($( ($const_name:ident, $index:expr, $symbol:literal) ),* $(,)?) => {
        $(
            #[doc = concat!($symbol, " Spot (index: ", $index, ", @", $index, ")")]
            pub const $const_name: Symbol = Symbol::from_static(concat!("@", $index));
        )*
    };
}

// ==================== MAINNET ====================

// ==================== MAINNET PERPETUALS ====================

define_perp_symbols!(
    (ACE, 96),
    (ADA, 65),
    (AI, 115),
    (AI16Z, 166),
    (AIXBT, 167),
    (ALGO, 158),
    (ALT, 107),
    (ANIME, 176),
    (APE, 8),
    (APT, 27),
    (AR, 117),
    (ARB, 11),
    (ARK, 55),
    (ATOM, 2),
    (AVAX, 6),
    (BABY, 189),
    (BADGER, 77),
    (BANANA, 49),
    (BCH, 26),
    (BERA, 180),
    (BIGTIME, 59),
    (BIO, 169),
    (BLAST, 137),
    (BLUR, 62),
    (BLZ, 47),
    (BNB, 7),
    (BNT, 56),
    (BOME, 120),
    (BRETT, 134),
    (BSV, 64),
    (BTC, 0),
    (CAKE, 99),
    (CANTO, 57),
    (CATI, 143),
    (CELO, 144),
    (CFX, 21),
    (CHILLGUY, 155),
    (COMP, 29),
    (CRV, 16),
    (CYBER, 45),
    (DOGE, 12),
    (DOOD, 194),
    (DOT, 48),
    (DYDX, 4),
    (DYM, 109),
    (EIGEN, 130),
    (ENA, 122),
    (ENS, 101),
    (ETC, 102),
    (ETH, 1),
    (ETHFI, 121),
    (FARTCOIN, 165),
    (FET, 72),
    (FIL, 80),
    (FRIEND, 43),
    (FTM, 22),
    (FTT, 51),
    (FXS, 32),
    (GALA, 93),
    (GAS, 69),
    (GMT, 86),
    (GMX, 23),
    (GOAT, 149),
    (GRASS, 151),
    (GRIFFAIN, 170),
    (HBAR, 127),
    (HMSTR, 145),
    (HPOS, 33),
    (HYPE, 159),
    (HYPER, 191),
    (ILV, 83),
    (IMX, 84),
    (INIT, 193),
    (INJ, 13),
    (IO, 135),
    (IOTA, 157),
    (IP, 183),
    (JELLY, 179),
    (JTO, 94),
    (JUP, 90),
    (KAITO, 185),
    (KAS, 60),
    (LAUNCHCOIN, 195),
    (LAYER, 182),
    (LDO, 17),
    (LINK, 18),
    (LISTA, 138),
    (LOOM, 52),
    (LTC, 10),
    (MANTA, 104),
    (MATIC, 3),
    (MAV, 97),
    (MAVIA, 110),
    (ME, 160),
    (MELANIA, 175),
    (MEME, 75),
    (MERL, 126),
    (MEW, 139),
    (MINA, 67),
    (MKR, 30),
    (MNT, 123),
    (MOODENG, 150),
    (MORPHO, 173),
    (MOVE, 161),
    (MYRO, 118),
    (NEAR, 74),
    (NEIROETH, 147),
    (NEO, 78),
    (NFTI, 89),
    (NIL, 186),
    (NOT, 132),
    (NTRN, 95),
    (NXPC, 196),
    (OGN, 53),
    (OM, 184),
    (OMNI, 129),
    (ONDO, 106),
    (OP, 9),
    (ORBS, 61),
    (ORDI, 76),
    (OX, 42),
    (PANDORA, 112),
    (PAXG, 187),
    (PENDLE, 70),
    (PENGU, 163),
    (PEOPLE, 100),
    (PIXEL, 114),
    (PNUT, 153),
    (POL, 142),
    (POLYX, 68),
    (POPCAT, 128),
    (PROMPT, 188),
    (PURR, 152),
    (PYTH, 81),
    (RDNT, 54),
    (RENDER, 140),
    (REQ, 58),
    (REZ, 131),
    (RLB, 34),
    (RNDR, 20),
    (RSR, 92),
    (RUNE, 41),
    (S, 172),
    (SAGA, 125),
    (SAND, 156),
    (SCR, 146),
    (SEI, 40),
    (SHIA, 44),
    (SNX, 24),
    (SOL, 5),
    (SOPH, 197),
    (SPX, 171),
    (STG, 71),
    (STRAX, 73),
    (STRK, 113),
    (STX, 19),
    (SUI, 14),
    (SUPER, 87),
    (SUSHI, 82),
    (TAO, 116),
    (TIA, 63),
    (TNSR, 124),
    (TON, 66),
    (TRB, 50),
    (TRUMP, 174),
    (TRX, 37),
    (TST, 181),
    (TURBO, 133),
    (UMA, 105),
    (UNI, 39),
    (UNIBOT, 35),
    (USTC, 88),
    (USUAL, 164),
    (VINE, 177),
    (VIRTUAL, 162),
    (VVV, 178),
    (W, 111),
    (WCT, 190),
    (WIF, 98),
    (WLD, 31),
    (XAI, 103),
    (XLM, 154),
    (XRP, 25),
    (YGG, 36),
    (ZEN, 79),
    (ZEREBRO, 168),
    (ZETA, 108),
    (ZK, 136),
    (ZORA, 192),
    (ZRO, 46),
);

define_perp_symbols_literal!(
    (KBONK, 85, "kBONK"),
    (KDOGS, 141, "kDOGS"),
    (KFLOKI, 119, "kFLOKI"),
    (KLUNC, 91, "kLUNC"),
    (KNEIRO, 148, "kNEIRO"),
    (KPEPE, 15, "kPEPE"),
    (KSHIB, 38, "kSHIB"),
);

// ==================== MAINNET SPOT PAIRS ====================

define_spot_symbols!(
    (ADHD_USDC, 40, "ADHD/USDC"),
    (ANON_USDC, 166, "ANON/USDC"),
    (ANSEM_USDC, 18, "ANSEM/USDC"),
    (ANT_USDC, 55, "ANT/USDC"),
    (ARI_USDC, 53, "ARI/USDC"),
    (ASI_USDC, 36, "ASI/USDC"),
    (ATEHUN_USDC, 51, "ATEHUN/USDC"),
    (AUTIST_USDC, 93, "AUTIST/USDC"),
    (BAGS_USDC, 17, "BAGS/USDC"),
    (BEATS_USDC, 128, "BEATS/USDC"),
    (BERA_USDC, 115, "BERA/USDC"),
    (BID_USDC, 33, "BID/USDC"),
    (BIGBEN_USDC, 25, "BIGBEN/USDC"),
    (BOZO_USDC, 76, "BOZO/USDC"),
    (BUBZ_USDC, 117, "BUBZ/USDC"),
    (BUDDY_USDC, 155, "BUDDY/USDC"),
    (BUSSY_USDC, 81, "BUSSY/USDC"),
    (CAPPY_USDC, 7, "CAPPY/USDC"),
    (CAT_USDC, 124, "CAT/USDC"),
    (CATBAL_USDC, 59, "CATBAL/USDC"),
    (CATNIP_USDC, 26, "CATNIP/USDC"),
    (CHEF_USDC, 106, "CHEF/USDC"),
    (CHINA_USDC, 68, "CHINA/USDC"),
    (CINDY_USDC, 67, "CINDY/USDC"),
    (COOK_USDC, 164, "COOK/USDC"),
    (COPE_USDC, 102, "COPE/USDC"),
    (COZY_USDC, 52, "COZY/USDC"),
    (CZ_USDC, 16, "CZ/USDC"),
    (DEFIN_USDC, 143, "DEFIN/USDC"),
    (DEPIN_USDC, 126, "DEPIN/USDC"),
    (DIABLO_USDC, 159, "DIABLO/USDC"),
    (DROP_USDC, 46, "DROP/USDC"),
    (EARTH_USDC, 97, "EARTH/USDC"),
    (FARM_USDC, 121, "FARM/USDC"),
    (FARMED_USDC, 30, "FARMED/USDC"),
    (FATCAT_USDC, 82, "FATCAT/USDC"),
    (FEIT_USDC, 89, "FEIT/USDC"),
    (FEUSD_USDC, 149, "FEUSD/USDC"),
    (FLASK_USDC, 122, "FLASK/USDC"),
    (FLY_USDC, 135, "FLY/USDC"),
    (FRAC_USDC, 50, "FRAC/USDC"),
    (FRCT_USDC, 167, "FRCT/USDC"),
    (FRIED_USDC, 70, "FRIED/USDC"),
    (FRUDO_USDC, 90, "FRUDO/USDC"),
    (FUCKY_USDC, 15, "FUCKY/USDC"),
    (FUN_USDC, 41, "FUN/USDC"),
    (FUND_USDC, 158, "FUND/USDC"),
    (G_USDC, 75, "G/USDC"),
    (GENESY_USDC, 116, "GENESY/USDC"),
    (GMEOW_USDC, 10, "GMEOW/USDC"),
    (GOD_USDC, 139, "GOD/USDC"),
    (GPT_USDC, 31, "GPT/USDC"),
    (GUESS_USDC, 61, "GUESS/USDC"),
    (GUP_USDC, 29, "GUP/USDC"),
    (H_USDC, 131, "H/USDC"),
    (HAPPY_USDC, 22, "HAPPY/USDC"),
    (HBOOST_USDC, 27, "HBOOST/USDC"),
    (HEAD_USDC, 141, "HEAD/USDC"),
    (HFUN_USDC, 1, "HFUN/USDC"),
    (HGOD_USDC, 95, "HGOD/USDC"),
    (HODL_USDC, 34, "HODL/USDC"),
    (HOLD_USDC, 113, "HOLD/USDC"),
    (HOP_USDC, 100, "HOP/USDC"),
    (HOPE_USDC, 80, "HOPE/USDC"),
    (HORSY_USDC, 174, "HORSY/USDC"),
    (HPEPE_USDC, 44, "HPEPE/USDC"),
    (HPUMP_USDC, 64, "HPUMP/USDC"),
    (HPYH_USDC, 103, "HPYH/USDC"),
    (HWTR_USDC, 138, "HWTR/USDC"),
    (HYENA_USDC, 125, "HYENA/USDC"),
    (HYPE_USDC, 105, "HYPE/USDC"),
    (ILIENS_USDC, 14, "ILIENS/USDC"),
    (JEET_USDC, 45, "JEET/USDC"),
    (JEFF_USDC, 4, "JEFF/USDC"),
    (JPEG_USDC, 144, "JPEG/USDC"),
    (KOBE_USDC, 21, "KOBE/USDC"),
    (LADY_USDC, 42, "LADY/USDC"),
    (LAUNCH_USDC, 120, "LAUNCH/USDC"),
    (LICK_USDC, 2, "LICK/USDC"),
    (LIQD_USDC, 130, "LIQD/USDC"),
    (LIQUID_USDC, 96, "LIQUID/USDC"),
    (LORA_USDC, 58, "LORA/USDC"),
    (LQNA_USDC, 85, "LQNA/USDC"),
    (LUCKY_USDC, 101, "LUCKY/USDC"),
    (MAGA_USDC, 94, "MAGA/USDC"),
    (MANLET_USDC, 3, "MANLET/USDC"),
    (MAXI_USDC, 62, "MAXI/USDC"),
    (MBAPPE_USDC, 47, "MBAPPE/USDC"),
    (MEOW_USDC, 110, "MEOW/USDC"),
    (MOG_USDC, 43, "MOG/USDC"),
    (MON_USDC, 127, "MON/USDC"),
    (MONAD_USDC, 79, "MONAD/USDC"),
    (MUNCH_USDC, 114, "MUNCH/USDC"),
    (NASDAQ_USDC, 86, "NASDAQ/USDC"),
    (NEIRO_USDC, 111, "NEIRO/USDC"),
    (NFT_USDC, 56, "NFT/USDC"),
    (NIGGO_USDC, 99, "NIGGO/USDC"),
    (NMTD_USDC, 63, "NMTD/USDC"),
    (NOCEX_USDC, 71, "NOCEX/USDC"),
    (OMNIX_USDC, 73, "OMNIX/USDC"),
    (ORA_USDC, 129, "ORA/USDC"),
    (OTTI_USDC, 171, "OTTI/USDC"),
    (PANDA_USDC, 38, "PANDA/USDC"),
    (PEAR_USDC, 112, "PEAR/USDC"),
    (PEG_USDC, 162, "PEG/USDC"),
    (PENIS_USDC, 160, "PENIS/USDC"),
    (PEPE_USDC, 11, "PEPE/USDC"),
    (PERP_USDC, 168, "PERP/USDC"),
    (PICKL_USDC, 118, "PICKL/USDC"),
    (PIGEON_USDC, 65, "PIGEON/USDC"),
    (PILL_USDC, 39, "PILL/USDC"),
    (PIP_USDC, 84, "PIP/USDC"),
    (POINTS_USDC, 8, "POINTS/USDC"),
    (PRFI_USDC, 156, "PRFI/USDC"),
    (PUMP_USDC, 20, "PUMP/USDC"),
    (PURR_USDC, 0, "PURR/USDC"),
    (PURRO_USDC, 169, "PURRO/USDC"),
    (PURRPS_USDC, 32, "PURRPS/USDC"),
    (QUANT_USDC, 150, "QUANT/USDC"),
    (RAGE_USDC, 49, "RAGE/USDC"),
    (RANK_USDC, 72, "RANK/USDC"),
    (RAT_USDC, 152, "RAT/USDC"),
    (RETARD_USDC, 109, "RETARD/USDC"),
    (RICH_USDC, 57, "RICH/USDC"),
    (RIP_USDC, 74, "RIP/USDC"),
    (RISE_USDC, 66, "RISE/USDC"),
    (RUB_USDC, 165, "RUB/USDC"),
    (RUG_USDC, 13, "RUG/USDC"),
    (SCHIZO_USDC, 23, "SCHIZO/USDC"),
    (SELL_USDC, 24, "SELL/USDC"),
    (SENT_USDC, 133, "SENT/USDC"),
    (SHEEP_USDC, 119, "SHEEP/USDC"),
    (SHOE_USDC, 78, "SHOE/USDC"),
    (SHREK_USDC, 83, "SHREK/USDC"),
    (SIX_USDC, 5, "SIX/USDC"),
    (SOLV_USDC, 134, "SOLV/USDC"),
    (SOVRN_USDC, 137, "SOVRN/USDC"),
    (SPH_USDC, 77, "SPH/USDC"),
    (STACK_USDC, 69, "STACK/USDC"),
    (STAR_USDC, 132, "STAR/USDC"),
    (STEEL_USDC, 108, "STEEL/USDC"),
    (STRICT_USDC, 92, "STRICT/USDC"),
    (SUCKY_USDC, 28, "SUCKY/USDC"),
    (SYLVI_USDC, 88, "SYLVI/USDC"),
    (TATE_USDC, 19, "TATE/USDC"),
    (TEST_USDC, 48, "TEST/USDC"),
    (TILT_USDC, 153, "TILT/USDC"),
    (TIME_USDC, 136, "TIME/USDC"),
    (TJIF_USDC, 60, "TJIF/USDC"),
    (TREND_USDC, 154, "TREND/USDC"),
    (TRUMP_USDC, 9, "TRUMP/USDC"),
    (UBTC_USDC, 140, "UBTC/USDC"),
    (UETH_USDC, 147, "UETH/USDC"),
    (UFART_USDC, 157, "UFART/USDC"),
    (UP_USDC, 98, "UP/USDC"),
    (USDE_USDC, 146, "USDE/USDC"),
    (USDHL_USDC, 172, "USDHL/USDC"),
    (USDT0_USDC, 161, "USDT0/USDC"),
    (USDXL_USDC, 148, "USDXL/USDC"),
    (USH_USDC, 163, "USH/USDC"),
    (USOL_USDC, 151, "USOL/USDC"),
    (USR_USDC, 170, "USR/USDC"),
    (VAPOR_USDC, 37, "VAPOR/USDC"),
    (VAULT_USDC, 123, "VAULT/USDC"),
    (VEGAS_USDC, 35, "VEGAS/USDC"),
    (VIZN_USDC, 91, "VIZN/USDC"),
    (VORTX_USDC, 142, "VORTX/USDC"),
    (WAGMI_USDC, 6, "WAGMI/USDC"),
    (WASH_USDC, 54, "WASH/USDC"),
    (WHYPI_USDC, 145, "WHYPI/USDC"),
    (WOW_USDC, 107, "WOW/USDC"),
    (XAUT0_USDC, 173, "XAUT0/USDC"),
    (XULIAN_USDC, 12, "XULIAN/USDC"),
    (YAP_USDC, 104, "YAP/USDC"),
    (YEETI_USDC, 87, "YEETI/USDC"),
);

// ==================== TESTNET ====================
// Only major assets included for testnet development

// ==================== TESTNET PERPETUALS ====================

define_perp_symbols_literal!(
    (TEST_API, 1, "API"),
    (TEST_ARB, 13, "ARB"),
    (TEST_ATOM, 2, "ATOM"),
    (TEST_AVAX, 7, "AVAX"),
    (TEST_BNB, 6, "BNB"),
    (TEST_BTC, 3, "BTC"),
    (TEST_ETH, 4, "ETH"),
    (TEST_MATIC, 5, "MATIC"),
    (TEST_OP, 11, "OP"),
    (TEST_SOL, 0, "SOL"),
    (TEST_SUI, 25, "SUI"),
);

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

    #[test]
    fn test_predefined_symbols() {
        use super::*;

        assert_eq!(BTC.as_str(), "BTC");
        assert!(BTC.is_perp());

        assert_eq!(HYPE_USDC.as_str(), "@105");
        assert!(HYPE_USDC.is_spot());
    }

    #[test]
    fn test_runtime_symbol_creation() {
        use super::*;

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
