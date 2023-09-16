#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::data::{model::WeaponModifier, scraper::GetDataError};

    #[test]
    fn should_get_unique_weapons() -> Result<(), GetDataError> {
        use std::collections::HashMap;

        use crate::data::scrape_data;

        let mut data = scrape_data()?;

        let normalize = [
            ("Str.", "Straight"),
            ("Gr.", "Great"),
            ("Man-srp.", "Man-serpent"),
            ("Ptg.", "Painting"),
        ];

        for d in data.iter_mut() {
            for (o, r) in normalize.iter() {
                if d.name.contains(o) {
                    d.name = d.name.replace(o, r);
                }
            }
        }

        fn get_key(name: &str) -> String {
            let first_word = name
                .split_whitespace()
                .next()
                .expect("Name should have at least one word");

            let modifier = WeaponModifier::from_str(first_word).ok();

            match modifier {
                None => name.to_owned(),
                Some(_) => name
                    .split_whitespace()
                    .skip(1)
                    .fold(String::new(), |acc, segment| {
                        if acc.is_empty() {
                            segment.to_owned()
                        } else {
                            format!("{} {}", acc, segment)
                        }
                    }),
            }
        }

        let merged_weapons = data.into_iter().map(|w| (get_key(&w.name), w)).fold(
            HashMap::new(),
            |mut acc, (k, v)| {
                acc.entry(k).or_insert(vec![]).push(v);

                acc
            },
        );

        assert_eq!(merged_weapons.len(), 101);

        let murakumo = merged_weapons
            .get("Murakumo")
            .expect("Murakumo should be here");

        assert!(murakumo.len() == 10);

        let nito_sword = merged_weapons
            .get("Gravelord Sword")
            .expect("Gravelord Sword should be here");

        assert!(nito_sword.len() == 1);

        let outliars: Vec<_> = merged_weapons
            .iter()
            .filter(|(_, v)| v.len() != 1 && v.len() != 10)
            .collect();

        std::fs::write(
            "output_outliars.json",
            serde_json::to_string(&outliars).unwrap(),
        )
        .unwrap();

        Ok(())
    }
}
