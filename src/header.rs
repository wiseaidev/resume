use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
pub fn Header() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    rsx! {header { class: "flex justify-between items-center w-full p-4 border-b border-white/10",
            div {  class: "flex items-center ",
            img {
                class: "px-2 py-2 w-20",
                src: "https://avatars.githubusercontent.com/u/62179149?v=4",
                alt: "Profile Image"
            }
            div { class: "flex flex-col font-medium",
                h1 { class: "text-white text-xl tracking-tight", "{i18n().t(\"header.name\")}" }
                p { class: "text-gray-400 text-xs tracking-tight", "{i18n().t(\"header.job_title\")}" }
            },}
            div { class: "flex items-center text-center gap-4",
            div { class: "flex flex-col gap-1 text-gray-400 text-xs",
            a {
                href: "mailto:business@wiseai.dev",
                target: "_blank",
                class: "hover:underline",
                "{i18n().t(\"header.email\")}"
            }
            a {
                href: "https://wiseai.dev",
                target: "_blank",
                class: "hover:underline",
                "{i18n().t(\"header.website\")}"
            }
            a {
                href: "https://www.linkedin.com/in/mahmoud-harmouch",
                target: "_blank",
                class: "hover:underline",
                "{i18n().t(\"header.linkedin\")}"
            }
            }
            LanguageDropdown {}
        }
            }
    }
}
#[component]
pub fn LanguageDropdown() -> Element {
    let I18nContext { set_language, .. } = use_context::<I18nContext>();
    let mut is_open = use_signal(|| false);
    let selected_lang = use_signal(|| "en".to_string());

    rsx! {
        div { class: "relative dropdown-container",
            button {
                class: "flex items-center gap-2 bg-gray-800 text-white px-2 py-1 text-xs rounded-md hover:bg-gray-700 transition",
                onclick: move |_| is_open.set(!is_open()),
                span { "{selected_lang()}" }
                svg {
                    class: "w-4 h-4 transition-transform",
                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M19 9l-7 7-7-7" }
                }
            }

            if is_open() {
                div { class: "absolute right-0 w-20 bg-gray-800 text-white shadow-md rounded-md overflow-hidden transition-opacity duration-300 opacity-100 scale-100",
                OptionItem {label: "🇺🇸 English", value: "en", selected_lang: selected_lang, set_language: set_language},
                OptionItem {label: "🇫🇷 French", value: "fr", selected_lang: selected_lang, set_language: set_language},
                    OptionItem {label: "🇪🇸 Spanish", value: "es", selected_lang: selected_lang, set_language: set_language},
                    OptionItem {label: "🇸🇦 Arabic", value: "ar", selected_lang:  selected_lang, set_language: set_language}
                }
            }
        }
    }
}

#[component]
fn OptionItem(
    label: &'static str,
    value: &'static str,
    selected_lang: Signal<String>,
    set_language: Callback<String>,
) -> Element {
    rsx! {
        div {
            class: "px-2 py-2 hover:bg-gray-700 cursor-pointer text-xs whitespace-nowrap",
            onclick: move |_| {
                selected_lang.set(value.to_string());
                set_language.call(value.to_string());
            },
            "{label}"
        }
    }
}
