use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}


#[derive(GraphQLObject)]
#[graphql(description = "An android in the Star Wars universe")]
struct Android {
    id: String,
    part_number: String,
    appears_in: Vec<Episode>,
    manufacturer: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "An android in the Star Wars universe")]
struct NewAndroid {
    part_number: String,
    appears_in: Vec<Episode>,
    manufacturer: String,
}


pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn human(_id: String) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
    fn android(_id: String) -> FieldResult<Android> {
        Ok(Android {
            id: "5678".to_owned(),
            part_number: "C3PO".to_owned(),
            appears_in: vec![Episode::NewHope, Episode::Empire, Episode::Jedi],
            manufacturer: "Earth".to_owned()
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
    fn create_android(new_android: NewAndroid) -> FieldResult<Android> {
        Ok(Android {
            id: "5678".to_owned(),
            part_number: new_android.part_number,
            appears_in: new_android.appears_in,
            manufacturer: new_android.manufacturer
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
