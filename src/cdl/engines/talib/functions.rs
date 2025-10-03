use crate::cdl::api::patterns::Pattern;
use crate::cdl::api::patterns::Pattern::*;
use crate::cdl::api::settings::Settings;
use std::sync::Once;
use ta_lib_wrapper::{
    TA_CDL3BLACKCROWS, TA_CDL3WHITESOLDIERS, TA_CDLDARKCLOUDCOVER, TA_CDLDOJI, TA_CDLDRAGONFLYDOJI,
    TA_CDLENGULFING, TA_CDLEVENINGSTAR, TA_CDLGRAVESTONEDOJI, TA_CDLHAMMER, TA_CDLHANGINGMAN,
    TA_CDLHARAMI, TA_CDLHARAMICROSS, TA_CDLINVERTEDHAMMER, TA_CDLLONGLINE, TA_CDLMARUBOZU,
    TA_CDLMORNINGSTAR, TA_CDLPIERCING, TA_CDLSHOOTINGSTAR, TA_CDLSHORTLINE, TA_CDLSPINNINGTOP,
};

pub(crate) static mut STAR_PENETRATION: f64 = 0.0;
pub(crate) static mut PIERCING_PENETRATION: f64 = 0.0;

static INIT_PENETRATION: Once = Once::new();

fn initialize_penetration_defaults() {
    INIT_PENETRATION.call_once(|| unsafe {
        STAR_PENETRATION = Settings::default().star_penetration_factor;
        PIERCING_PENETRATION = Settings::default().piercing_penetration_factor;
    });
}

#[allow(non_snake_case)]
unsafe extern "C" fn ta_cdlmorningstar(
    startIdx: i32,
    endIdx: i32,
    inOpen: *const f64,
    inHigh: *const f64,
    inLow: *const f64,
    inClose: *const f64,
    outBegIdx: *mut i32,
    outNBElement: *mut i32,
    outInteger: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    initialize_penetration_defaults();
    let penetration = STAR_PENETRATION;
    TA_CDLMORNINGSTAR(
        startIdx,
        endIdx,
        inOpen,
        inHigh,
        inLow,
        inClose,
        penetration,
        outBegIdx,
        outNBElement,
        outInteger,
    )
}

#[allow(non_snake_case)]
unsafe extern "C" fn ta_cdleveningstar(
    startIdx: i32,
    endIdx: i32,
    inOpen: *const f64,
    inHigh: *const f64,
    inLow: *const f64,
    inClose: *const f64,
    outBegIdx: *mut i32,
    outNBElement: *mut i32,
    outInteger: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    initialize_penetration_defaults();
    let penetration = STAR_PENETRATION;
    TA_CDLEVENINGSTAR(
        startIdx,
        endIdx,
        inOpen,
        inHigh,
        inLow,
        inClose,
        penetration,
        outBegIdx,
        outNBElement,
        outInteger,
    )
}

#[allow(non_snake_case)]
unsafe extern "C" fn ta_cdldarkcloudcover(
    startIdx: i32,
    endIdx: i32,
    inOpen: *const f64,
    inHigh: *const f64,
    inLow: *const f64,
    inClose: *const f64,
    outBegIdx: *mut i32,
    outNBElement: *mut i32,
    outInteger: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    initialize_penetration_defaults();
    let penetration = STAR_PENETRATION;
    TA_CDLDARKCLOUDCOVER(
        startIdx,
        endIdx,
        inOpen,
        inHigh,
        inLow,
        inClose,
        penetration,
        outBegIdx,
        outNBElement,
        outInteger,
    )
}

pub type TaCdlFnPtr = unsafe extern "C" fn(
    i32,
    i32,
    *const f64,
    *const f64,
    *const f64,
    *const f64,
    *mut i32,
    *mut i32,
    *mut i32,
) -> ta_lib_wrapper::TA_RetCode;

#[derive(PartialEq, Eq)]
enum Filter {
    Bullish,
    Bearish,
}

