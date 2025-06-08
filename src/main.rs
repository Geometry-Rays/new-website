use sycamore::prelude::*;

#[component]
fn App() -> View {
    let lead_dev: &str = "Puppet";
    let website_dev: &str = "Puppet";

    view! {
        div(id="main-div") {
            h1(style="padding-top: 200px; padding-bottom: 5px;") {
                "Geometry Rays"
            }

            h2(style="padding-top: 10px; font-size: 40px; color: #ff6a00;") {
                "Fyre"
            }

            p {
                (format!("Lead Dev: {}", lead_dev))
            }

            p {
                (format!("Website made by: {}", website_dev))
            }

            h2 {
                "Links"
            }

            p(class="link") {
                a(href="https://github.com/Geometry-Rays/geometry-rays/releases") {
                    "Download"
                }
            }

            p(class="link") {
                a(href="https://github.com/Geometry-Rays") {
                    "Github"
                }
            }

            p(class="link") {
                a(href="https://discord.gg/dpkT256hWm") {
                    "Discord"
                }
            }

            h2 {
                "Team"
            }

            p {
                "Puppet: Lead Dev and Website Creator"
            }

            p(style="padding-bottom: 200px;") {
                "Lncvrt: Secondary Dev"
            }
        }
    }
}

fn main() {
    sycamore::render(App);
}
