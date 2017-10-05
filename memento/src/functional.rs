///
/// Cannot create object while
/// you do not have all fields, self validating
#[derive(Debug, Clone)]
struct Relation {
    src: String,
    dst: String
}

#[derive(Debug)]
struct CurrentRelation {
    history_index: usize,
    relations: Vec<Relation>
}


fn new_relation(src: String, dst: String, state: CurrentRelation) -> CurrentRelation {
    let r = Relation { src, dst };
    //    let idx = state.relations.len();
    let idx = state.history_index;

    let mut vec2 = state.relations.clone();
    vec2.push(r);
    CurrentRelation { history_index: idx, relations: vec2 }
}

fn undo(state: CurrentRelation) -> CurrentRelation
{
    let idx = (state.history_index as i32 - 1).abs() as usize;
    CurrentRelation { history_index: idx, relations: state.relations }
}


fn redo(state: CurrentRelation) -> CurrentRelation
{
    let idx = (state.history_index as i32 + 1).abs() as usize;
    CurrentRelation { history_index: idx, relations: state.relations }
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let init_state = CurrentRelation { history_index: 0, relations: vec![] };
    let state =
        redo(new_relation("Milanówek".to_string(), "Błonie".to_string(),
                          undo(new_relation("Grodzisk".to_string(),
                                            "Płońsk".to_string(),
                                            new_relation("Gdańsk".to_string(),
                                                         "Radom".to_string(),
                                                         new_relation(
                                                             "Warszawa".to_string(),
                                                             "Rzeszów".to_string(),
                                                             init_state))))));

    println!("{:?}", state);
}