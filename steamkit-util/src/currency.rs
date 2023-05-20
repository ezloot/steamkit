// #[derive(Debug, Clone, Default)]
// struct CurrencyData {
//     prefix: Option<&'static str>,
//     suffix: Option<&'static str>,
//     commas: bool,
//     whole: bool,
// }

// impl CurrencyData {
//     const USD: Self = Self::from_prefix("$");
//     const GBP: Self = Self::from_prefix("£");
//     const EUR: Self = Self::from_suffix_commas("€", true);
//     const CHF: Self = Self::from_suffix(" CHF");
//     const RUB: Self = Self::from_suffix_commas_whole(" p\u{0443}\u{0431}.", true, true);
//     const PLN: Self = Self::from_suffix(" PLN");
//     const BRL: Self = Self::from_suffix_commas(" R$", true);
//     const JPY: Self = Self::from_prefix_whole("¥ ", true);
//     const NOK: Self = Self::from_suffix_commas(" kr", true);
//     const IDR: Self = Self::from_prefix("Rp ");
//     const MYR: Self = Self::from_prefix("RM ");
//     const PHP: Self = Self::from_prefix("P ");
//     const SGD: Self = Self::from_prefix("S$ ");
//     const THB: Self = Self::from_prefix("฿ ");
//     const VND: Self = Self::from_suffix(" VND");
//     const KRW: Self = Self::from_prefix_whole("₩", true);
//     const TRY: Self = Self::from_suffix(" TRY");
//     const UAH: Self = Self::from_suffix(" UAH");
//     const MXN: Self = Self::from_prefix("Mex$ ");
//     const CAD: Self = Self::from_prefix("CDN$ ");
//     const AUD: Self = Self::from_prefix("A$ ");
//     const NZD: Self = Self::from_prefix("NZ$ ");
//     const CNY: Self = Self::from_suffix(" CNY");
//     const INR: Self = Self::from_suffix(" INR");
//     const CLP: Self = Self::from_suffix(" CLP");
//     const PEN: Self = Self::from_suffix(" PEN");
//     const COP: Self = Self::from_suffix(" COP");
//     const ZAR: Self = Self::from_suffix(" ZAR");
//     const HKD: Self = Self::from_suffix(" HKD");
//     const TWD: Self = Self::from_suffix(" TWD");
//     const SAR: Self = Self::from_suffix(" SAR");
//     const AED: Self = Self::from_suffix(" AED");

//     const fn from_prefix(prefix: &'static str) -> Self {
//         Self {
//             prefix: Some(prefix),
//             ..Self::default()
//         }
//     }

//     const fn from_prefix_commas(prefix: &'static str, commas: bool) -> Self {
//         Self {
//             prefix: Some(prefix),
//             commas,
//             ..Self::default()
//         }
//     }

//     const fn from_prefix_whole(prefix: &'static str, whole: bool) -> Self {
//         Self {
//             prefix: Some(prefix),
//             whole,
//             ..Self::default()
//         }
//     }

//     const fn from_suffix(suffix: &'static str) -> Self {
//         Self {
//             suffix: Some(suffix),
//             ..Self::default()
//         }
//     }

//     const fn from_suffix_commas(suffix: &'static str, commas: bool) -> Self {
//         Self {
//             suffix: Some(suffix),
//             commas,
//             ..Self::default()
//         }
//     }

//     const fn from_suffix_commas_whole(suffix: &'static str, commas: bool, whole: bool) -> Self {
//         Self {
//             suffix: Some(suffix),
//             commas,
//             whole,
//             ..Self::default()
//         }
//     }

//     const fn default() -> Self {
//         Self {
//             prefix: None,
//             suffix: None,
//             commas: false,
//             whole: false,
//         }
//     }
// }
