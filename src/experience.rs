use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
pub fn Experience() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    let experience_data = vec![
        (
            "Freelance",
            "Full Stack Developer",
            "2020 - Present",
            "experience.description_1",
        ),
        ("", "", "", "experience.description_2"),
        ("", "", "", "experience.description_3"),
        ("", "", "", "experience.description_4"),
    ];

    let experience_items = experience_data
        .iter()
        .enumerate()
        .map(|(_index, (company, position, period, description_key))| {
            rsx! {
                li { class: "flex flex-col",
                    if !company.is_empty() {
                        div { class: "flex flex-col",
                            div { class: "flex flex-col",
                                div { class: "flex items-center gap-2 text-sm font-medium tracking-tight",
                                    h3 { class: "m-0", "{company} - " }
                                    p { class: "m-0", " {position}" }
                                }
                                p { class: "text-xs text-gray-400 font-normal mt-1", "{period}" }
                            }
                        }
                    }
                    p { class: "text-[10px] text-gray-400 leading-[14px] tracking-tight mt-1", "{i18n().t(description_key)}" }
                }
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        section { class: "px-4 py-2 flex flex-col w-1/2",
            h2 { class: "text-white text-base font-medium mb-4 tracking-tight", "{i18n().t(\"experience.title\")}" }
            ul { class: "flex flex-col text-white font-inter",
                {experience_items.into_iter()}
            }
        }
    }
}
