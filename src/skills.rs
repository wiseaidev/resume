use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
pub fn Skills() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    let skills_data = vec![
        (
            "skills.categories.design",
            vec!["skills.items.project_architecture"],
        ),
        (
            "skills.categories.tools",
            vec![
                "skills.items.ci_cd",
                "skills.items.frontend",
                "skills.items.backend",
                "skills.items.other_tools",
            ],
        ),
        (
            "skills.categories.languages",
            vec!["skills.items.languages"],
        ),
    ];

    rsx! {
        section { class: "px-4 py-2 flex flex-col items-end w-full border-t border-white/10 mt-5",
            div { class: "gap-x-[43px] gap-y-10 mt-7 w-full",
                h2 { class: "text-white text-base font-medium tracking-tight mb-4", "{i18n().t(\"skills.title\")}" }

                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-10 w-full",
                    {skills_data.iter().map(|(category_key, skill_keys)| {
                        rsx! {
                            div { class: "flex flex-col",
                                h3 { class: "text-white text-sm font-medium tracking-tight mb-2", "{i18n().t(category_key)}" }

                                ul { class: "text-[10px] text-gray-400 font-normal leading-[14px] tracking-tight list-none p-0 m-0",
                                    {skill_keys.iter().map(|skill_key| {
                                        rsx! {
                                            li { class: "mb-1", "{i18n().t(skill_key)}" }
                                        }
                                    }).into_iter()}
                                }
                            }
                        }
                    }).into_iter()}
                }
            }
        }
    }
}
