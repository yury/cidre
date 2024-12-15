#[cfg(target_os = "macos")]
mod macos {
    use std::collections::HashMap;

    use cidre::{arc, cf, sec};

    pub(crate) fn main() {
        let policy = sec::Policy::with_props(sec::Policy::apple_code_signing(), None).unwrap();
        let now = cf::Date::new();
        let query = cf::DictionaryOf::with_keys_values(
            &[
                sec::class_key(),
                sec::match_keys::limit(),
                sec::match_keys::policy(),
                sec::match_keys::valid_on_date(),
            ],
            &[
                sec::class::certificate().as_type_ref(),
                sec::match_limit::all(),
                &policy,
                &now,
            ],
        );
        let certs = sec::item_matching(&query).unwrap();

        assert_eq!(certs.get_type_id(), cf::Array::type_id());
        let certs: arc::R<cf::ArrayOf<sec::Cert>> = unsafe { std::mem::transmute(certs) };

        let mut map = HashMap::new();
        let subject_key = sec::cert_oids::x509_v1_subject_name();
        let org_name_label = sec::cert_oids::organization_name();
        let unit_name_label = sec::cert_oids::organizational_unit_name();
        let prop_value_key = sec::prop_keys::value();
        let prop_label_key = sec::prop_keys::label();
        let keys = cf::ArrayOf::from_slice(&[subject_key]);
        for cert in certs.iter() {
            let Ok(vals) = cert.values(&keys) else {
                continue;
            };
            let Some(value) = vals.get(subject_key) else {
                continue;
            };
            let Some(section) = value.get(prop_value_key) else {
                continue;
            };
            assert_eq!(section.get_type_id(), cf::Array::type_id());

            let section: &cf::ArrayOf<cf::DictionaryOf<cf::String, cf::Type>> =
                unsafe { std::mem::transmute(section) };

            let mut team_id = None;
            let mut team_name = None;
            for dict in section.iter() {
                let Some(label) = dict.get(prop_label_key) else {
                    continue;
                };
                let Some(value) = dict.get(prop_value_key) else {
                    continue;
                };
                if value.get_type_id() != cf::String::type_id() {
                    continue;
                }

                let value: &cf::String = unsafe { std::mem::transmute(value) };

                if label.equal(org_name_label) {
                    team_name = Some(value);
                } else if label.equal(unit_name_label) {
                    team_id = Some(value);
                }
            }

            if let (Some(id), Some(name)) = (team_id, team_name) {
                let id = id.to_string();
                let name = name.to_string();
                map.insert(id, name);
            }
        }
        for (id, name) in map {
            println!("{name} ({id})");
        }
    }
}

#[cfg(target_os = "macos")]
use macos::main;

#[cfg(not(target_os = "macos"))]
fn main() {
    todo!()
}
