use surreal_use::core::{
    sql::{CreateData, UpdateData},
    Stmt,
};
use surrealdb::sql::Output;

use crate::lib::{
    db::DB,
    entry::{dto::Team, vo},
};

use super::Record;

pub async fn create_team_by_name(name: &str, owner: &str) -> Option<(String, Team)> {
    let team = Team::new_rand(name, owner);
    let sql = Stmt::create()
        .table("team".into())
        .data(CreateData::content(team))
        .output(Output::After)
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<Record<Team>> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res.to_record())
    } else {
        None
    }
}

pub async fn select_team_record_by_id(id: &str) -> Option<(String, Team)> {
    let sql = Stmt::select()
        .table(("team", id).into())
        .field_all()
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<Record<Team>> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res.to_record())
    } else {
        None
    }
}

pub async fn update_team_by_id(id: &str, team: Team) -> bool {
    let sql = Stmt::update()
        .table(("team", id).into())
        .data(UpdateData::content(team))
        .output(Output::After)
        .to_string();

    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<Record<Team>> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        true
    } else {
        false
    }
}
