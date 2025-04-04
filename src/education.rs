use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
pub fn Education() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    let education_data = vec![(
        i18n().t("education.university.institution"),
        i18n().t("education.university.date"),
        i18n().t("education.university.description"),
    )];

    rsx! {
        section { class: "px-4 py-2 flex flex-col w-1/2",
            h2 { class: "text-white text-lg font-medium mb-4", "{i18n().t(\"education.title\")}" }

            div { class: "flex flex-col gap-4",
                for (institution, date, description) in education_data.iter() {
                    div { class: "flex flex-col",
                        div { class: "flex flex-col",
                            h3 { class: "text-white text-sm font-medium", "{institution}" }
                            p { class: "text-gray-400 text-xs", "{date}" }
                        }
                        p { class: "text-gray-400 text-[10px] leading-4 mt-1", "{description}" }
                    }
                }
            }
        }
    }
}
