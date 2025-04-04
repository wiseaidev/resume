mod education;
mod experience;
mod header;
mod skills;

use crate::education::Education;
use crate::experience::Experience;
use crate::header::Header;
use crate::skills::Skills;
use dioxus::prelude::*;
use i18nrs::dioxus::I18nProvider;
use std::collections::HashMap;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let translations = HashMap::from([
        ("en", include_str!("../i18n/en/base.json")),
        ("es", include_str!("../i18n/es/base.json")),
        ("fr", include_str!("../i18n/fr/base.json")),
        ("ar", include_str!("../i18n/ar/base.json")),
    ]);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        I18nProvider {
            translations: translations,
            default_language: "en".to_string(),
            main {
                class: "bg-black flex flex-col items-start gap-2.5 overflow-y-auto p-7 h-screen",

                Header {}

                div { class: "flex flex-col items-start w-full",

                    div { class: "flex gap-10 w-full mt-6",
                        Education {}
                        Experience {}
                    }
                }

                Skills {}
            }
        }
    }
}
