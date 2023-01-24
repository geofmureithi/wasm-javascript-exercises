use crate::database::db::establish_connection;
use crate::models::models::TeamWithPoints;
use actix_web::http::StatusCode;
use actix_web::{get, HttpResponse, Responder};
use diesel::prelude::*;
use std::collections::HashMap;
use yew::prelude::*;
use yew::ServerRenderer;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world from Actix.")
}

#[get("/scoreboard")]
pub async fn scoreboard() -> impl Responder {
    let renderer = ServerRenderer::<App>::new();
    let rendered = renderer.render().await;
    //HttpResponse::Ok().body(rendered)
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}

#[function_component]
fn App() -> Html {
    html! {
        <html>
        <head>
            <title>{"Scoreboard"}</title>
            <link href={"https://fonts.googleapis.com/css2?family=Roboto+Slab:wght@300&display=swap"} rel={"stylesheet"} />

        </head>
        <body style={"display: flex; flex-direction: column; justify-content: center; align-items: center; font-family: 'Roboto Slab', serif;"}>
            <h1>{"Scoreboard"}</h1>
            <ScoreBoard />
        </body>
        </html>
    }
}

#[function_component]
fn ScoreBoard() -> Html {
    use crate::schema::exercises::dsl::*;
    use crate::schema::scoreboard::dsl::*;
    // get scores from db
    let connection = &mut establish_connection();
    let score_board_entries = scoreboard
        .inner_join(exercises)
        .select((team, points))
        .load::<TeamWithPoints>(connection)
        .expect("Error loading points");
    let scores = get_scores(score_board_entries);

    html! {
        <>
        <div style={"display: flex; width: 24rem; font-size: 1.5rem; line-height: 2rem;"}>
            <div style={"width: 12rem; display: flex; justify-content: center;"}>
                {"Team Name"}
            </div>
            <div style={"width: 12rem; display: flex; justify-content: center;"}>
                {"Score"}
            </div>
        </div>
        <ul style={"border-top: 1px solid black; border-bottom: 1px solid black; list-style-type: none; font-size: 1.125rem; line-height: 1.75rem; font-weight: 700; width: 24rem; padding: 0px; margin: 0px;"}>
            {
                scores.into_iter().map(|ele| {
                    html!{

                        <li style={"border-left: 1px solid black; border-right: 1px solid black; list-style-type: none; display: flex; justify-content: center; width: 24rem; padding: 0px;"}>
                                <div style={"border-right: 1px solid black; width: 12rem; display: flex; justify-content: center;"}>
                                    {ele.0}
                                </div>
                                <div style={"width: 12rem; display: flex; justify-content: center;"}>
                                    {ele.1}
                                </div>
                        </li>
                    }
                }).collect::<Html>()
            }
        </ul>
        </>
    }
}

fn get_scores(team_points: Vec<TeamWithPoints>) -> Vec<(String, i32)> {
    let mut team_scores = HashMap::new();
    for element in team_points {
        team_scores
            .entry(element.team)
            .and_modify(|score| *score += element.points)
            .or_insert(element.points);
    }
    let mut sorted_team_scores: Vec<(String, i32)> = team_scores.into_iter().collect();
    sorted_team_scores.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_team_scores
}
