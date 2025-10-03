use crate::cdl::api::patterns::Pattern;
use crate::cdl::api::patterns::Pattern::*;
use crate::cdl::api::settings::Settings;
use std::sync::Once;
use ta_lib_sys::{CDLKICKING, CDL3BLACKCROWS, CDL3WHITESOLDIERS, CDLDARKCLOUDCOVER, CDLDOJI, CDLDRAGONFLYDOJI, CDLENGULFING, CDLEVENINGSTAR, CDLGRAVESTONEDOJI, CDLHAMMER, CDLHANGINGMAN, CDLHARAMI, CDLHARAMICROSS, CDLINVERTEDHAMMER, CDLLONGLINE, CDLMARUBOZU, CDLMORNINGSTAR, CDLPIERCING, CDLSHOOTINGSTAR, CDLSHORTLINE, CDLSPINNINGTOP};
use ta_lib_sys::RetCode;

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
) -> RetCode {
    initialize_penetration_defaults();
    let penetration = STAR_PENETRATION;
    CDLMORNINGSTAR(
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
) -> RetCode {
    initialize_penetration_defaults();
    let penetration = STAR_PENETRATION;
    CDLEVENINGSTAR(
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
) -> RetCode {
    initialize_penetration_defaults();
    let penetration = STAR_PENETRATION;
    CDLDARKCLOUDCOVER(
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
) -> RetCode;

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
) -> RetCode {
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

    if ret_code == RetCode::SUCCESS {
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
) -> RetCode {
    filter_wrapper(
        CDLENGULFING,
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
) -> RetCode {
    filter_wrapper(
        CDLENGULFING,
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
) -> RetCode {
    filter_wrapper(CDLHARAMI, Filter::Bullish, s, e, o, h, l, c, ob, on, oi)
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
) -> RetCode {
    filter_wrapper(CDLHARAMI, Filter::Bearish, s, e, o, h, l, c, ob, on, oi)
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
) -> RetCode {
    filter_wrapper(
        CDLHARAMICROSS,
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
) -> RetCode {
    filter_wrapper(
        CDLHARAMICROSS,
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
) -> RetCode {
    filter_wrapper(
        CDLMARUBOZU,
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
) -> RetCode {
    filter_wrapper(
        CDLMARUBOZU,
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
) -> RetCode {
    filter_wrapper(
        CDLLONGLINE,
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
) -> RetCode {
    filter_wrapper(
        CDLLONGLINE,
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
) -> RetCode {
    filter_wrapper(
        CDLSHORTLINE,
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
) -> RetCode {
    filter_wrapper(
        CDLSHORTLINE,
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
) -> RetCode {
    filter_wrapper(
        CDLKICKING,
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
) -> RetCode {
    filter_wrapper(
        CDLKICKING,
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
            Hammer => CDLHAMMER,
            InvertedHammer => CDLINVERTEDHAMMER,
            ThreeWhiteSoldiers => CDL3WHITESOLDIERS,
            PiercingLine => CDLPIERCING,
            DragonFly => CDLDRAGONFLYDOJI,

            HangingMan => CDLHANGINGMAN,
            ShootingStar => CDLSHOOTINGSTAR,
            ThreeBlackCrows => CDL3BLACKCROWS,
            Gravestone => CDLGRAVESTONEDOJI,

            Doji => CDLDOJI,
            SpinningTop => CDLSPINNINGTOP,

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
