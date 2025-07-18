use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");


fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Add CSS
        document::Link {
            rel: "stylesheet",
            href: MAIN_CSS
        }

        // Add Google Fonts
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap"
        }

        // Main Container
        main { class: "container",
            CV {}
        }
    }
}

#[component]
fn CV() -> Element {
    rsx! {
        // Header Section
        section { class: "cv",
            header { class: "cv-header",
                h1 { "Sergei Glukhov" }
                div { class: "contact-info",
                    span { "Moscow" }
                    span { class: "separator", "•" }
                    span { "Apr 21, 2003" }
                    span { class: "separator", "•" }
                    a { href: "mailto:sglukhov.private@gmail.com", target: "_blank", "sglukhov.private@gmail.com" }
                    span { class: "separator", "•" }
                    a {
                        href: "https://t.me/iloveeconom",
                        target: "_blank",
                        class: "telegram-link",
                        // SVG for Telegram logo with appropriate sizing
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "18",
                            height: "18",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            class: "telegram-icon",
                            path {
                                d: "M11.944 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0a12 12 0 0 0-.056 0zm4.962 7.224c.1-.002.321.023.465.14a.506.506 0 0 1 .171.325c.016.093.036.306.02.472-.18 1.898-.962 6.502-1.36 8.627-.168.9-.499 1.201-.82 1.23-.696.065-1.225-.46-1.9-.902-1.056-.693-1.653-1.124-2.678-1.8-1.185-.78-.417-1.21.258-1.91.177-.184 3.247-2.977 3.307-3.23.007-.032.014-.15-.056-.212s-.174-.041-.249-.024c-.106.024-1.793 1.14-5.061 3.345-.48.33-.913.49-1.302.48-.428-.008-1.252-.241-1.865-.44-.752-.245-1.349-.374-1.297-.789.027-.216.325-.437.893-.663 3.498-1.524 5.83-2.529 6.998-3.014 3.332-1.386 4.025-1.627 4.476-1.635z"
                            }
                        }
                    }
                }
                div { class: "github-container",
                    a {
                        class: "github-link",
                        href: "https://github.com/SergeiGL",
                        target: "_blank",
                        "GitHub"
                    }
                }
            }

            // Experience Section
            section { class: "cv-section",
                h2 { class: "section-title", "Experience" }

                div { class: "experience-item",
                    div { class: "experience-header",
                        h3 { class: "company-name", "SBER CIB + HSE"
                            span { class: "separator", "•" }
                            span { class: "position", "Rust Developer" }
                        }
                        span { class: "duration", "Sep 2024 - Jul 2025" }
                    }
                    ul { class: "achievements",
                        li { "World’s Fastest MCS Algorithm (>500x speedup)"}
                        li { "Ecosystem: Web, Python/C++ connectors" }
                        li { "23,625 lines; 392 tests" }
                        li { "Solved Real-world Financial Challenges" }
                    }
                }

                div { class: "experience-item",
                    div { class: "experience-header",
                        h3 { class: "company-name", "Cryptanium Fund"
                            span { class: "separator", "•" }
                            span { class: "position", "Python Developer" }
                        }
                        span { class: "duration", "Sep 2023 - May 2024" }
                    }
                    ul { class: "achievements",
                        li { "Trading Strategies (Python & C)" }
                        li { "Real-time Data Collection" }
                        li { "Telegram & GUI Apps for Top Executives" }
                    }
                }
                div { class: "minor-role",
                    div { class: "experience-item",
                        div { class: "experience-header",
                            h3 { class: "company-name", "McK Partners" span { class: "position", "Consultant" } }
                        }
                        ul { class: "achievements",
                            li { "Presentations, Analytics, Market Research"}
                        }
                    }

                    div { class: "experience-item",
                        div { class: "experience-header",
                            h3 { class: "company-name", "EVRAZ Group" span {class: "position", "Finance Intern"}
}
                        }
                        ul { class: "achievements",
                            li { "Credit Documents Structuring (Python)" }
                        }
                    }
                }
            }
            // Education Section
            section { class: "cv-section",
                h2 { class: "section-title", "Education" }

                div { class: "education-item",
                    div {class: "education-header",
                        h3 { class: "company-name",
                            "ICEF (BSc, HSE & UoL)"
                        }
                        span { class: "duration", "2021 - 2025" }
                    }
                    ul { class: "achievements",
                        li { "Top 10% Academic Performance" }
                        li { "Advanced Courses: Mathematics++, Statistics++, ML" }
                    }
                }

                div { class: "education-item",
                    div {class: "education-header",
                        h3 { class: "company-name", "Lyceum №1535 (#1 School, RU)" }
                        span { class: "duration", "2016 - 2020" }
                    }
                    ul { class: "achievements",
                        li { "Specialization: Mathematics & Economics" }
                        li { "4.8 Average Graduation Score" }
                    }
                }
            }

            // TechnicalSkillsSection
            section { class: "cv-section",
                h2 { class: "section-title", "Technical Skills" }
                div { class: "skills-container",
                    div { class: "skill-group",
                        h3 { class: "list-header", "Programming" }
                        ul { class: "skills-list",
                            li { span {class: "highlight", "Rust"}" (tokio, axum, nalgebra)" }
                            li { span {class: "highlight", "Python"}" (pytorch, selenium)" }
                            li { "PostgreSQL, MongoDB, Redis" }
                            li { "Docker, Kubernetes" }
                        }
                    }
                    div { class: "skill-group",
                        h3 {class: "list-header", "Analytics" }
                        ul { class: "skills-list",
                            li { "NumPy, Pandas, Plotly, SciPy" }
                            li { "PowerPoint" }
                            li { "Excel, VBA" }
                            li { "LaTeX, Word"}
                        }
                    }
                }
            }

            // Other Skills Section
            section { class: "cv-section",
                h2 { class: "section-title", "Interests & Other Skills" }

                div { class: "skills-container",
                    div { class: "skill-group",
                        h3 {class: "list-header", "Interests" }
                        ul { class: "skills-list",
                            li { span {class: "highlight", "Piano"}" (music school diploma)" }
                            li { span {class: "highlight", "Padel"}"" }
                            li { "Chess (1400 elo)" }
                            li { "Table Tennis" }
                        }
                    }
                    div { class: "skill-group",
                        h3 {class: "list-header", "Personal Skills" }
                        ul { class: "skills-list",
                            li { span {class: "highlight", "Inner Drive"} }
                            li { span {class: "highlight", "Positive Mindset"} }
                            li { "Team Player" }
                            li { "Adaptive Thinker" }
                        }
                    }
                }
            }
            div { class: "updated-at",
                "Last updated: July 2025"
            }
        }
    }
}