#[allow(non_snake_case, clippy::too_many_arguments)]
unsafe fn filter_wrapper(
    base_fn: TaCdlFnPtr,
    filter_type: Filter,
    startIdx: i32,
    endIdx: i32,
    inOpen: *const f64,
    inHigh: *const f64,
    inLow: *const f64,
    inClose: *const f64,
    outBegIdx: *mut i32,
    outNBElement: *mut i32,
    outInteger: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    let ret_code = base_fn(
        startIdx,
        endIdx,
        inOpen,
        inHigh,
        inLow,
        inClose,
        outBegIdx,
        outNBElement,
        outInteger,
    );

    if ret_code == ta_lib_wrapper::TA_RetCode::TA_SUCCESS {
        for i in 0..*outNBElement {
            let val_ptr = outInteger.add(i as usize);

            match filter_type {
                Filter::Bullish => {
                    *val_ptr = if *val_ptr > 0 { 100 } else { 0 };
                }
                Filter::Bearish => {
                    *val_ptr = if *val_ptr < 0 { 100 } else { 0 };
                }
            }
        }
    }

    ret_code
}

#[allow(non_snake_case)]
unsafe extern "C" fn bullish_engulfing(
    startIdx: i32,
    endIdx: i32,
    inOpen: *const f64,
    inHigh: *const f64,
    inLow: *const f64,
    inClose: *const f64,
    outBegIdx: *mut i32,
    outNBElement: *mut i32,
    outInteger: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(
        TA_CDLENGULFING,
        Filter::Bullish,
        startIdx,
        endIdx,
        inOpen,
        inHigh,
        inLow,
        inClose,
        outBegIdx,
        outNBElement,
        outInteger,
    )
}

#[allow(non_snake_case)]
unsafe extern "C" fn bearish_engulfing(
    startIdx: i32,
    endIdx: i32,
    inOpen: *const f64,
    inHigh: *const f64,
    inLow: *const f64,
    inClose: *const f64,
    outBegIdx: *mut i32,
    outNBElement: *mut i32,
    outInteger: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(
        TA_CDLENGULFING,
        Filter::Bearish,
        startIdx,
        endIdx,
        inOpen,
        inHigh,
        inLow,
        inClose,
        outBegIdx,
        outNBElement,
        outInteger,
    )
}

// Harami
#[allow(non_snake_case)]
unsafe extern "C" fn bullish_harami(
    s: i32,
    e: i32,
    o: *const f64,
    h: *const f64,
    l: *const f64,
    c: *const f64,
    ob: *mut i32,
    on: *mut i32,
    oi: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(TA_CDLHARAMI, Filter::Bullish, s, e, o, h, l, c, ob, on, oi)
}
#[allow(non_snake_case)]
unsafe extern "C" fn bearish_harami(
    s: i32,
    e: i32,
    o: *const f64,
    h: *const f64,
    l: *const f64,
    c: *const f64,
    ob: *mut i32,
    on: *mut i32,
    oi: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(TA_CDLHARAMI, Filter::Bearish, s, e, o, h, l, c, ob, on, oi)
}

// HaramiCross
#[allow(non_snake_case)]
unsafe extern "C" fn bullish_haramicross(
    s: i32,
    e: i32,
    o: *const f64,
    h: *const f64,
    l: *const f64,
    c: *const f64,
    ob: *mut i32,
    on: *mut i32,
    oi: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(
        TA_CDLHARAMICROSS,
        Filter::Bullish,
        s,
        e,
        o,
        h,
        l,
        c,
        ob,
        on,
        oi,
    )
}
#[allow(non_snake_case)]
unsafe extern "C" fn bearish_haramicross(
    s: i32,
    e: i32,
    o: *const f64,
    h: *const f64,
    l: *const f64,
    c: *const f64,
    ob: *mut i32,
    on: *mut i32,
    oi: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(
        TA_CDLHARAMICROSS,
        Filter::Bearish,
        s,
        e,
        o,
        h,
        l,
        c,
        ob,
        on,
        oi,
    )
}

// Marubozu
#[allow(non_snake_case)]
unsafe extern "C" fn bullish_marubozu(
    s: i32,
    e: i32,
    o: *const f64,
    h: *const f64,
    l: *const f64,
    c: *const f64,
    ob: *mut i32,
    on: *mut i32,
    oi: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(
        TA_CDLMARUBOZU,
        Filter::Bullish,
        s,
        e,
        o,
        h,
        l,
        c,
        ob,
        on,
        oi,
    )
}
#[allow(non_snake_case)]
unsafe extern "C" fn bearish_marubozu(
    s: i32,
    e: i32,
    o: *const f64,
    h: *const f64,
    l: *const f64,
    c: *const f64,
    ob: *mut i32,
    on: *mut i32,
    oi: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(
        TA_CDLMARUBOZU,
        Filter::Bearish,
        s,
        e,
        o,
        h,
        l,
        c,
        ob,
        on,
        oi,
    )
}

