use actix_web::{get, Result as AwResult};
use maud::{html, Markup};

#[get("/")]
pub async fn index() -> AwResult<Markup> {
    Ok(html! {
        html {
            head {
                title { "Rook Rustpad" }
            }
            body {
                h1 { "Rook Rustpad" }
                p {
                    "This is a place for me to practice some rust code."
                }
                ul {
                    li {
                        a href="/aoc" {
                            "Advent of Code"
                        }
                    }
                }
            }
        }
    })
}