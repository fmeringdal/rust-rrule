use chrono::Local;

/// A wrapper around `chrono_tz::Tz` that is able to represent `Local` timezone also.
///
/// # Usage
///
/// ```
/// use rrule::Tz;
///
/// let utc = Tz::UTC;
/// let local = Tz::LOCAL;
/// let berlin = Tz::Europe__Berlin;
/// // From `chrono_tz::Tz`
/// let berlin: Tz = chrono_tz::Tz::Europe__Berlin.into();
/// ```
#[derive(Clone, Copy)]
pub enum Tz {
    /// Local timezone
    Local(Local),
    /// Timezone represented by `chrono_tz::Tz`
    Tz(chrono_tz::Tz),
}

impl Tz {
    /// Name of timezone
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Local(_) => "Local",
            Self::Tz(tz) => tz.name(),
        }
    }

    /// Check if timezone is the Local timezone
    #[must_use]
    pub fn is_local(&self) -> bool {
        match self {
            Self::Local(_) => true,
            Self::Tz(_) => false,
        }
    }

    /// Local timezone
    #[allow(non_upper_case_globals)]
    pub const LOCAL: Self = Self::Local(Local);

    // Duplicating all chrono_tz variants with the following program:
    //    for tz in chrono_tz::TZ_VARIANTS {
    //        let tz_name = tz.name();
    //        let tz_name = if let Some(idx) = tz_name.find("-") {
    //            let next_char = &tz_name[idx + 1..idx + 2];
    //            if "0123456789".contains(next_char) {
    //                tz_name.replace("-", "Minus")
    //            } else {
    //                tz_name.replace("-", "")
    //            }
    //        } else {
    //            tz_name.to_string()
    //        };
    //        let tz_name = tz_name.replace("+", "Plus");
    //        println!("#[allow(non_upper_case_globals)]");
    //        println!("#[allow(missing_docs)]");
    //        println!(
    //            "pub const {}: Self = Self::Tz(chrono_tz::{});",
    //            tz_name.replace("/", "__"),
    //            tz_name.replace("/", "::")
    //        );
    //    }
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Abidjan: Self = Self::Tz(chrono_tz::Africa::Abidjan);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Accra: Self = Self::Tz(chrono_tz::Africa::Accra);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Addis_Ababa: Self = Self::Tz(chrono_tz::Africa::Addis_Ababa);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Algiers: Self = Self::Tz(chrono_tz::Africa::Algiers);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Asmara: Self = Self::Tz(chrono_tz::Africa::Asmara);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Asmera: Self = Self::Tz(chrono_tz::Africa::Asmera);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Bamako: Self = Self::Tz(chrono_tz::Africa::Bamako);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Bangui: Self = Self::Tz(chrono_tz::Africa::Bangui);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Banjul: Self = Self::Tz(chrono_tz::Africa::Banjul);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Bissau: Self = Self::Tz(chrono_tz::Africa::Bissau);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Blantyre: Self = Self::Tz(chrono_tz::Africa::Blantyre);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Brazzaville: Self = Self::Tz(chrono_tz::Africa::Brazzaville);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Bujumbura: Self = Self::Tz(chrono_tz::Africa::Bujumbura);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Cairo: Self = Self::Tz(chrono_tz::Africa::Cairo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Casablanca: Self = Self::Tz(chrono_tz::Africa::Casablanca);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Ceuta: Self = Self::Tz(chrono_tz::Africa::Ceuta);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Conakry: Self = Self::Tz(chrono_tz::Africa::Conakry);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Dakar: Self = Self::Tz(chrono_tz::Africa::Dakar);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Dar_es_Salaam: Self = Self::Tz(chrono_tz::Africa::Dar_es_Salaam);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Djibouti: Self = Self::Tz(chrono_tz::Africa::Djibouti);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Douala: Self = Self::Tz(chrono_tz::Africa::Douala);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__El_Aaiun: Self = Self::Tz(chrono_tz::Africa::El_Aaiun);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Freetown: Self = Self::Tz(chrono_tz::Africa::Freetown);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Gaborone: Self = Self::Tz(chrono_tz::Africa::Gaborone);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Harare: Self = Self::Tz(chrono_tz::Africa::Harare);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Johannesburg: Self = Self::Tz(chrono_tz::Africa::Johannesburg);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Juba: Self = Self::Tz(chrono_tz::Africa::Juba);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Kampala: Self = Self::Tz(chrono_tz::Africa::Kampala);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Khartoum: Self = Self::Tz(chrono_tz::Africa::Khartoum);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Kigali: Self = Self::Tz(chrono_tz::Africa::Kigali);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Kinshasa: Self = Self::Tz(chrono_tz::Africa::Kinshasa);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Lagos: Self = Self::Tz(chrono_tz::Africa::Lagos);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Libreville: Self = Self::Tz(chrono_tz::Africa::Libreville);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Lome: Self = Self::Tz(chrono_tz::Africa::Lome);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Luanda: Self = Self::Tz(chrono_tz::Africa::Luanda);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Lubumbashi: Self = Self::Tz(chrono_tz::Africa::Lubumbashi);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Lusaka: Self = Self::Tz(chrono_tz::Africa::Lusaka);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Malabo: Self = Self::Tz(chrono_tz::Africa::Malabo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Maputo: Self = Self::Tz(chrono_tz::Africa::Maputo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Maseru: Self = Self::Tz(chrono_tz::Africa::Maseru);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Mbabane: Self = Self::Tz(chrono_tz::Africa::Mbabane);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Mogadishu: Self = Self::Tz(chrono_tz::Africa::Mogadishu);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Monrovia: Self = Self::Tz(chrono_tz::Africa::Monrovia);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Nairobi: Self = Self::Tz(chrono_tz::Africa::Nairobi);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Ndjamena: Self = Self::Tz(chrono_tz::Africa::Ndjamena);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Niamey: Self = Self::Tz(chrono_tz::Africa::Niamey);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Nouakchott: Self = Self::Tz(chrono_tz::Africa::Nouakchott);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Ouagadougou: Self = Self::Tz(chrono_tz::Africa::Ouagadougou);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__PortoNovo: Self = Self::Tz(chrono_tz::Africa::PortoNovo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Sao_Tome: Self = Self::Tz(chrono_tz::Africa::Sao_Tome);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Timbuktu: Self = Self::Tz(chrono_tz::Africa::Timbuktu);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Tripoli: Self = Self::Tz(chrono_tz::Africa::Tripoli);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Tunis: Self = Self::Tz(chrono_tz::Africa::Tunis);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Africa__Windhoek: Self = Self::Tz(chrono_tz::Africa::Windhoek);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Adak: Self = Self::Tz(chrono_tz::America::Adak);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Anchorage: Self = Self::Tz(chrono_tz::America::Anchorage);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Anguilla: Self = Self::Tz(chrono_tz::America::Anguilla);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Antigua: Self = Self::Tz(chrono_tz::America::Antigua);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Araguaina: Self = Self::Tz(chrono_tz::America::Araguaina);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__Buenos_Aires: Self =
        Self::Tz(chrono_tz::America::Argentina::Buenos_Aires);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__Catamarca: Self =
        Self::Tz(chrono_tz::America::Argentina::Catamarca);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__ComodRivadavia: Self =
        Self::Tz(chrono_tz::America::Argentina::ComodRivadavia);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__Cordoba: Self = Self::Tz(chrono_tz::America::Argentina::Cordoba);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__Jujuy: Self = Self::Tz(chrono_tz::America::Argentina::Jujuy);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__La_Rioja: Self =
        Self::Tz(chrono_tz::America::Argentina::La_Rioja);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__Mendoza: Self = Self::Tz(chrono_tz::America::Argentina::Mendoza);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__Rio_Gallegos: Self =
        Self::Tz(chrono_tz::America::Argentina::Rio_Gallegos);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__Salta: Self = Self::Tz(chrono_tz::America::Argentina::Salta);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__San_Juan: Self =
        Self::Tz(chrono_tz::America::Argentina::San_Juan);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__San_Luis: Self =
        Self::Tz(chrono_tz::America::Argentina::San_Luis);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__Tucuman: Self = Self::Tz(chrono_tz::America::Argentina::Tucuman);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Argentina__Ushuaia: Self = Self::Tz(chrono_tz::America::Argentina::Ushuaia);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Aruba: Self = Self::Tz(chrono_tz::America::Aruba);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Asuncion: Self = Self::Tz(chrono_tz::America::Asuncion);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Atikokan: Self = Self::Tz(chrono_tz::America::Atikokan);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Atka: Self = Self::Tz(chrono_tz::America::Atka);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Bahia: Self = Self::Tz(chrono_tz::America::Bahia);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Bahia_Banderas: Self = Self::Tz(chrono_tz::America::Bahia_Banderas);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Barbados: Self = Self::Tz(chrono_tz::America::Barbados);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Belem: Self = Self::Tz(chrono_tz::America::Belem);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Belize: Self = Self::Tz(chrono_tz::America::Belize);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__BlancSablon: Self = Self::Tz(chrono_tz::America::BlancSablon);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Boa_Vista: Self = Self::Tz(chrono_tz::America::Boa_Vista);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Bogota: Self = Self::Tz(chrono_tz::America::Bogota);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Boise: Self = Self::Tz(chrono_tz::America::Boise);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Buenos_Aires: Self = Self::Tz(chrono_tz::America::Buenos_Aires);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Cambridge_Bay: Self = Self::Tz(chrono_tz::America::Cambridge_Bay);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Campo_Grande: Self = Self::Tz(chrono_tz::America::Campo_Grande);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Cancun: Self = Self::Tz(chrono_tz::America::Cancun);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Caracas: Self = Self::Tz(chrono_tz::America::Caracas);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Catamarca: Self = Self::Tz(chrono_tz::America::Catamarca);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Cayenne: Self = Self::Tz(chrono_tz::America::Cayenne);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Cayman: Self = Self::Tz(chrono_tz::America::Cayman);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Chicago: Self = Self::Tz(chrono_tz::America::Chicago);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Chihuahua: Self = Self::Tz(chrono_tz::America::Chihuahua);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Coral_Harbour: Self = Self::Tz(chrono_tz::America::Coral_Harbour);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Cordoba: Self = Self::Tz(chrono_tz::America::Cordoba);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Costa_Rica: Self = Self::Tz(chrono_tz::America::Costa_Rica);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Creston: Self = Self::Tz(chrono_tz::America::Creston);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Cuiaba: Self = Self::Tz(chrono_tz::America::Cuiaba);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Curacao: Self = Self::Tz(chrono_tz::America::Curacao);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Danmarkshavn: Self = Self::Tz(chrono_tz::America::Danmarkshavn);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Dawson: Self = Self::Tz(chrono_tz::America::Dawson);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Dawson_Creek: Self = Self::Tz(chrono_tz::America::Dawson_Creek);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Denver: Self = Self::Tz(chrono_tz::America::Denver);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Detroit: Self = Self::Tz(chrono_tz::America::Detroit);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Dominica: Self = Self::Tz(chrono_tz::America::Dominica);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Edmonton: Self = Self::Tz(chrono_tz::America::Edmonton);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Eirunepe: Self = Self::Tz(chrono_tz::America::Eirunepe);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__El_Salvador: Self = Self::Tz(chrono_tz::America::El_Salvador);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Ensenada: Self = Self::Tz(chrono_tz::America::Ensenada);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Fort_Nelson: Self = Self::Tz(chrono_tz::America::Fort_Nelson);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Fort_Wayne: Self = Self::Tz(chrono_tz::America::Fort_Wayne);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Fortaleza: Self = Self::Tz(chrono_tz::America::Fortaleza);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Glace_Bay: Self = Self::Tz(chrono_tz::America::Glace_Bay);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Godthab: Self = Self::Tz(chrono_tz::America::Godthab);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Goose_Bay: Self = Self::Tz(chrono_tz::America::Goose_Bay);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Grand_Turk: Self = Self::Tz(chrono_tz::America::Grand_Turk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Grenada: Self = Self::Tz(chrono_tz::America::Grenada);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Guadeloupe: Self = Self::Tz(chrono_tz::America::Guadeloupe);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Guatemala: Self = Self::Tz(chrono_tz::America::Guatemala);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Guayaquil: Self = Self::Tz(chrono_tz::America::Guayaquil);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Guyana: Self = Self::Tz(chrono_tz::America::Guyana);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Halifax: Self = Self::Tz(chrono_tz::America::Halifax);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Havana: Self = Self::Tz(chrono_tz::America::Havana);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Hermosillo: Self = Self::Tz(chrono_tz::America::Hermosillo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Indiana__Indianapolis: Self =
        Self::Tz(chrono_tz::America::Indiana::Indianapolis);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Indiana__Knox: Self = Self::Tz(chrono_tz::America::Indiana::Knox);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Indiana__Marengo: Self = Self::Tz(chrono_tz::America::Indiana::Marengo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Indiana__Petersburg: Self =
        Self::Tz(chrono_tz::America::Indiana::Petersburg);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Indiana__Tell_City: Self = Self::Tz(chrono_tz::America::Indiana::Tell_City);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Indiana__Vevay: Self = Self::Tz(chrono_tz::America::Indiana::Vevay);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Indiana__Vincennes: Self = Self::Tz(chrono_tz::America::Indiana::Vincennes);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Indiana__Winamac: Self = Self::Tz(chrono_tz::America::Indiana::Winamac);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Indianapolis: Self = Self::Tz(chrono_tz::America::Indianapolis);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Inuvik: Self = Self::Tz(chrono_tz::America::Inuvik);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Iqaluit: Self = Self::Tz(chrono_tz::America::Iqaluit);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Jamaica: Self = Self::Tz(chrono_tz::America::Jamaica);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Jujuy: Self = Self::Tz(chrono_tz::America::Jujuy);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Juneau: Self = Self::Tz(chrono_tz::America::Juneau);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Kentucky__Louisville: Self =
        Self::Tz(chrono_tz::America::Kentucky::Louisville);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Kentucky__Monticello: Self =
        Self::Tz(chrono_tz::America::Kentucky::Monticello);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Knox_IN: Self = Self::Tz(chrono_tz::America::Knox_IN);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Kralendijk: Self = Self::Tz(chrono_tz::America::Kralendijk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__La_Paz: Self = Self::Tz(chrono_tz::America::La_Paz);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Lima: Self = Self::Tz(chrono_tz::America::Lima);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Los_Angeles: Self = Self::Tz(chrono_tz::America::Los_Angeles);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Louisville: Self = Self::Tz(chrono_tz::America::Louisville);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Lower_Princes: Self = Self::Tz(chrono_tz::America::Lower_Princes);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Maceio: Self = Self::Tz(chrono_tz::America::Maceio);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Managua: Self = Self::Tz(chrono_tz::America::Managua);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Manaus: Self = Self::Tz(chrono_tz::America::Manaus);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Marigot: Self = Self::Tz(chrono_tz::America::Marigot);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Martinique: Self = Self::Tz(chrono_tz::America::Martinique);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Matamoros: Self = Self::Tz(chrono_tz::America::Matamoros);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Mazatlan: Self = Self::Tz(chrono_tz::America::Mazatlan);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Mendoza: Self = Self::Tz(chrono_tz::America::Mendoza);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Menominee: Self = Self::Tz(chrono_tz::America::Menominee);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Merida: Self = Self::Tz(chrono_tz::America::Merida);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Metlakatla: Self = Self::Tz(chrono_tz::America::Metlakatla);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Mexico_City: Self = Self::Tz(chrono_tz::America::Mexico_City);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Miquelon: Self = Self::Tz(chrono_tz::America::Miquelon);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Moncton: Self = Self::Tz(chrono_tz::America::Moncton);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Monterrey: Self = Self::Tz(chrono_tz::America::Monterrey);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Montevideo: Self = Self::Tz(chrono_tz::America::Montevideo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Montreal: Self = Self::Tz(chrono_tz::America::Montreal);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Montserrat: Self = Self::Tz(chrono_tz::America::Montserrat);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Nassau: Self = Self::Tz(chrono_tz::America::Nassau);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__New_York: Self = Self::Tz(chrono_tz::America::New_York);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Nipigon: Self = Self::Tz(chrono_tz::America::Nipigon);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Nome: Self = Self::Tz(chrono_tz::America::Nome);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Noronha: Self = Self::Tz(chrono_tz::America::Noronha);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__North_Dakota__Beulah: Self =
        Self::Tz(chrono_tz::America::North_Dakota::Beulah);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__North_Dakota__Center: Self =
        Self::Tz(chrono_tz::America::North_Dakota::Center);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__North_Dakota__New_Salem: Self =
        Self::Tz(chrono_tz::America::North_Dakota::New_Salem);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Nuuk: Self = Self::Tz(chrono_tz::America::Nuuk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Ojinaga: Self = Self::Tz(chrono_tz::America::Ojinaga);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Panama: Self = Self::Tz(chrono_tz::America::Panama);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Pangnirtung: Self = Self::Tz(chrono_tz::America::Pangnirtung);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Paramaribo: Self = Self::Tz(chrono_tz::America::Paramaribo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Phoenix: Self = Self::Tz(chrono_tz::America::Phoenix);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__PortauPrince: Self = Self::Tz(chrono_tz::America::PortauPrince);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Port_of_Spain: Self = Self::Tz(chrono_tz::America::Port_of_Spain);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Porto_Acre: Self = Self::Tz(chrono_tz::America::Porto_Acre);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Porto_Velho: Self = Self::Tz(chrono_tz::America::Porto_Velho);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Puerto_Rico: Self = Self::Tz(chrono_tz::America::Puerto_Rico);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Punta_Arenas: Self = Self::Tz(chrono_tz::America::Punta_Arenas);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Rainy_River: Self = Self::Tz(chrono_tz::America::Rainy_River);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Rankin_Inlet: Self = Self::Tz(chrono_tz::America::Rankin_Inlet);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Recife: Self = Self::Tz(chrono_tz::America::Recife);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Regina: Self = Self::Tz(chrono_tz::America::Regina);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Resolute: Self = Self::Tz(chrono_tz::America::Resolute);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Rio_Branco: Self = Self::Tz(chrono_tz::America::Rio_Branco);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Rosario: Self = Self::Tz(chrono_tz::America::Rosario);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Santa_Isabel: Self = Self::Tz(chrono_tz::America::Santa_Isabel);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Santarem: Self = Self::Tz(chrono_tz::America::Santarem);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Santiago: Self = Self::Tz(chrono_tz::America::Santiago);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Santo_Domingo: Self = Self::Tz(chrono_tz::America::Santo_Domingo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Sao_Paulo: Self = Self::Tz(chrono_tz::America::Sao_Paulo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Scoresbysund: Self = Self::Tz(chrono_tz::America::Scoresbysund);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Shiprock: Self = Self::Tz(chrono_tz::America::Shiprock);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Sitka: Self = Self::Tz(chrono_tz::America::Sitka);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__St_Barthelemy: Self = Self::Tz(chrono_tz::America::St_Barthelemy);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__St_Johns: Self = Self::Tz(chrono_tz::America::St_Johns);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__St_Kitts: Self = Self::Tz(chrono_tz::America::St_Kitts);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__St_Lucia: Self = Self::Tz(chrono_tz::America::St_Lucia);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__St_Thomas: Self = Self::Tz(chrono_tz::America::St_Thomas);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__St_Vincent: Self = Self::Tz(chrono_tz::America::St_Vincent);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Swift_Current: Self = Self::Tz(chrono_tz::America::Swift_Current);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Tegucigalpa: Self = Self::Tz(chrono_tz::America::Tegucigalpa);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Thule: Self = Self::Tz(chrono_tz::America::Thule);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Thunder_Bay: Self = Self::Tz(chrono_tz::America::Thunder_Bay);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Tijuana: Self = Self::Tz(chrono_tz::America::Tijuana);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Toronto: Self = Self::Tz(chrono_tz::America::Toronto);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Tortola: Self = Self::Tz(chrono_tz::America::Tortola);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Vancouver: Self = Self::Tz(chrono_tz::America::Vancouver);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Virgin: Self = Self::Tz(chrono_tz::America::Virgin);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Whitehorse: Self = Self::Tz(chrono_tz::America::Whitehorse);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Winnipeg: Self = Self::Tz(chrono_tz::America::Winnipeg);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Yakutat: Self = Self::Tz(chrono_tz::America::Yakutat);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const America__Yellowknife: Self = Self::Tz(chrono_tz::America::Yellowknife);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Antarctica__Casey: Self = Self::Tz(chrono_tz::Antarctica::Casey);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Antarctica__Davis: Self = Self::Tz(chrono_tz::Antarctica::Davis);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Antarctica__DumontDUrville: Self = Self::Tz(chrono_tz::Antarctica::DumontDUrville);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Antarctica__Macquarie: Self = Self::Tz(chrono_tz::Antarctica::Macquarie);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Antarctica__Mawson: Self = Self::Tz(chrono_tz::Antarctica::Mawson);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Antarctica__McMurdo: Self = Self::Tz(chrono_tz::Antarctica::McMurdo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Antarctica__Palmer: Self = Self::Tz(chrono_tz::Antarctica::Palmer);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Antarctica__Rothera: Self = Self::Tz(chrono_tz::Antarctica::Rothera);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Antarctica__South_Pole: Self = Self::Tz(chrono_tz::Antarctica::South_Pole);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Antarctica__Syowa: Self = Self::Tz(chrono_tz::Antarctica::Syowa);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Antarctica__Troll: Self = Self::Tz(chrono_tz::Antarctica::Troll);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Antarctica__Vostok: Self = Self::Tz(chrono_tz::Antarctica::Vostok);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Arctic__Longyearbyen: Self = Self::Tz(chrono_tz::Arctic::Longyearbyen);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Aden: Self = Self::Tz(chrono_tz::Asia::Aden);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Almaty: Self = Self::Tz(chrono_tz::Asia::Almaty);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Amman: Self = Self::Tz(chrono_tz::Asia::Amman);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Anadyr: Self = Self::Tz(chrono_tz::Asia::Anadyr);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Aqtau: Self = Self::Tz(chrono_tz::Asia::Aqtau);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Aqtobe: Self = Self::Tz(chrono_tz::Asia::Aqtobe);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Ashgabat: Self = Self::Tz(chrono_tz::Asia::Ashgabat);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Ashkhabad: Self = Self::Tz(chrono_tz::Asia::Ashkhabad);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Atyrau: Self = Self::Tz(chrono_tz::Asia::Atyrau);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Baghdad: Self = Self::Tz(chrono_tz::Asia::Baghdad);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Bahrain: Self = Self::Tz(chrono_tz::Asia::Bahrain);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Baku: Self = Self::Tz(chrono_tz::Asia::Baku);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Bangkok: Self = Self::Tz(chrono_tz::Asia::Bangkok);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Barnaul: Self = Self::Tz(chrono_tz::Asia::Barnaul);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Beirut: Self = Self::Tz(chrono_tz::Asia::Beirut);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Bishkek: Self = Self::Tz(chrono_tz::Asia::Bishkek);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Brunei: Self = Self::Tz(chrono_tz::Asia::Brunei);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Calcutta: Self = Self::Tz(chrono_tz::Asia::Calcutta);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Chita: Self = Self::Tz(chrono_tz::Asia::Chita);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Choibalsan: Self = Self::Tz(chrono_tz::Asia::Choibalsan);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Chongqing: Self = Self::Tz(chrono_tz::Asia::Chongqing);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Chungking: Self = Self::Tz(chrono_tz::Asia::Chungking);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Colombo: Self = Self::Tz(chrono_tz::Asia::Colombo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Dacca: Self = Self::Tz(chrono_tz::Asia::Dacca);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Damascus: Self = Self::Tz(chrono_tz::Asia::Damascus);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Dhaka: Self = Self::Tz(chrono_tz::Asia::Dhaka);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Dili: Self = Self::Tz(chrono_tz::Asia::Dili);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Dubai: Self = Self::Tz(chrono_tz::Asia::Dubai);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Dushanbe: Self = Self::Tz(chrono_tz::Asia::Dushanbe);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Famagusta: Self = Self::Tz(chrono_tz::Asia::Famagusta);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Gaza: Self = Self::Tz(chrono_tz::Asia::Gaza);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Harbin: Self = Self::Tz(chrono_tz::Asia::Harbin);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Hebron: Self = Self::Tz(chrono_tz::Asia::Hebron);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Ho_Chi_Minh: Self = Self::Tz(chrono_tz::Asia::Ho_Chi_Minh);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Hong_Kong: Self = Self::Tz(chrono_tz::Asia::Hong_Kong);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Hovd: Self = Self::Tz(chrono_tz::Asia::Hovd);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Irkutsk: Self = Self::Tz(chrono_tz::Asia::Irkutsk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Istanbul: Self = Self::Tz(chrono_tz::Asia::Istanbul);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Jakarta: Self = Self::Tz(chrono_tz::Asia::Jakarta);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Jayapura: Self = Self::Tz(chrono_tz::Asia::Jayapura);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Jerusalem: Self = Self::Tz(chrono_tz::Asia::Jerusalem);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Kabul: Self = Self::Tz(chrono_tz::Asia::Kabul);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Kamchatka: Self = Self::Tz(chrono_tz::Asia::Kamchatka);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Karachi: Self = Self::Tz(chrono_tz::Asia::Karachi);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Kashgar: Self = Self::Tz(chrono_tz::Asia::Kashgar);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Kathmandu: Self = Self::Tz(chrono_tz::Asia::Kathmandu);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Katmandu: Self = Self::Tz(chrono_tz::Asia::Katmandu);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Khandyga: Self = Self::Tz(chrono_tz::Asia::Khandyga);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Kolkata: Self = Self::Tz(chrono_tz::Asia::Kolkata);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Krasnoyarsk: Self = Self::Tz(chrono_tz::Asia::Krasnoyarsk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Kuala_Lumpur: Self = Self::Tz(chrono_tz::Asia::Kuala_Lumpur);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Kuching: Self = Self::Tz(chrono_tz::Asia::Kuching);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Kuwait: Self = Self::Tz(chrono_tz::Asia::Kuwait);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Macao: Self = Self::Tz(chrono_tz::Asia::Macao);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Macau: Self = Self::Tz(chrono_tz::Asia::Macau);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Magadan: Self = Self::Tz(chrono_tz::Asia::Magadan);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Makassar: Self = Self::Tz(chrono_tz::Asia::Makassar);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Manila: Self = Self::Tz(chrono_tz::Asia::Manila);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Muscat: Self = Self::Tz(chrono_tz::Asia::Muscat);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Nicosia: Self = Self::Tz(chrono_tz::Asia::Nicosia);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Novokuznetsk: Self = Self::Tz(chrono_tz::Asia::Novokuznetsk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Novosibirsk: Self = Self::Tz(chrono_tz::Asia::Novosibirsk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Omsk: Self = Self::Tz(chrono_tz::Asia::Omsk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Oral: Self = Self::Tz(chrono_tz::Asia::Oral);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Phnom_Penh: Self = Self::Tz(chrono_tz::Asia::Phnom_Penh);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Pontianak: Self = Self::Tz(chrono_tz::Asia::Pontianak);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Pyongyang: Self = Self::Tz(chrono_tz::Asia::Pyongyang);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Qatar: Self = Self::Tz(chrono_tz::Asia::Qatar);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Qostanay: Self = Self::Tz(chrono_tz::Asia::Qostanay);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Qyzylorda: Self = Self::Tz(chrono_tz::Asia::Qyzylorda);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Rangoon: Self = Self::Tz(chrono_tz::Asia::Rangoon);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Riyadh: Self = Self::Tz(chrono_tz::Asia::Riyadh);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Saigon: Self = Self::Tz(chrono_tz::Asia::Saigon);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Sakhalin: Self = Self::Tz(chrono_tz::Asia::Sakhalin);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Samarkand: Self = Self::Tz(chrono_tz::Asia::Samarkand);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Seoul: Self = Self::Tz(chrono_tz::Asia::Seoul);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Shanghai: Self = Self::Tz(chrono_tz::Asia::Shanghai);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Singapore: Self = Self::Tz(chrono_tz::Asia::Singapore);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Srednekolymsk: Self = Self::Tz(chrono_tz::Asia::Srednekolymsk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Taipei: Self = Self::Tz(chrono_tz::Asia::Taipei);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Tashkent: Self = Self::Tz(chrono_tz::Asia::Tashkent);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Tbilisi: Self = Self::Tz(chrono_tz::Asia::Tbilisi);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Tehran: Self = Self::Tz(chrono_tz::Asia::Tehran);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Tel_Aviv: Self = Self::Tz(chrono_tz::Asia::Tel_Aviv);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Thimbu: Self = Self::Tz(chrono_tz::Asia::Thimbu);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Thimphu: Self = Self::Tz(chrono_tz::Asia::Thimphu);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Tokyo: Self = Self::Tz(chrono_tz::Asia::Tokyo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Tomsk: Self = Self::Tz(chrono_tz::Asia::Tomsk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Ujung_Pandang: Self = Self::Tz(chrono_tz::Asia::Ujung_Pandang);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Ulaanbaatar: Self = Self::Tz(chrono_tz::Asia::Ulaanbaatar);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Ulan_Bator: Self = Self::Tz(chrono_tz::Asia::Ulan_Bator);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Urumqi: Self = Self::Tz(chrono_tz::Asia::Urumqi);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__UstNera: Self = Self::Tz(chrono_tz::Asia::UstNera);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Vientiane: Self = Self::Tz(chrono_tz::Asia::Vientiane);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Vladivostok: Self = Self::Tz(chrono_tz::Asia::Vladivostok);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Yakutsk: Self = Self::Tz(chrono_tz::Asia::Yakutsk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Yangon: Self = Self::Tz(chrono_tz::Asia::Yangon);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Yekaterinburg: Self = Self::Tz(chrono_tz::Asia::Yekaterinburg);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Asia__Yerevan: Self = Self::Tz(chrono_tz::Asia::Yerevan);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Atlantic__Azores: Self = Self::Tz(chrono_tz::Atlantic::Azores);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Atlantic__Bermuda: Self = Self::Tz(chrono_tz::Atlantic::Bermuda);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Atlantic__Canary: Self = Self::Tz(chrono_tz::Atlantic::Canary);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Atlantic__Cape_Verde: Self = Self::Tz(chrono_tz::Atlantic::Cape_Verde);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Atlantic__Faeroe: Self = Self::Tz(chrono_tz::Atlantic::Faeroe);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Atlantic__Faroe: Self = Self::Tz(chrono_tz::Atlantic::Faroe);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Atlantic__Jan_Mayen: Self = Self::Tz(chrono_tz::Atlantic::Jan_Mayen);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Atlantic__Madeira: Self = Self::Tz(chrono_tz::Atlantic::Madeira);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Atlantic__Reykjavik: Self = Self::Tz(chrono_tz::Atlantic::Reykjavik);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Atlantic__South_Georgia: Self = Self::Tz(chrono_tz::Atlantic::South_Georgia);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Atlantic__St_Helena: Self = Self::Tz(chrono_tz::Atlantic::St_Helena);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Atlantic__Stanley: Self = Self::Tz(chrono_tz::Atlantic::Stanley);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__ACT: Self = Self::Tz(chrono_tz::Australia::ACT);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Adelaide: Self = Self::Tz(chrono_tz::Australia::Adelaide);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Brisbane: Self = Self::Tz(chrono_tz::Australia::Brisbane);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Broken_Hill: Self = Self::Tz(chrono_tz::Australia::Broken_Hill);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Canberra: Self = Self::Tz(chrono_tz::Australia::Canberra);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Currie: Self = Self::Tz(chrono_tz::Australia::Currie);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Darwin: Self = Self::Tz(chrono_tz::Australia::Darwin);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Eucla: Self = Self::Tz(chrono_tz::Australia::Eucla);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Hobart: Self = Self::Tz(chrono_tz::Australia::Hobart);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__LHI: Self = Self::Tz(chrono_tz::Australia::LHI);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Lindeman: Self = Self::Tz(chrono_tz::Australia::Lindeman);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Lord_Howe: Self = Self::Tz(chrono_tz::Australia::Lord_Howe);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Melbourne: Self = Self::Tz(chrono_tz::Australia::Melbourne);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__NSW: Self = Self::Tz(chrono_tz::Australia::NSW);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__North: Self = Self::Tz(chrono_tz::Australia::North);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Perth: Self = Self::Tz(chrono_tz::Australia::Perth);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Queensland: Self = Self::Tz(chrono_tz::Australia::Queensland);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__South: Self = Self::Tz(chrono_tz::Australia::South);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Sydney: Self = Self::Tz(chrono_tz::Australia::Sydney);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Tasmania: Self = Self::Tz(chrono_tz::Australia::Tasmania);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Victoria: Self = Self::Tz(chrono_tz::Australia::Victoria);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__West: Self = Self::Tz(chrono_tz::Australia::West);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Australia__Yancowinna: Self = Self::Tz(chrono_tz::Australia::Yancowinna);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Brazil__Acre: Self = Self::Tz(chrono_tz::Brazil::Acre);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Brazil__DeNoronha: Self = Self::Tz(chrono_tz::Brazil::DeNoronha);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Brazil__East: Self = Self::Tz(chrono_tz::Brazil::East);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Brazil__West: Self = Self::Tz(chrono_tz::Brazil::West);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const CET: Self = Self::Tz(chrono_tz::CET);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const CST6CDT: Self = Self::Tz(chrono_tz::CST6CDT);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Canada__Atlantic: Self = Self::Tz(chrono_tz::Canada::Atlantic);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Canada__Central: Self = Self::Tz(chrono_tz::Canada::Central);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Canada__Eastern: Self = Self::Tz(chrono_tz::Canada::Eastern);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Canada__Mountain: Self = Self::Tz(chrono_tz::Canada::Mountain);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Canada__Newfoundland: Self = Self::Tz(chrono_tz::Canada::Newfoundland);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Canada__Pacific: Self = Self::Tz(chrono_tz::Canada::Pacific);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Canada__Saskatchewan: Self = Self::Tz(chrono_tz::Canada::Saskatchewan);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Canada__Yukon: Self = Self::Tz(chrono_tz::Canada::Yukon);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Chile__Continental: Self = Self::Tz(chrono_tz::Chile::Continental);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Chile__EasterIsland: Self = Self::Tz(chrono_tz::Chile::EasterIsland);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Cuba: Self = Self::Tz(chrono_tz::Cuba);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const EET: Self = Self::Tz(chrono_tz::EET);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const EST: Self = Self::Tz(chrono_tz::EST);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const EST5EDT: Self = Self::Tz(chrono_tz::EST5EDT);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Egypt: Self = Self::Tz(chrono_tz::Egypt);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Eire: Self = Self::Tz(chrono_tz::Eire);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMT: Self = Self::Tz(chrono_tz::Etc::GMT);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus0: Self = Self::Tz(chrono_tz::Etc::GMTPlus0);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus1: Self = Self::Tz(chrono_tz::Etc::GMTPlus1);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus10: Self = Self::Tz(chrono_tz::Etc::GMTPlus10);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus11: Self = Self::Tz(chrono_tz::Etc::GMTPlus11);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus12: Self = Self::Tz(chrono_tz::Etc::GMTPlus12);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus2: Self = Self::Tz(chrono_tz::Etc::GMTPlus2);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus3: Self = Self::Tz(chrono_tz::Etc::GMTPlus3);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus4: Self = Self::Tz(chrono_tz::Etc::GMTPlus4);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus5: Self = Self::Tz(chrono_tz::Etc::GMTPlus5);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus6: Self = Self::Tz(chrono_tz::Etc::GMTPlus6);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus7: Self = Self::Tz(chrono_tz::Etc::GMTPlus7);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus8: Self = Self::Tz(chrono_tz::Etc::GMTPlus8);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTPlus9: Self = Self::Tz(chrono_tz::Etc::GMTPlus9);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus0: Self = Self::Tz(chrono_tz::Etc::GMTMinus0);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus1: Self = Self::Tz(chrono_tz::Etc::GMTMinus1);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus10: Self = Self::Tz(chrono_tz::Etc::GMTMinus10);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus11: Self = Self::Tz(chrono_tz::Etc::GMTMinus11);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus12: Self = Self::Tz(chrono_tz::Etc::GMTMinus12);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus13: Self = Self::Tz(chrono_tz::Etc::GMTMinus13);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus14: Self = Self::Tz(chrono_tz::Etc::GMTMinus14);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus2: Self = Self::Tz(chrono_tz::Etc::GMTMinus2);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus3: Self = Self::Tz(chrono_tz::Etc::GMTMinus3);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus4: Self = Self::Tz(chrono_tz::Etc::GMTMinus4);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus5: Self = Self::Tz(chrono_tz::Etc::GMTMinus5);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus6: Self = Self::Tz(chrono_tz::Etc::GMTMinus6);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus7: Self = Self::Tz(chrono_tz::Etc::GMTMinus7);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus8: Self = Self::Tz(chrono_tz::Etc::GMTMinus8);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMTMinus9: Self = Self::Tz(chrono_tz::Etc::GMTMinus9);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__GMT0: Self = Self::Tz(chrono_tz::Etc::GMT0);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__Greenwich: Self = Self::Tz(chrono_tz::Etc::Greenwich);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__UCT: Self = Self::Tz(chrono_tz::Etc::UCT);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__UTC: Self = Self::Tz(chrono_tz::Etc::UTC);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__Universal: Self = Self::Tz(chrono_tz::Etc::Universal);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Etc__Zulu: Self = Self::Tz(chrono_tz::Etc::Zulu);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Amsterdam: Self = Self::Tz(chrono_tz::Europe::Amsterdam);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Andorra: Self = Self::Tz(chrono_tz::Europe::Andorra);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Astrakhan: Self = Self::Tz(chrono_tz::Europe::Astrakhan);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Athens: Self = Self::Tz(chrono_tz::Europe::Athens);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Belfast: Self = Self::Tz(chrono_tz::Europe::Belfast);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Belgrade: Self = Self::Tz(chrono_tz::Europe::Belgrade);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Berlin: Self = Self::Tz(chrono_tz::Europe::Berlin);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Bratislava: Self = Self::Tz(chrono_tz::Europe::Bratislava);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Brussels: Self = Self::Tz(chrono_tz::Europe::Brussels);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Bucharest: Self = Self::Tz(chrono_tz::Europe::Bucharest);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Budapest: Self = Self::Tz(chrono_tz::Europe::Budapest);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Busingen: Self = Self::Tz(chrono_tz::Europe::Busingen);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Chisinau: Self = Self::Tz(chrono_tz::Europe::Chisinau);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Copenhagen: Self = Self::Tz(chrono_tz::Europe::Copenhagen);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Dublin: Self = Self::Tz(chrono_tz::Europe::Dublin);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Gibraltar: Self = Self::Tz(chrono_tz::Europe::Gibraltar);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Guernsey: Self = Self::Tz(chrono_tz::Europe::Guernsey);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Helsinki: Self = Self::Tz(chrono_tz::Europe::Helsinki);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Isle_of_Man: Self = Self::Tz(chrono_tz::Europe::Isle_of_Man);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Istanbul: Self = Self::Tz(chrono_tz::Europe::Istanbul);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Jersey: Self = Self::Tz(chrono_tz::Europe::Jersey);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Kaliningrad: Self = Self::Tz(chrono_tz::Europe::Kaliningrad);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Kiev: Self = Self::Tz(chrono_tz::Europe::Kiev);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Kirov: Self = Self::Tz(chrono_tz::Europe::Kirov);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Lisbon: Self = Self::Tz(chrono_tz::Europe::Lisbon);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Ljubljana: Self = Self::Tz(chrono_tz::Europe::Ljubljana);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__London: Self = Self::Tz(chrono_tz::Europe::London);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Luxembourg: Self = Self::Tz(chrono_tz::Europe::Luxembourg);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Madrid: Self = Self::Tz(chrono_tz::Europe::Madrid);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Malta: Self = Self::Tz(chrono_tz::Europe::Malta);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Mariehamn: Self = Self::Tz(chrono_tz::Europe::Mariehamn);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Minsk: Self = Self::Tz(chrono_tz::Europe::Minsk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Monaco: Self = Self::Tz(chrono_tz::Europe::Monaco);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Moscow: Self = Self::Tz(chrono_tz::Europe::Moscow);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Nicosia: Self = Self::Tz(chrono_tz::Europe::Nicosia);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Oslo: Self = Self::Tz(chrono_tz::Europe::Oslo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Paris: Self = Self::Tz(chrono_tz::Europe::Paris);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Podgorica: Self = Self::Tz(chrono_tz::Europe::Podgorica);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Prague: Self = Self::Tz(chrono_tz::Europe::Prague);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Riga: Self = Self::Tz(chrono_tz::Europe::Riga);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Rome: Self = Self::Tz(chrono_tz::Europe::Rome);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Samara: Self = Self::Tz(chrono_tz::Europe::Samara);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__San_Marino: Self = Self::Tz(chrono_tz::Europe::San_Marino);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Sarajevo: Self = Self::Tz(chrono_tz::Europe::Sarajevo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Saratov: Self = Self::Tz(chrono_tz::Europe::Saratov);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Simferopol: Self = Self::Tz(chrono_tz::Europe::Simferopol);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Skopje: Self = Self::Tz(chrono_tz::Europe::Skopje);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Sofia: Self = Self::Tz(chrono_tz::Europe::Sofia);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Stockholm: Self = Self::Tz(chrono_tz::Europe::Stockholm);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Tallinn: Self = Self::Tz(chrono_tz::Europe::Tallinn);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Tirane: Self = Self::Tz(chrono_tz::Europe::Tirane);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Tiraspol: Self = Self::Tz(chrono_tz::Europe::Tiraspol);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Ulyanovsk: Self = Self::Tz(chrono_tz::Europe::Ulyanovsk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Uzhgorod: Self = Self::Tz(chrono_tz::Europe::Uzhgorod);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Vaduz: Self = Self::Tz(chrono_tz::Europe::Vaduz);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Vatican: Self = Self::Tz(chrono_tz::Europe::Vatican);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Vienna: Self = Self::Tz(chrono_tz::Europe::Vienna);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Vilnius: Self = Self::Tz(chrono_tz::Europe::Vilnius);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Volgograd: Self = Self::Tz(chrono_tz::Europe::Volgograd);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Warsaw: Self = Self::Tz(chrono_tz::Europe::Warsaw);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Zagreb: Self = Self::Tz(chrono_tz::Europe::Zagreb);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Zaporozhye: Self = Self::Tz(chrono_tz::Europe::Zaporozhye);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Europe__Zurich: Self = Self::Tz(chrono_tz::Europe::Zurich);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const GB: Self = Self::Tz(chrono_tz::GB);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const GBEire: Self = Self::Tz(chrono_tz::GBEire);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const GMT: Self = Self::Tz(chrono_tz::GMT);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const GMTPlus0: Self = Self::Tz(chrono_tz::GMTPlus0);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const GMTMinus0: Self = Self::Tz(chrono_tz::GMTMinus0);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const GMT0: Self = Self::Tz(chrono_tz::GMT0);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Greenwich: Self = Self::Tz(chrono_tz::Greenwich);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const HST: Self = Self::Tz(chrono_tz::HST);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Hongkong: Self = Self::Tz(chrono_tz::Hongkong);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Iceland: Self = Self::Tz(chrono_tz::Iceland);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Indian__Antananarivo: Self = Self::Tz(chrono_tz::Indian::Antananarivo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Indian__Chagos: Self = Self::Tz(chrono_tz::Indian::Chagos);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Indian__Christmas: Self = Self::Tz(chrono_tz::Indian::Christmas);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Indian__Cocos: Self = Self::Tz(chrono_tz::Indian::Cocos);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Indian__Comoro: Self = Self::Tz(chrono_tz::Indian::Comoro);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Indian__Kerguelen: Self = Self::Tz(chrono_tz::Indian::Kerguelen);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Indian__Mahe: Self = Self::Tz(chrono_tz::Indian::Mahe);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Indian__Maldives: Self = Self::Tz(chrono_tz::Indian::Maldives);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Indian__Mauritius: Self = Self::Tz(chrono_tz::Indian::Mauritius);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Indian__Mayotte: Self = Self::Tz(chrono_tz::Indian::Mayotte);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Indian__Reunion: Self = Self::Tz(chrono_tz::Indian::Reunion);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Iran: Self = Self::Tz(chrono_tz::Iran);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Israel: Self = Self::Tz(chrono_tz::Israel);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Jamaica: Self = Self::Tz(chrono_tz::Jamaica);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Japan: Self = Self::Tz(chrono_tz::Japan);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Kwajalein: Self = Self::Tz(chrono_tz::Kwajalein);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Libya: Self = Self::Tz(chrono_tz::Libya);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const MET: Self = Self::Tz(chrono_tz::MET);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const MST: Self = Self::Tz(chrono_tz::MST);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const MST7MDT: Self = Self::Tz(chrono_tz::MST7MDT);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Mexico__BajaNorte: Self = Self::Tz(chrono_tz::Mexico::BajaNorte);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Mexico__BajaSur: Self = Self::Tz(chrono_tz::Mexico::BajaSur);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Mexico__General: Self = Self::Tz(chrono_tz::Mexico::General);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const NZ: Self = Self::Tz(chrono_tz::NZ);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const NZCHAT: Self = Self::Tz(chrono_tz::NZCHAT);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Navajo: Self = Self::Tz(chrono_tz::Navajo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const PRC: Self = Self::Tz(chrono_tz::PRC);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const PST8PDT: Self = Self::Tz(chrono_tz::PST8PDT);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Apia: Self = Self::Tz(chrono_tz::Pacific::Apia);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Auckland: Self = Self::Tz(chrono_tz::Pacific::Auckland);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Bougainville: Self = Self::Tz(chrono_tz::Pacific::Bougainville);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Chatham: Self = Self::Tz(chrono_tz::Pacific::Chatham);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Chuuk: Self = Self::Tz(chrono_tz::Pacific::Chuuk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Easter: Self = Self::Tz(chrono_tz::Pacific::Easter);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Efate: Self = Self::Tz(chrono_tz::Pacific::Efate);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Enderbury: Self = Self::Tz(chrono_tz::Pacific::Enderbury);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Fakaofo: Self = Self::Tz(chrono_tz::Pacific::Fakaofo);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Fiji: Self = Self::Tz(chrono_tz::Pacific::Fiji);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Funafuti: Self = Self::Tz(chrono_tz::Pacific::Funafuti);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Galapagos: Self = Self::Tz(chrono_tz::Pacific::Galapagos);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Gambier: Self = Self::Tz(chrono_tz::Pacific::Gambier);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Guadalcanal: Self = Self::Tz(chrono_tz::Pacific::Guadalcanal);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Guam: Self = Self::Tz(chrono_tz::Pacific::Guam);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Honolulu: Self = Self::Tz(chrono_tz::Pacific::Honolulu);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Johnston: Self = Self::Tz(chrono_tz::Pacific::Johnston);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Kanton: Self = Self::Tz(chrono_tz::Pacific::Kanton);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Kiritimati: Self = Self::Tz(chrono_tz::Pacific::Kiritimati);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Kosrae: Self = Self::Tz(chrono_tz::Pacific::Kosrae);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Kwajalein: Self = Self::Tz(chrono_tz::Pacific::Kwajalein);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Majuro: Self = Self::Tz(chrono_tz::Pacific::Majuro);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Marquesas: Self = Self::Tz(chrono_tz::Pacific::Marquesas);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Midway: Self = Self::Tz(chrono_tz::Pacific::Midway);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Nauru: Self = Self::Tz(chrono_tz::Pacific::Nauru);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Niue: Self = Self::Tz(chrono_tz::Pacific::Niue);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Norfolk: Self = Self::Tz(chrono_tz::Pacific::Norfolk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Noumea: Self = Self::Tz(chrono_tz::Pacific::Noumea);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Pago_Pago: Self = Self::Tz(chrono_tz::Pacific::Pago_Pago);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Palau: Self = Self::Tz(chrono_tz::Pacific::Palau);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Pitcairn: Self = Self::Tz(chrono_tz::Pacific::Pitcairn);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Pohnpei: Self = Self::Tz(chrono_tz::Pacific::Pohnpei);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Ponape: Self = Self::Tz(chrono_tz::Pacific::Ponape);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Port_Moresby: Self = Self::Tz(chrono_tz::Pacific::Port_Moresby);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Rarotonga: Self = Self::Tz(chrono_tz::Pacific::Rarotonga);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Saipan: Self = Self::Tz(chrono_tz::Pacific::Saipan);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Samoa: Self = Self::Tz(chrono_tz::Pacific::Samoa);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Tahiti: Self = Self::Tz(chrono_tz::Pacific::Tahiti);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Tarawa: Self = Self::Tz(chrono_tz::Pacific::Tarawa);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Tongatapu: Self = Self::Tz(chrono_tz::Pacific::Tongatapu);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Truk: Self = Self::Tz(chrono_tz::Pacific::Truk);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Wake: Self = Self::Tz(chrono_tz::Pacific::Wake);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Wallis: Self = Self::Tz(chrono_tz::Pacific::Wallis);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Pacific__Yap: Self = Self::Tz(chrono_tz::Pacific::Yap);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Poland: Self = Self::Tz(chrono_tz::Poland);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Portugal: Self = Self::Tz(chrono_tz::Portugal);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const ROC: Self = Self::Tz(chrono_tz::ROC);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const ROK: Self = Self::Tz(chrono_tz::ROK);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Singapore: Self = Self::Tz(chrono_tz::Singapore);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Turkey: Self = Self::Tz(chrono_tz::Turkey);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const UCT: Self = Self::Tz(chrono_tz::UCT);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const US__Alaska: Self = Self::Tz(chrono_tz::US::Alaska);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const US__Aleutian: Self = Self::Tz(chrono_tz::US::Aleutian);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const US__Arizona: Self = Self::Tz(chrono_tz::US::Arizona);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const US__Central: Self = Self::Tz(chrono_tz::US::Central);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const US__EastIndiana: Self = Self::Tz(chrono_tz::US::EastIndiana);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const US__Eastern: Self = Self::Tz(chrono_tz::US::Eastern);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const US__Hawaii: Self = Self::Tz(chrono_tz::US::Hawaii);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const US__IndianaStarke: Self = Self::Tz(chrono_tz::US::IndianaStarke);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const US__Michigan: Self = Self::Tz(chrono_tz::US::Michigan);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const US__Mountain: Self = Self::Tz(chrono_tz::US::Mountain);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const US__Pacific: Self = Self::Tz(chrono_tz::US::Pacific);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const US__Samoa: Self = Self::Tz(chrono_tz::US::Samoa);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const UTC: Self = Self::Tz(chrono_tz::UTC);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Universal: Self = Self::Tz(chrono_tz::Universal);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const WSU: Self = Self::Tz(chrono_tz::WSU);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const WET: Self = Self::Tz(chrono_tz::WET);
    #[allow(non_upper_case_globals)]
    #[allow(missing_docs)]
    pub const Zulu: Self = Self::Tz(chrono_tz::Zulu);
}