// LongLine
#[allow(non_snake_case)]
unsafe extern "C" fn bullish_longline(
    s: i32,
    e: i32,
    o: *const f64,
    h: *const f64,
    l: *const f64,
    c: *const f64,
    ob: *mut i32,
    on: *mut i32,
    oi: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(
        TA_CDLLONGLINE,
        Filter::Bullish,
        s,
        e,
        o,
        h,
        l,
        c,
        ob,
        on,
        oi,
    )
}
#[allow(non_snake_case)]
unsafe extern "C" fn bearish_longline(
    s: i32,
    e: i32,
    o: *const f64,
    h: *const f64,
    l: *const f64,
    c: *const f64,
    ob: *mut i32,
    on: *mut i32,
    oi: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(
        TA_CDLLONGLINE,
        Filter::Bearish,
        s,
        e,
        o,
        h,
        l,
        c,
        ob,
        on,
        oi,
    )
}

// ShortLine
#[allow(non_snake_case)]
unsafe extern "C" fn bullish_shortline(
    s: i32,
    e: i32,
    o: *const f64,
    h: *const f64,
    l: *const f64,
    c: *const f64,
    ob: *mut i32,
    on: *mut i32,
    oi: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(
        TA_CDLSHORTLINE,
        Filter::Bullish,
        s,
        e,
        o,
        h,
        l,
        c,
        ob,
        on,
        oi,
    )
}
#[allow(non_snake_case)]
unsafe extern "C" fn bearish_shortline(
    s: i32,
    e: i32,
    o: *const f64,
    h: *const f64,
    l: *const f64,
    c: *const f64,
    ob: *mut i32,
    on: *mut i32,
    oi: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(
        TA_CDLSHORTLINE,
        Filter::Bearish,
        s,
        e,
        o,
        h,
        l,
        c,
        ob,
        on,
        oi,
    )
}

// Kicking
#[allow(non_snake_case)]
unsafe extern "C" fn bullish_kicking(
    s: i32,
    e: i32,
    o: *const f64,
    h: *const f64,
    l: *const f64,
    c: *const f64,
    ob: *mut i32,
    on: *mut i32,
    oi: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(
        ta_lib_wrapper::TA_CDLKICKING,
        Filter::Bullish,
        s,
        e,
        o,
        h,
        l,
        c,
        ob,
        on,
        oi,
    )
}
#[allow(non_snake_case)]
unsafe extern "C" fn bearish_kicking(
    s: i32,
    e: i32,
    o: *const f64,
    h: *const f64,
    l: *const f64,
    c: *const f64,
    ob: *mut i32,
    on: *mut i32,
    oi: *mut i32,
) -> ta_lib_wrapper::TA_RetCode {
    filter_wrapper(
        ta_lib_wrapper::TA_CDLKICKING,
        Filter::Bearish,
        s,
        e,
        o,
        h,
        l,
        c,
        ob,
        on,
        oi,
    )
}

impl Pattern {
    pub(crate) fn ta_lib_function(&self) -> TaCdlFnPtr {
        match self {
            Hammer => TA_CDLHAMMER,
            InvertedHammer => TA_CDLINVERTEDHAMMER,
            ThreeWhiteSoldiers => TA_CDL3WHITESOLDIERS,
            PiercingLine => TA_CDLPIERCING,
            DragonFly => TA_CDLDRAGONFLYDOJI,

            HangingMan => TA_CDLHANGINGMAN,
            ShootingStar => TA_CDLSHOOTINGSTAR,
            ThreeBlackCrows => TA_CDL3BLACKCROWS,
            Gravestone => TA_CDLGRAVESTONEDOJI,

            Doji => TA_CDLDOJI,
            SpinningTop => TA_CDLSPINNINGTOP,

            MorningStar => ta_cdlmorningstar,
            EveningStar => ta_cdleveningstar,
            DarkCloudCover => ta_cdldarkcloudcover,

            BullishEngulfing => bullish_engulfing,
            BearishEngulfing => bearish_engulfing,

            BullishHarami => bullish_harami,
            BearishHarami => bearish_harami,

            BullishHaramiCross => bullish_haramicross,
            BearishHaramiCross => bearish_haramicross,

            BullishMarubozu => bullish_marubozu,
            BearishMarubozu => bearish_marubozu,

            BullishLongLine => bullish_longline,
            BearishLongLine => bearish_longline,

            BullishShortLine => bullish_shortline,
            BearishShortLine => bearish_shortline,

            BullishKicking => bullish_kicking,
            BearishKicking => bearish_kicking,
        }
    }
}
